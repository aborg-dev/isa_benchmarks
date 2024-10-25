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
        gnu32-pkgs = pkgs.pkgsCross.gnu32;
      in {
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.just
            pkgs.zig
            riscv64-pkgs.buildPackages.gcc
            riscv64-pkgs.buildPackages.buildPackages.qemu
            aarch64-pkgs.buildPackages.gcc
            aarch64-pkgs.buildPackages.buildPackages.qemu
            gnu32-pkgs.buildPackages.gcc
            gnu32-pkgs.buildPackages.buildPackages.qemu
          ];
        };
      });
}
