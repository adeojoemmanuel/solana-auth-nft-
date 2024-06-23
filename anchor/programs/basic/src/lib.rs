use anchor_lang::prelude::*;

declare_id!("CmGapmbjwm1Ydzej4ggNtXpPpTqF4PCcAkzF6SZb4yAQ");

#[program]
pub mod basic {
    use super::*;

    pub fn greet(_ctx: Context<Initialize>) -> Result<()> {
        msg!("GM!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}