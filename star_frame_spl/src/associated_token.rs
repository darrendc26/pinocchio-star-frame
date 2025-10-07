use crate::token::{
    state::{MintAccount, TokenAccount},
    Token,
};

use star_frame::{derive_more, empty_star_frame_instruction, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct AssociatedToken;

impl AssociatedToken {
    /// Find the associated token address for the given wallet and mint.
    ///
    /// See [`spl_associated_token_account_interface::address::get_associated_token_address`].
    /// ```
    /// # use star_frame_spl::{token::state::MintAccount,associated_token::AssociatedToken};
    /// # use spl_associated_token_account_interface::address::get_associated_token_address;
    /// # use pretty_assertions::assert_eq;
    /// # use star_frame::prelude::{KeyFor, Pubkey};
    /// let wallet = Pubkey::new_unique();
    /// let mint = KeyFor::<MintAccount>::new(Pubkey::new_unique());
    /// assert_eq!(
    ///     AssociatedToken::find_address(&wallet, &mint),
    ///     get_associated_token_address(&wallet, &mint.pubkey()),
    /// );
    /// ```
    pub fn find_address(wallet: &Pubkey, mint: &KeyFor<MintAccount>) -> Pubkey {
        Self::find_address_with_bump(wallet, mint).0
    }

    /// Find the associated token address for the given wallet and mint, with a bump.
    pub fn find_address_with_bump(wallet: &Pubkey, mint: &KeyFor<MintAccount>) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[wallet.as_ref(), Token::ID.as_ref(), mint.pubkey().as_ref()],
            &Self::ID,
        )
    }
}

impl StarFrameProgram for AssociatedToken {
    type InstructionSet = instructions::AssociatedTokenInstructionSet;
    type AccountDiscriminant = ();
    /// See [`spl_associated_token_account_interface::program::ID`].
    /// ```
    /// # use star_frame::program::StarFrameProgram;
    /// # use star_frame_spl::associated_token::AssociatedToken;
    /// assert_eq!(AssociatedToken::ID, spl_associated_token_account_interface::program::ID);
    /// ```
    const ID: Pubkey = pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
}

#[cfg(all(feature = "idl", not(target_os = "solana")))]
mod idl_impl {
    use super::*;
    use star_frame::{
        idl::{FindIdlSeeds, FindSeed, SeedsToIdl},
        star_frame_idl::seeds::{IdlFindSeed, IdlSeed, IdlSeeds},
    };

    use crate::token::{state::MintAccount, Token};
    use star_frame::star_frame_idl::IdlDefinition;

    #[repr(C)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AssociatedTokenSeeds {
        pub wallet: Pubkey,
        pub mint: KeyFor<MintAccount>,
    }

    pub type AtaSeeds = AssociatedTokenSeeds;
    pub type FindAtaSeeds = FindAssociatedTokenSeeds;

    impl GetSeeds for AssociatedTokenSeeds {
        fn seeds(&self) -> [&[u8]; 3] {
            let seeds: [&[u8]; 3] = [
                self.wallet.as_ref(),
                Token::ID.as_ref(),
                self.mint.pubkey().as_ref(),
            ];
            seeds
        }
    }

    impl SeedsToIdl for AssociatedTokenSeeds {
        fn seeds_to_idl(idl_definition: &mut IdlDefinition) -> star_frame::IdlResult<IdlSeeds> {
            Ok(IdlSeeds(vec![
                IdlSeed::Variable {
                    name: "wallet".to_string(),
                    description: vec![],
                    ty: <Pubkey as TypeToIdl>::type_to_idl(idl_definition)?,
                },
                IdlSeed::Const(Token::ID.as_ref().to_vec()),
                IdlSeed::Variable {
                    name: "mint".to_string(),
                    description: vec![],
                    ty: <Pubkey as TypeToIdl>::type_to_idl(idl_definition)?,
                },
            ]))
        }
    }

    impl ProgramToIdl for AssociatedToken {
        type Errors = ();
        fn crate_metadata() -> star_frame::star_frame_idl::CrateMetadata {
            star_frame::star_frame_idl::CrateMetadata {
                version: star_frame::star_frame_idl::Version::new(3, 0, 4),
                name: "associated_token".to_string(),
                docs: vec![],
                description: None,
                homepage: None,
                license: None,
                repository: None,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct FindAssociatedTokenSeeds {
        pub wallet: FindSeed<Pubkey>,
        pub mint: FindSeed<Pubkey>,
    }
    impl FindIdlSeeds for FindAssociatedTokenSeeds {
        fn find_seeds(&self) -> star_frame::IdlResult<Vec<IdlFindSeed>> {
            Ok(vec![
                Into::into(&self.wallet),
                IdlFindSeed::Const(Token::ID.as_ref().to_vec()),
                Into::into(&self.mint),
            ])
        }
    }
}

#[cfg(all(feature = "idl", not(target_os = "solana")))]
pub use idl_impl::*;
use star_frame::derive_more::{Deref, DerefMut};

pub mod instructions {
    pub use super::*;

    #[derive(Copy, Debug, Clone, PartialEq, Eq, InstructionSet)]
    #[ix_set(use_repr)]
    #[repr(u8)]
    pub enum AssociatedTokenInstructionSet {
        Create(Create),
        CreateIdempotent(CreateIdempotent),
        RecoverNested(RecoverNested),
    }

    // create
    /// See [`spl_associated_token_account_interface::instruction::AssociatedTokenAccountInstruction::Create`].
    #[derive(Copy, Clone, Debug, Eq, PartialEq, InstructionArgs)]
    #[type_to_idl(program = AssociatedToken)]
    pub struct Create;
    /// Accounts for the [`Create`] instruction.
    #[derive(Debug, Clone, AccountSet)]
    pub struct CreateAccounts {
        pub funder: Mut<Signer>,
        #[idl(arg = Seeds(FindAtaSeeds {
            wallet: seed_path("wallet"),
            mint: seed_path("mint"),
        }))]
        pub token_account: Mut<AccountInfo>,
        pub wallet: AccountInfo,
        pub mint: AccountInfo,
        pub system_program: Program<System>,
        pub token_program: Program<Token>,
    }

    empty_star_frame_instruction!(Create, CreateAccounts);

    // create idempotent
    /// See [`spl_associated_token_account_interface::instruction::AssociatedTokenAccountInstruction::CreateIdempotent`].
    ///
    /// This instruction has an identical AccountSet to [`Create`].
    #[derive(Copy, Clone, Debug, Eq, PartialEq, InstructionArgs)]
    #[type_to_idl(program = AssociatedToken)]
    pub struct CreateIdempotent;
    empty_star_frame_instruction!(CreateIdempotent, CreateAccounts);

    // recover nested
    /// See [`spl_associated_token_account_interface::instruction::AssociatedTokenAccountInstruction::RecoverNested`].
    #[derive(Copy, Clone, Debug, Eq, PartialEq, InstructionArgs)]
    #[type_to_idl(program = AssociatedToken)]
    pub struct RecoverNested;
    /// Accounts for the [`RecoverNested`] instruction.
    #[derive(Debug, Clone, AccountSet)]
    pub struct RecoverNestedAccounts {
        #[idl(arg =
            Seeds(FindAtaSeeds {
                wallet: seed_path("owner_ata"),
                mint: seed_path("nested_mint"),
            })
        )]
        pub nested_ata: Mut<AccountInfo>,
        pub nested_mint: AccountInfo,
        #[idl(arg =
            Seeds(FindAtaSeeds {
                wallet: seed_path("wallet"),
                mint: seed_path("nested_mint"),
            })
        )]
        pub destination_ata: Mut<AccountInfo>,
        #[idl(arg =
            Seeds(FindAtaSeeds {
                wallet: seed_path("wallet"),
                mint: seed_path("owner_mint"),
            })
        )]
        pub owner_ata: Mut<AccountInfo>,
        pub owner_mint: AccountInfo,
        pub wallet: Mut<Signer>,
        pub token_program: Program<Token>,
    }
    empty_star_frame_instruction!(RecoverNested, RecoverNestedAccounts);
}

pub mod state {
    use star_frame::{
        account_set::{
            modifiers::{CanInitAccount, CanInitSeeds},
            AccountSetValidate, CanFundRent,
        },
        data_types::{GetKeyFor, GetOptionalKeyFor},
        errors::ErrorCode,
    };

    use super::*;

    #[derive(AccountSet, Debug, Deref, DerefMut)]
    #[validate(
        id = "validate_ata",
        arg = ValidateAta<'a>,
        generics = [<'a>],
        extra_validation = self.validate_ata(arg)
    )]
    pub struct AssociatedTokenAccount(
        #[single_account_set(skip_can_init_account, skip_can_init_seeds)] pub(crate) TokenAccount,
    );

    impl GetKeyFor<AssociatedTokenAccount> for AssociatedTokenAccount {
        fn key_for(&self) -> &KeyFor<AssociatedTokenAccount> {
            KeyFor::new_ref(self.pubkey())
        }
    }

    impl GetOptionalKeyFor<AssociatedTokenAccount> for AssociatedTokenAccount {
        fn optional_key_for(&self) -> &OptionalKeyFor<AssociatedTokenAccount> {
            self.key_for().into()
        }
    }

    impl AssociatedTokenAccount {
        /// Validates that the given account is an associated token account.
        pub fn validate_ata(&self, validate_ata: ValidateAta) -> Result<()> {
            let expected_address =
                AssociatedToken::find_address(validate_ata.wallet, validate_ata.mint);
            if self.pubkey() != &expected_address {
                return Err(ErrorCode::AddressMismatch.into());
            }
            Ok(())
        }
    }

    impl<A> CanInitSeeds<A> for AssociatedTokenAccount
    where
        Self: AccountSetValidate<A>,
    {
        fn init_seeds(&mut self, _arg: &A, _ctx: &Context) -> Result<()> {
            Ok(())
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub struct ValidateAta<'a> {
        pub wallet: &'a Pubkey,
        pub mint: &'a KeyFor<MintAccount>,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct InitAta<'a, WalletInfo, MintInfo>
    where
        WalletInfo: SingleAccountSet,
        MintInfo: SingleAccountSet,
    {
        pub wallet: &'a WalletInfo,
        pub mint: &'a MintInfo,
        pub system_program: Program<System>,
        pub token_program: Program<Token>,
    }

    impl<'a, WalletInfo, MintInfo> InitAta<'a, WalletInfo, MintInfo>
    where
        WalletInfo: SingleAccountSet,
        MintInfo: SingleAccountSet,
    {
        pub fn new(
            wallet: &'a WalletInfo,
            mint: &'a MintInfo,
            system_program: Program<System>,
            token_program: Program<Token>,
        ) -> Self {
            Self {
                wallet,
                mint,
                system_program,
                token_program,
            }
        }
    }

    impl<'a, WalletInfo, MintInfo> From<InitAta<'a, WalletInfo, MintInfo>> for ValidateAta<'a>
    where
        WalletInfo: SingleAccountSet,
        MintInfo: SingleAccountSet,
    {
        fn from(value: InitAta<'a, WalletInfo, MintInfo>) -> Self {
            Self {
                mint: KeyFor::new_ref(value.mint.pubkey()),
                wallet: value.wallet.pubkey(),
            }
        }
    }

    impl<'a, WalletInfo, MintInfo> CanInitAccount<InitAta<'a, WalletInfo, MintInfo>>
        for AssociatedTokenAccount
    where
        WalletInfo: SingleAccountSet,
        MintInfo: SingleAccountSet,
    {
        fn init_account<const IF_NEEDED: bool>(
            &mut self,
            arg: InitAta<'a, WalletInfo, MintInfo>,
            account_seeds: Option<Vec<&[u8]>>,
            ctx: &Context,
        ) -> Result<()> {
            let funder = ctx
                .get_funder()
                .ok_or_else(|| ErrorCode::EmpthFunderCache.into())?;
            self.init_account::<IF_NEEDED>((arg, funder), account_seeds, ctx)
        }
    }

    impl<'a, WalletInfo, MintInfo, Funder>
        CanInitAccount<(InitAta<'a, WalletInfo, MintInfo>, &Funder)> for AssociatedTokenAccount
    where
        WalletInfo: SingleAccountSet,
        MintInfo: SingleAccountSet,
        Funder: CanFundRent + ?Sized,
    {
        fn init_account<const IF_NEEDED: bool>(
            &mut self,
            (init_ata, funder): (InitAta<'a, WalletInfo, MintInfo>, &Funder),
            account_seeds: Option<&[&[u8]]>,
            ctx: &Context,
        ) -> Result<()> {
            if IF_NEEDED && self.owner_pubkey() == Token::ID {
                self.validate()?;
                self.validate_ata(init_ata.into())?;
                return Ok(());
            }
            if !funder.can_create_account() {
                let current_lamports = self.account_info().lamports();
                let rent = ctx.get_rent()?;
                let required_rent = rent
                    .minimum_balance(TokenAccount::LEN)
                    .saturating_sub(current_lamports);
                if required_rent > 0 {
                    funder.fund_rent(self, required_rent, ctx)?;
                }
            }
            if matches!(account_seeds, Some(_)) {
                Err(ErrorCode::InvalidSeeds.into())
            }
            self.check_writable()?;
            let funder_seeds = funder.signer_seeds();
            let seeds: &[&[&[u8]]] = match &funder_seeds {
                Some(seeds) => &[seeds],
                None => &[],
            };

            let wallet_ai = init_ata.wallet.account_info();
            let mint_ai = init_ata.mint.account_info();
            let token_ai = self.account_info();
            let sys_ai = init_ata.system_program.account_info();
            let tok_ai = init_ata.token_program.account_info();
            let funder_ai = funder.account_to_modify();

            AssociatedToken::cpi(
                instructions::Create,
                instructions::CreateCpiAccounts {
                    funder: funder_ai,
                    token_account: *token_ai,
                    wallet: *wallet_ai,
                    mint: *mint_ai,
                    system_program: *sys_ai,
                    token_program: *tok_ai,
                },
                None,
            )
            .invoke_signed(seeds)?;

            Ok(())
        }
    }
}
