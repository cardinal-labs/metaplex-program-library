use anchor_lang::prelude::*;

#[error_code]
pub enum CandyError {
    #[msg("Account does not have correct owner!")]
    IncorrectOwner,
    #[msg("Account is not initialized!")]
    Uninitialized,
    #[msg("Mint Mismatch!")]
    MintMismatch,
    #[msg("Index greater than length!")]
    IndexGreaterThanLength,
    #[msg("Numerical overflow error!")]
    NumericalOverflowError,
    #[msg("Can only provide up to 4 creators to candy machine (because candy machine is one)!")]
    TooManyCreators,
    #[msg("Uuid must be exactly of 6 length")]
    UuidMustBeExactly6Length,
    #[msg("Not enough tokens to pay for this minting")]
    NotEnoughTokens,
    #[msg("Not enough SOL to pay for this minting")]
    NotEnoughSOL,
    #[msg("Token transfer failed")]
    TokenTransferFailed,
    #[msg("Candy machine is empty!")]
    CandyMachineEmpty,
    #[msg("Candy machine is not live!")]
    CandyMachineNotLive,
    #[msg("Configs that are using hidden uris do not have config lines, they have a single hash representing hashed order")]
    HiddenSettingsConfigsDoNotHaveConfigLines,
    #[msg("Cannot change number of lines unless is a hidden config")]
    CannotChangeNumberOfLines,
    #[msg("Derived key invalid")]
    DerivedKeyInvalid,
    #[msg("Public key mismatch")]
    PublicKeyMismatch,
    #[msg("No whitelist token present")]
    NoWhitelistToken,
    #[msg("Token burn failed")]
    TokenBurnFailed,
    #[msg("Missing gateway app when required")]
    GatewayAppMissing,
    #[msg("Missing gateway token when required")]
    GatewayTokenMissing,
    #[msg("Invalid gateway token expire time")]
    GatewayTokenExpireTimeInvalid,
    #[msg("Missing gateway network expire feature when required")]
    NetworkExpireFeatureMissing,
    #[msg("Unable to find an unused config line near your random number index")]
    CannotFindUsableConfigLine,
    #[msg("Invalid string")]
    InvalidString,
    #[msg("Suspicious transaction detected")]
    SuspiciousTransaction,
    #[msg("Cannot Switch to Hidden Settings after items available is greater than 0")]
    CannotSwitchToHiddenSettings,
    #[msg("Incorrect SlotHashes PubKey")]
    IncorrectSlotHashesPubkey,
    #[msg("Incorrect collection NFT authority")]
    IncorrectCollectionAuthority,
    #[msg("Collection PDA address is invalid")]
    MismatchedCollectionPDA,
    #[msg("Provided mint account doesn't match collection PDA mint")]
    MismatchedCollectionMint,
    #[msg("Slot hashes Sysvar is empty")]
    SlotHashesEmpty,
    #[msg("The metadata account has data in it, and this must be empty to mint a new NFT")]
    MetadataAccountMustBeEmpty,
    #[msg("Missing set collection during mint IX for Candy Machine with collection set")]
    MissingSetCollectionDuringMint,
    #[msg("Can't change collection settings after items have begun to be minted")]
    NoChangingCollectionDuringMint,
    #[msg("Retain authority must be true for Candy Machines with a collection set")]
    CandyCollectionRequiresRetainAuthority,
    #[msg("Error within Gateway program")]
    GatewayProgramError,
    #[msg("Invalid candy machine authority")]
    InvalidCandyMachineAuthority,
    #[msg("Lockup settings lockup type is invalid")]
    InvalidLockupType,
    #[msg("Lockup settings account missing")]
    LockupSettingsAccountMissing,
    #[msg("Lockup settings account invalid")]
    LockupSettingsAccountInvalid,
    #[msg("Lockup settings missing accounts")]
    LockupSettingsMissingAccounts,
    #[msg("Lockup settings missing token manager")]
    LockupSettingsMissingTokenManager,
    #[msg("Lockup settings missing token managertoken account")]
    LockupSettingsMissingTokenManagerTokenAccount,
    #[msg("Lockup settings missing mint counter")]
    LockupSettingsMissingMintCounter,
    #[msg("Lockup settings missing recipient token account")]
    LockupSettingsMissingRecipientTokenAccount,
    #[msg("Lockup settings missing time invalidator")]
    LockupSettingsMissingTimeInvalidator,
    #[msg("Lockup settings missing time invalidator program")]
    LockupSettingsMissingTimeInvalidatorProgram,
    #[msg("Lockup settings invalid time invalidator program")]
    LockupSettingsInvalidTimeInvalidatorProgram,
    #[msg("Lockup settings missing token manager program")]
    LockupSettingsMissingTokenManagerProgram,
    #[msg("Lockup settings invalid token manager program")]
    LockupSettingsInvalidTokenManagerProgram,
    #[msg("Permissioned settings account invalid")]
    PermissionedSettingsAccountInvalid,
    #[msg("Permissioned settings missing mint manager")]
    PermissionedSettingsMissingMintManager,
    #[msg("Permissioned settings missing token manager")]
    PermissionedSettingsMissingTokenManager,
    #[msg("Permissioned settings missing token manager token account")]
    PermissionedSettingsMissingTokenManagerTokenAccount,
    #[msg("Permissioned settings missing mint counter")]
    PermissionedSettingsMissingMintCounter,
    #[msg("Permissioned settings missing recipient token accuont")]
    PermissionedSettingsMissingRecipientTokenAccount,
    #[msg("Permissioned settings missing mint manager")]
    PermissionedSettingsMissingMintMananger,
    #[msg("Permissioned settings missing token manager program")]
    PermissionedSettingsMissingTokenManagerProgram,
    #[msg("Permissioned settings invalid token manager program")]
    PermissionedSettingsInvalidTokenManagerProgram,
    #[msg("Permissioned settings missing collector")]
    PermissionedSettingsMissingCollector,
}
