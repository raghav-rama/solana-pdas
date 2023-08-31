use std::mem::size_of;

use anchor_lang::prelude::*;

declare_id!("DaR44rt9hcx2QPdXnPpv2DWnpJS7uaS6i5UbkmumJVCK");

#[program]
pub mod demo_pda {
    use super::*;

    pub fn initialize(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.from = ctx.accounts.from.key();
        escrow.to = ctx.accounts.to.key();
        escrow.amount = amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(
        init,
        seeds = [b"escrow".as_ref(), from.key().as_ref(), to.key().as_ref()],
        bump,
        payer = from,
        space = size_of::<EscrowAccount>() + 500
    )]
    pub escrow: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EscrowAccount {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}
