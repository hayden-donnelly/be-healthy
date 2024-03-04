{
    description = "A program for tracking and visualizating health data";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/22.11";
        flake-utils.url = "github:numtide/flake-utils/3db36a8b464d0c4532ba1c7dda728f4576d6d073";
    };
    outputs = inputs@{ self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system: let
        inherit (nixpkgs) lib;
        pkgs = import nixpkgs { inherit system; };
    in {
        devShells = let
            pyVer = "310";
            py = "python${pyVer}";
        in rec {
            default = pkgs.mkShell {
                name = "cuda";
                buildInputs = [
                    (pkgs.${py}.withPackages (pp: with pp; [
                        matplotlib
                    ]))
                ];
            };
        };
    });
}
