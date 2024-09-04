{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage {
  pname = "call-me";
  version = "0.1.0";

  src = pkgs.nix-gitignore.gitignoreSourcePure ./.gitignore ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    allowBuiltinFetchGit = true;
  };

  meta = {
    description = "Get a ping when someone joins your voice channel";
    homepage = "https://github.com/Jack5079/call-me";
    license = pkgs.lib.licenses.mpl20;
    mainProgram = "call-me";
  };
}
