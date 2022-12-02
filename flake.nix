{
  description = "Advent of Code Solutions - 2022";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit self;} {
      systems = ["x86_64-linux" "x86_64-darwin"];
      perSystem = {
        config,
        lib,
        pkgs,
        ...
      }: {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [pkgs.pkg-config];
          packages = with pkgs; [cargo config.packages.cargo-aoc cargo-watch rust-analyzer] ++ lib.optional pkgs.stdenv.isDarwin pkgs.libiconv;
        };

        packages.cargo-aoc = let
          pname = "cargo-aoc";
          version = "0.3.3";
        in
          pkgs.rustPlatform.buildRustPackage {
            inherit pname version;

            src = pkgs.fetchCrate {
              inherit pname version;
              sha256 = "sha256-kRGsW5piOKQzmKp8RE+qjUOrfB1nDIkPnSFu1BQa5q8=";
            };

            cargoSha256 = "sha256-09EHGOG8V3x1/VuLLYFEDxyI8iJRukJVdlbGsPdZYag=";

            nativeBuildInputs = [pkgs.pkg-config];
            buildInputs =
              [pkgs.openssl.dev]
              ++ pkgs.lib.optional pkgs.stdenv.isDarwin
              pkgs.darwin.apple_sdk.frameworks.Security;
          };
      };
    };
}
