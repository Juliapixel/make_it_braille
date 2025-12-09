{
  pkgs,
  rustPlatform,
  lib,
}:
let
  cargoToml = (lib.importTOML ./Cargo.toml);
in
rustPlatform.buildRustPackage {
  name = cargoToml.package.name;
  pname = cargoToml.package.name;

  version = cargoToml.package.version;

  src = ./.;

  buildFeatures = [ "bin" ];

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
