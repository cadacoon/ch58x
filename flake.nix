{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShell = with pkgs; mkShell {
          buildInputs = [ toolchain rustfmt pre-commit rust-analyzer-unwrapped svdtools svd2rust wchisp ];
          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
