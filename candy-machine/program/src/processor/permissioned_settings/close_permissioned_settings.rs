use anchor_lang::prelude::*;

use crate::{
    constants::PERMISSIONED_SETTINGS_FEATURE_INDEX, remove_feature_flag,
    state::PERMISSIONED_SETTINGS_SEED, CandyError, CandyMachine, PermissionedSettings,
};

/// Set the collection PDA for the candy machine
#[derive(Accounts)]
pub struct ClosePermissionedSettings<'info> {
    /// CHECK: account may be empty
    #[account(mut)]
    candy_machine: UncheckedAccount<'info>,
    authority: Signer<'info>,
    #[account(
        mut,
        close = authority,
        seeds = [PERMISSIONED_SETTINGS_SEED.as_bytes(), candy_machine.to_account_info().key.as_ref()],
        bump
    )]
    permissioned_settings: Box<Account<'info, PermissionedSettings>>,
    system_program: Program<'info, System>,
}

pub fn handle_close_permissioned_settings(ctx: Context<ClosePermissionedSettings>) -> Result<()> {
    let candy_machine_info = &ctx.accounts.candy_machine;
    if !candy_machine_info.data_is_empty() {
        let candy_machine = &mut Account::<CandyMachine>::try_from(candy_machine_info)?;
        if candy_machine.authority != ctx.accounts.authority.key() {
            return err!(CandyError::InvalidCandyMachineAuthority);
        }
        remove_feature_flag(
            &mut candy_machine.data.uuid,
            PERMISSIONED_SETTINGS_FEATURE_INDEX,
        );
    }
    Ok(())
}
