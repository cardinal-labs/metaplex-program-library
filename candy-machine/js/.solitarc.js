// @ts-check
const path = require('path');
const programDir = path.join(__dirname, '..', 'program');
const idlDir = path.join(__dirname, 'idl');
const sdkDir = path.join(__dirname, 'src', 'generated');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'candy_machine',
  programId: 'ccmpgw68x3NJmNPePFrTm6TsKCEYUVhF8rEAVL9rSDd',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
