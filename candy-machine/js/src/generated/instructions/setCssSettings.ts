/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js';
import * as beetSolana from '@metaplex-foundation/beet-solana';
import * as beet from '@metaplex-foundation/beet';

/**
 * @category Instructions
 * @category SetCssSettings
 * @category generated
 */
export type SetCssSettingsInstructionArgs = {
  creator: web3.PublicKey;
  ruleset: web3.PublicKey;
};
/**
 * @category Instructions
 * @category SetCssSettings
 * @category generated
 */
export const setCssSettingsStruct = new beet.BeetArgsStruct<
  SetCssSettingsInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['creator', beetSolana.publicKey],
    ['ruleset', beetSolana.publicKey],
  ],
  'SetCssSettingsInstructionArgs',
);
/**
 * Accounts required by the _setCssSettings_ instruction
 *
 * @property [_writable_] candyMachine
 * @property [**signer**] authority
 * @property [_writable_] ccsSettings
 * @property [_writable_, **signer**] payer
 * @category Instructions
 * @category SetCssSettings
 * @category generated
 */
export type SetCssSettingsInstructionAccounts = {
  candyMachine: web3.PublicKey;
  authority: web3.PublicKey;
  ccsSettings: web3.PublicKey;
  payer: web3.PublicKey;
  systemProgram?: web3.PublicKey;
};

export const setCssSettingsInstructionDiscriminator = [39, 81, 171, 122, 148, 62, 2, 149];

/**
 * Creates a _SetCssSettings_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category SetCssSettings
 * @category generated
 */
export function createSetCssSettingsInstruction(
  accounts: SetCssSettingsInstructionAccounts,
  args: SetCssSettingsInstructionArgs,
  programId = new web3.PublicKey('ccmpgw68x3NJmNPePFrTm6TsKCEYUVhF8rEAVL9rSDd'),
) {
  const [data] = setCssSettingsStruct.serialize({
    instructionDiscriminator: setCssSettingsInstructionDiscriminator,
    ...args,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.candyMachine,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.authority,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.ccsSettings,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.payer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
  ];

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  });
  return ix;
}
