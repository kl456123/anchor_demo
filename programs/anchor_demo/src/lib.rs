use anchor_lang::prelude::*;
use anchor_lang::{solana_program::instruction::Instruction, InstructionData};

declare_id!("3HHRxzJ7Ewqo8QKHnGZYbaztXWG9Mpsij1xtn86qKfWY");

#[program]
pub mod anchor_demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, num: u64) -> Result<()> {
        ctx.accounts.new_account.num = num;
        msg!("Changed data to {}!", num);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=signer, space=8+8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NewAccount {
    pub num: u64,
}
