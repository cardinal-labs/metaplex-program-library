export * from './CandyMachine';
export * from './CollectionPDA';
export * from './LockupSettings';
export * from './PermissionedSettings';

import { CandyMachine } from './CandyMachine';
import { CollectionPDA } from './CollectionPDA';
import { LockupSettings } from './LockupSettings';
import { PermissionedSettings } from './PermissionedSettings';

export const accountProviders = {
  CandyMachine,
  CollectionPDA,
  LockupSettings,
  PermissionedSettings,
};
