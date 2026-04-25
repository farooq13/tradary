use anchor_lang::prelude::*;

declare_id!("6Waiw2whwzAKVmqmsdSmJdBXtqyveYEMvPitRK3MoHRL");

#[program]
pub mod tradary_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
