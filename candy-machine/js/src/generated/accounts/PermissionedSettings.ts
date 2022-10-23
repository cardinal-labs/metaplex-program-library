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
 * Arguments used to create {@link PermissionedSettings}
 * @category Accounts
 * @category generated
 */
export type PermissionedSettingsArgs = {
  candyMachine: web3.PublicKey;
  creator: web3.PublicKey;
};

export const permissionedSettingsDiscriminator = [19, 206, 24, 59, 125, 147, 136, 121];
/**
 * Holds the data for the {@link PermissionedSettings} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class PermissionedSettings implements PermissionedSettingsArgs {
  private constructor(readonly candyMachine: web3.PublicKey, readonly creator: web3.PublicKey) {}

  /**
   * Creates a {@link PermissionedSettings} instance from the provided args.
   */
  static fromArgs(args: PermissionedSettingsArgs) {
    return new PermissionedSettings(args.candyMachine, args.creator);
  }

  /**
   * Deserializes the {@link PermissionedSettings} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0,
  ): [PermissionedSettings, number] {
    return PermissionedSettings.deserialize(accountInfo.data, offset);
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link PermissionedSettings} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
  ): Promise<PermissionedSettings> {
    const accountInfo = await connection.getAccountInfo(address);
    if (accountInfo == null) {
      throw new Error(`Unable to find PermissionedSettings account at ${address}`);
    }
    return PermissionedSettings.fromAccountInfo(accountInfo, 0)[0];
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey('cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ'),
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, permissionedSettingsBeet);
  }

  /**
   * Deserializes the {@link PermissionedSettings} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [PermissionedSettings, number] {
    return permissionedSettingsBeet.deserialize(buf, offset);
  }

  /**
   * Serializes the {@link PermissionedSettings} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return permissionedSettingsBeet.serialize({
      accountDiscriminator: permissionedSettingsDiscriminator,
      ...this,
    });
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link PermissionedSettings}
   */
  static get byteSize() {
    return permissionedSettingsBeet.byteSize;
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link PermissionedSettings} data from rent
   *
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    connection: web3.Connection,
    commitment?: web3.Commitment,
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(PermissionedSettings.byteSize, commitment);
  }

  /**
   * Determines if the provided {@link Buffer} has the correct byte size to
   * hold {@link PermissionedSettings} data.
   */
  static hasCorrectByteSize(buf: Buffer, offset = 0) {
    return buf.byteLength - offset === PermissionedSettings.byteSize;
  }

  /**
   * Returns a readable version of {@link PermissionedSettings} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      candyMachine: this.candyMachine.toBase58(),
      creator: this.creator.toBase58(),
    };
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const permissionedSettingsBeet = new beet.BeetStruct<
  PermissionedSettings,
  PermissionedSettingsArgs & {
    accountDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ['accountDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['candyMachine', beetSolana.publicKey],
    ['creator', beetSolana.publicKey],
  ],
  PermissionedSettings.fromArgs,
  'PermissionedSettings',
);
