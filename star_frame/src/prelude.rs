// TODO: create a prelude module for star_frame

pub use crate::{context::Context, ensure, Result};

pub use pinocchio::{
    account_info::AccountInfo, instruction::AccountMeta as PinocchioAccountMeta, msg,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

// bytemuck
pub use bytemuck::{CheckedBitPattern, NoUninit, Pod, Zeroable};

pub use borsh::{BorshDeserialize, BorshSerialize};

// ensure derive macros are in scope
pub use star_frame_proc::{zero_copy, InstructionToIdl, TypeToIdl};
