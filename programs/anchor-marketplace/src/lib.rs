use anchor_lang::prelude::*;

declare_id!("9AH77yrfK1UTgLVfXnBcVBkns49KzYeq1uA7wQPudMju");

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
