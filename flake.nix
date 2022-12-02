{
  description = "Advent of Code Solutions - 2022";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs @ {
    self,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit self;} {
      systems = ["x86_64-linux" "x86_64-darwin"];
      perSystem = {
        config,
        lib,
        system,
        ...
      }: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [inputs.rust-overlay.overlays.default];
        };
        rustTools = (pkgs.rust-bin.fromRustupToolchainFile
          ./rust-toolchain.toml)
        .override {extensions = ["rust-analyzer" "rust-src"];};
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [pkgs.pkg-config];
          packages = with pkgs;
            [config.packages.cargo-aoc bacon cargo-watch rustTools]
            ++ lib.optional pkgs.stdenv.isDarwin pkgs.libiconv;

          CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER =
            lib.optional pkgs.stdenv.isLinux "${pkgs.clang}/bin/clang";
          RUSTFLAGS =
            lib.optional pkgs.stdenv.isLinux
            "-C link-arg=-fuse-ld=${pkgs.mold}/bin/mold";
          RUST_SRC_PATH = "${rustTools}/lib/rustlib/src/rust/library";
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
