{
  rustPlatform,
  lib,
  installShellFiles,
}:
let
  cargoToml = (lib.importTOML ./Cargo.toml);
in
rustPlatform.buildRustPackage rec {
  name = cargoToml.package.name;
  pname = cargoToml.package.name;

  version = cargoToml.package.version;

  src = ./.;

  nativeBuildInputs = [ installShellFiles ];

  buildFeatures = [ "bin" ];

  postInstall = ''
    installShellCompletion --cmd ${name} \
      --zsh <($out/bin/${name} completions_zsh) \
      --bash <($out/bin/${name} completions_bash) \
      --fish <($out/bin/${name} completions_fish)
  '';

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
