//! Processing and handling of instructions from a [`StarFrameProgram::entrypoint`].
//!
//! This implementation supports hybrid Pod + Borsh deserialization where:
//! - Fixed-size instruction data is Pod (zero-copy, fast)
//! - Variable-size data is Borsh-encoded trailing bytes (flexible)

use crate::{
    account_set::{AccountSetCleanup, AccountSetDecode, AccountSetValidate},
    prelude::*,
};
use bytemuck::{bytes_of, Pod};
use pinocchio::cpi::set_return_data;
use std::fmt::Debug;

pub use star_frame_proc::{
    star_frame_instruction, InstructionArgs, InstructionSet, InstructionToIdl,
};

mod no_op;
mod un_callable;
pub use un_callable::UnCallable;

/// A set of instructions that can be used as input to a program.
///
/// This can be derived using the [`derive@InstructionSet`] macro on an enum.
pub trait InstructionSet {
    /// The discriminant type used by this program's instructions.
    type Discriminant: Pod;

    /// Dispatches the instruction data from the program entrypoint and then
    /// calls the appropriate [`Instruction::process_from_raw`] method.
    ///
    /// This is called directly by [`StarFrameProgram::entrypoint`].
    fn dispatch(
        program_id: &'static Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> Result<()>;
}

/// A helper trait for the value of the instruction discriminant on an instruction.
///
/// Since a single instruction can be in multiple [`InstructionSet`]s, this trait is generic over it
/// (with `IxSet`).
pub trait InstructionDiscriminant<IxSet>
where
    IxSet: InstructionSet,
{
    /// The actual value of the discriminant. For a single [`InstructionSet`], each member should
    /// have a unique discriminant.
    const DISCRIMINANT: <IxSet as InstructionSet>::Discriminant;

    #[must_use]
    fn discriminant_bytes() -> Vec<u8> {
        bytes_of(&Self::DISCRIMINANT).into()
    }
}

/// A callable instruction that can be used as input to a program.
pub trait Instruction {
    /// Runs the instruction from a raw solana input.
    ///
    /// This is called from [`InstructionSet::dispatch`] after the discriminant is parsed and matched on.
    fn process_from_raw(
        program_id: &'static Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> Result<()>;
}

/// Helper type for the return of [`InstructionArgs::split_to_args`].
pub struct IxArgs<'a, T: InstructionArgs> {
    pub decode: <T as InstructionArgs>::DecodeArg<'a>,
    pub validate: <T as InstructionArgs>::ValidateArg<'a>,
    pub run: <T as InstructionArgs>::RunArg<'a>,
    pub cleanup: <T as InstructionArgs>::CleanupArg<'a>,
}

// Implement Default if all argument types also implement Default
impl<'a, T> Default for IxArgs<'a, T>
where
    T: InstructionArgs,
    T::DecodeArg<'a>: Default,
    T::ValidateArg<'a>: Default,
    T::RunArg<'a>: Default,
    T::CleanupArg<'a>: Default,
{
    fn default() -> Self {
        IxArgs {
            decode: Default::default(),
            validate: Default::default(),
            run: Default::default(),
            cleanup: Default::default(),
        }
    }
}

// Implement Debug if all argument types also implement Debug
impl<'a, T> std::fmt::Debug for IxArgs<'a, T>
where
    T: InstructionArgs,
    T::DecodeArg<'a>: Debug,
    T::ValidateArg<'a>: Debug,
    T::RunArg<'a>: Debug,
    T::CleanupArg<'a>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IxArgs")
            .field("decode", &self.decode)
            .field("validate", &self.validate)
            .field("run", &self.run)
            .field("cleanup", &self.cleanup)
            .finish()
    }
}

/// A helper trait to split a struct into arguments for a [`StarFrameInstruction`].
///
/// Derivable via [`derive@InstructionArgs`].
pub trait InstructionArgs: Sized {
    /// The instruction data type used to decode accounts.
    type DecodeArg<'a>;
    /// The instruction data type used to validate accounts.
    type ValidateArg<'a>;
    /// The instruction data type used to run the instruction.
    type RunArg<'a>;
    /// The instruction data type used to cleanup accounts.
    type CleanupArg<'a>;
    /// Splits self into decode, validate, cleanup, and run args.
    fn split_to_args(r: &mut Self) -> IxArgs<'_, Self>;
}

#[doc(hidden)]
#[diagnostic::on_unimplemented(
    message = "`StarFrameInstruction` requires the return type to be `Result<T>`"
)]
/// A helper trait to get the inner T of a [`Result`] from a [`StarFrameInstruction::process`] declaration. This is used internally in the [`star_frame_instruction`] macro.
pub trait IxReturnType {
    type ReturnType;
}
impl<T, E> IxReturnType for Result<T, E> {
    type ReturnType = T;
}

/// An opinionated (and recommended) [`Instruction`] using [`AccountSet`] and other traits. Can be derived using the [`star_frame_instruction`] macro.
///
/// The steps for how this implements [`Instruction::process_from_raw`] are as follows:
/// 1. Decode the fixed-size Pod instruction data using [`bytemuck::from_bytes`].
/// 2. Decode the trailing Borsh data (if present) using [`BorshDeserialize`].
/// 3. Split Self into decode, validate, run, and cleanup args using [`InstructionArgs::split_to_args`].
/// 4. Decode the accounts using [`Self::Accounts::decode_accounts`](AccountSetDecode::decode_accounts).
/// 5. Validate the accounts using [`Self::Accounts::validate_accounts`](AccountSetValidate::validate_accounts).
/// 6. Process the instruction using [`Self::process`].
/// 7. Cleanup the accounts using [`Self::Accounts::cleanup_accounts`](AccountSetCleanup::cleanup_accounts).
/// 8. Set the solana return data using [`bytemuck::bytes_of`] if it is not empty.
pub trait StarFrameInstruction: Pod + InstructionArgs {
    /// The return type of this instruction.
    type ReturnType: NoUninit;

    /// The trailing data type for variable-length data (Borsh-encoded).
    ///
    /// Set to `()` if no trailing data is needed.
    ///
    /// If trailing data is present, it will be deserialized from the bytes
    /// after the fixed-size Pod instruction data.

    /// The [`AccountSet`] used by this instruction.
    type Accounts<'decode, 'arg>: AccountSetDecode<'decode, Self::DecodeArg<'arg>>
        + AccountSetValidate<Self::ValidateArg<'arg>>
        + AccountSetCleanup<Self::CleanupArg<'arg>>;

    /// Processes the instruction.
    ///
    /// # Arguments
    /// * `accounts` - The decoded and validated account set
    /// * `run_arg` - The run-time arguments from the Pod instruction data
    /// * `ctx` - The execution context
    fn process(
        accounts: &mut Self::Accounts<'_, '_>,
        run_arg: Self::RunArg<'_>,
        ctx: &mut Context,
    ) -> Result<Self::ReturnType>;
}

impl<T> Instruction for T
where
    T: StarFrameInstruction,
{
    #[inline]
    fn process_from_raw(
        program_id: &'static Pubkey,
        mut accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> Result<()> {
        let mut ctx = Context::new(program_id);

        // Step 1: Parse the fixed-size Pod instruction data (zero-copy)
        let pod_size = size_of::<T>();
        ensure!(
            instruction_data.len() >= pod_size,
            "Instruction data too small: expected at least {} bytes, got {}",
            pod_size,
            instruction_data.len()
        );

        // SAFETY: T is Pod, so it is safe to cast from bytes
        let mut data: T = *bytemuck::from_bytes(&instruction_data[..pod_size]);

        // Step 2: Parse trailing Borsh data (if present)
        let trailing = if instruction_data.len() > pod_size {
            <T as StarFrameInstruction>::TrailingData::deserialize(
                &mut &instruction_data[pod_size..],
            )
            .ctx("Failed to deserialize trailing data")?
        } else {
            // No trailing data provided, use default
            <T as StarFrameInstruction>::TrailingData::default()
        };

        // Step 3: Split instruction data into args
        let IxArgs {
            decode,
            validate,
            run,
            cleanup,
        } = T::split_to_args(&mut data);

        // Step 4: Decode accounts
        let mut account_set: <T as StarFrameInstruction>::Accounts<'_, '_> =
            <T as StarFrameInstruction>::Accounts::decode_accounts(&mut accounts, decode, &mut ctx)
                .ctx("Failed to decode accounts")?;

        // Step 5: Validate accounts
        account_set
            .validate_accounts(validate, &mut ctx)
            .ctx("Failed to validate accounts")?;

        // Step 6: Process the instruction with trailing data
        let ret: <T as StarFrameInstruction>::ReturnType =
            T::process(&mut account_set, run, trailing, &mut ctx)
                .ctx("Failed to run instruction")?;

        // Step 7: Cleanup accounts
        account_set
            .cleanup_accounts(cleanup, &mut ctx)
            .ctx("Failed to cleanup accounts")?;

        // Step 8: Set return data if non-empty
        if size_of::<T::ReturnType>() > 0 {
            set_return_data(bytemuck::bytes_of(&ret));
        }

        Ok(())
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! empty_star_frame_instruction {
    ($ix:ident, $accounts:ident) => {
        impl $crate::instruction::StarFrameInstruction for $ix {
            type ReturnType = ();
            type TrailingData = ();
            type Accounts<'decode, 'arg> = $accounts;

            fn process(
                _accounts: &mut Self::Accounts<'_, '_>,
                _run_arg: Self::RunArg<'_>,
                _trailing: Self::TrailingData,
                _ctx: &mut $crate::context::Context,
            ) -> $crate::Result<Self::ReturnType> {
                Ok(())
            }
        }
    };
}

/// A helper macro for implementing blank instructions for testing.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_blank_ix {
    ($($ix:ident),*) => {
        $(
            impl $crate::instruction::Instruction for $ix {
                fn process_from_raw(
                    _program_id: &'static $crate::prelude::Pubkey,
                    _accounts: &[$crate::prelude::AccountInfo],
                    _data: &[u8],
                ) -> $crate::Result<()> {
                    todo!()
                }
            }
        )*
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::impl_blank_ix;
    use star_frame_proc::InstructionSet;

    #[allow(dead_code)]
    struct Ix1 {
        val: u8,
    }
    #[allow(dead_code)]
    struct Ix2 {
        val: u64,
    }

    impl_blank_ix!(Ix1, Ix2);

    #[allow(dead_code)]
    #[derive(InstructionSet)]
    #[ix_set(skip_idl)]
    enum TestInstructionSet {
        Ix1(Ix1),
        Ix2(Ix2),
    }
}
