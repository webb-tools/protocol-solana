use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;

mod hashing;
mod merkle_tree;

declare_id!("Dw96F8NjN84googpni4mtSnCuAud9XkaPUFM1RJX53cK");

#[program]
pub mod anchor_escrow {
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";

    pub fn initialize(
        ctx: Context<Initialize>,
        _vault_account_bump: u8,
        deposit_amount: u64,
        levels: u8,
    ) -> ProgramResult {
        ctx.accounts.anchor_metadata.initializer_key = *ctx.accounts.initializer.key;
        ctx.accounts.anchor_metadata.deposit_token_account =
            *ctx.accounts.deposit_token_account.to_account_info().key;
        ctx.accounts.anchor_metadata.deposit_amount = deposit_amount;

        let mut merkle_tree_account = ctx.accounts.merkle_tree_account.load_init()?;
        merkle_tree_account.initializer_key = *ctx.accounts.initializer.key;

        merkle_tree_account.current_root_index = 0u8;
        merkle_tree_account.next_index = 0u32;
        merkle_tree_account.levels = levels;
        merkle_tree_account.roots = [[0u8; 32]; 32];
        merkle_tree_account.filled_subtrees = [[0u8; 32]; 32];

        let (vault_authority, _vault_authority_bump) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);

        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;

        Ok(())
    }

    pub fn deposit(ctx: Context<DepositInto>, commitment: [u8; 32]) -> ProgramResult {
        let mut merkle_tree_account = ctx.accounts.merkle_tree_account.load_mut()?;
        let _inserted_index = merkle_tree_account.insert(commitment);
        if let Ok(index) = _inserted_index {
            msg!("inserted_index: {}", index);
            ctx.accounts.anchor_metadata.deposit_count += 1;
            token::transfer(
                ctx.accounts.into_transfer_to_pda_context(),
                ctx.accounts.anchor_metadata.deposit_amount,
            )?;
        }

        Ok(())
    }

    // pub fn setup_params(ctx: Context<HashInitialize>, params: Vec<F>)
}

#[derive(Accounts)]
#[instruction(vault_account_bump: u8, deposit_amount: u64)]
pub struct Initialize<'info> {
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = [b"token-seed".as_ref()],
        bump = vault_account_bump,
        payer = initializer,
        token::mint = mint,
        token::authority = initializer,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = deposit_token_account.amount >= deposit_amount
    )]
    pub deposit_token_account: Account<'info, TokenAccount>,
    #[account(init, payer = initializer)]
    pub anchor_metadata: Account<'info, AnchorMetadata>,
    #[account(zero)]
    pub merkle_tree_account: Loader<'info, MerkleTreeAccount>,
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: AccountInfo<'info>,
}

#[account(zero_copy)]
pub struct MerkleTreeAccount {
    pub initializer_key: Pubkey,
    pub current_root_index: u8,
    pub next_index: u32,
    pub levels: u8,
    pub roots: [[u8; 32]; 32],
    pub filled_subtrees: [[u8; 32]; 32],
    pub params: [u8; 6536],
}

#[account]
#[derive(Default)]
pub struct AnchorMetadata {
    pub initializer_key: Pubkey,
    pub deposit_token_account: Pubkey,
    pub deposit_amount: u64,
    pub deposit_count: u64,
    pub withdrawal_count: u64,
}

#[derive(Accounts)]
pub struct DepositInto<'info> {
    #[account(mut, signer)]
    pub depositor: AccountInfo<'info>,
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = deposit_token_account.amount >= anchor_metadata.deposit_amount
    )]
    pub deposit_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    merkle_tree_account: Loader<'info, MerkleTreeAccount>,
    #[account(mut)]
    anchor_metadata: Account<'info, AnchorMetadata>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

impl<'info> Initialize<'info> {
    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.vault_account.to_account_info().clone(),
            current_authority: self.initializer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

impl<'info> DepositInto<'info> {
    fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.deposit_token_account.to_account_info().clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.depositor.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}
