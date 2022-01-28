use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    error::MetadataError,
    state::{PREFIX, FREEZER},
    utils::assert_derivation,
};
use solana_program::{
    entrypoint::ProgramResult,
    program_option::COption,
};

pub fn assert_freeze_authority_matches_mint(
    freeze_authority: &COption<Pubkey>,
    freeze_authority_info: &AccountInfo,
) -> ProgramResult {
    match freeze_authority {
        COption::None => {
            return Err(MetadataError::InvalidMintAuthority.into());
        }
        COption::Some(key) => {
            if freeze_authority_info.key != key {
                return Err(MetadataError::InvalidMintAuthority.into());
            }
        }
    }
    Ok(())
}

pub fn process_freeze_authority_validation(
    program_id: &Pubkey,
    freeze_authority_record_info: &AccountInfo,
    user_info: &AccountInfo,
    mint_info: &AccountInfo,
    must_be_empty: bool,
) -> Result<u8, ProgramError> {
    let record_info_empty = freeze_authority_record_info.try_data_is_empty()?;
    if must_be_empty {
        if !record_info_empty {
            return Err(MetadataError::FreezeAuthorityRecordAlreadyExists.into());
        }
    } else {
        if record_info_empty || freeze_authority_record_info.data.borrow()[0] == 0 {
            return Err(MetadataError::FreezeAuthorityRecordAlreadyRevoked.into());
        }
    }
    let freeze_authority_seeds = [
        PREFIX.as_bytes(),
        program_id.as_ref(),
        &mint_info.key.as_ref(),
        FREEZER.as_bytes(),
        &user_info.key.as_ref(),
    ];
    assert_derivation(&program_id, freeze_authority_record_info, &freeze_authority_seeds)
}
