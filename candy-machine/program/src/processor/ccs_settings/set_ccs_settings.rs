use anchor_lang::prelude::*;

use crate::{
    constants::CCS_SETTINGS_FEATURE_INDEX,
    set_feature_flag,
    state::{CCS_SETTINGS_SEED, CCS_SETTINGS_SIZE},
    CCSSettings, CandyMachine,
};

/// Set the collection PDA for the candy machine
#[derive(Accounts)]
pub struct SetCCSSettingsCtx<'info> {
    #[account(mut, has_one = authority)]
    candy_machine: Account<'info, CandyMachine>,
    authority: Signer<'info>,
    #[account(
        init_if_needed,
        seeds = [CCS_SETTINGS_SEED.as_bytes(), candy_machine.to_account_info().key.as_ref()],
        space = CCS_SETTINGS_SIZE,
        payer = payer,
        bump
    )]
    ccs_settings: Box<Account<'info, CCSSettings>>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handle_set_ccs_settings(
    ctx: Context<SetCCSSettingsCtx>,
    creator: Pubkey,
    ruleset: Pubkey,
) -> Result<()> {
    let candy_machine = &mut ctx.accounts.candy_machine;
    let ccs_settings = &mut ctx.accounts.ccs_settings;
    ccs_settings.candy_machine = candy_machine.key();
    ccs_settings.creator = creator;
    ccs_settings.ruleset = ruleset;
    set_feature_flag(&mut candy_machine.data.uuid, CCS_SETTINGS_FEATURE_INDEX);
    Ok(())
}
