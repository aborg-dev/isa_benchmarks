{
  description = "A flake with project build dependencies";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        riscv64-pkgs = pkgs.pkgsCross.riscv64;
        aarch64-pkgs = pkgs.pkgsCross.aarch64-multiplatform;
      in {
        devShells.default = pkgs.mkShell {
          packages = [
            riscv64-pkgs.buildPackages.gcc
            riscv64-pkgs.buildPackages.buildPackages.qemu
            aarch64-pkgs.buildPackages.gcc
            aarch64-pkgs.buildPackages.buildPackages.qemu
            # Qemu deps.
            # pkgs.pcre2
            # pkgs.gnutls
            # pkgs.libxkbcommon
            # pkgs.elfutils
            # pkgs.zstd
            # pkgs.libnfs
            # pkgs.libssh
            # pkgs.curl
          ];
        };
      });
}
