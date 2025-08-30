{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = with pkgs; mkShell {
          nativeBuildInputs = [
            rust-analyzer
            rustup

            svdtools
            svd2rust

            wchisp
          ];

          LD_LIBRARY_PATH = lib.makeLibraryPath [ stdenv.cc.cc.lib ];
        };
      }
    );
}
