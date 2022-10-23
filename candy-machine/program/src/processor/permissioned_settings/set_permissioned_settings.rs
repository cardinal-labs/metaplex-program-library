use anchor_lang::prelude::*;

use crate::{
    constants::PERMISSIONED_SETTINGS_FEATURE_INDEX,
    set_feature_flag,
    state::{PERMISSIONED_SETTINGS_SEED, PERMISSIONED_SETTINGS_SIZE},
    CandyMachine, PermissionedSettings,
};

/// Set the collection PDA for the candy machine
#[derive(Accounts)]
pub struct SetPermissionedSettings<'info> {
    #[account(mut, has_one = authority)]
    candy_machine: Account<'info, CandyMachine>,
    authority: Signer<'info>,
    #[account(
        init_if_needed,
        seeds = [PERMISSIONED_SETTINGS_SEED.as_bytes(), candy_machine.to_account_info().key.as_ref()],
        space = PERMISSIONED_SETTINGS_SIZE,
        payer = payer,
        bump
    )]
    permissioned_settings: Box<Account<'info, PermissionedSettings>>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handle_set_permissioned_settings(
    ctx: Context<SetPermissionedSettings>,
    creator: Pubkey,
) -> Result<()> {
    let candy_machine = &mut ctx.accounts.candy_machine;
    let permissioned_settings = &mut ctx.accounts.permissioned_settings;
    permissioned_settings.candy_machine = candy_machine.key();
    permissioned_settings.creator = creator;
    set_feature_flag(
        &mut candy_machine.data.uuid,
        PERMISSIONED_SETTINGS_FEATURE_INDEX,
    );
    Ok(())
}
