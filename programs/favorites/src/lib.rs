use anchor_lang::prelude::*;

declare_id!("4dCN8wo7eAx9gknei3khKYEnscxvrjD6f4BYThwn1vT8");

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
