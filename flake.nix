{
    description = "A program for tracking and visualizating health data";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
        fenix = {
            url = "github:nix-community/fenix";
            inputs.nixpkgs.follows = "nixpkgs";
        };
        flake-utils = {
            url = "github:numtide/flake-utils";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };
    outputs = inputs@{ self, nixpkgs, fenix, flake-utils, ... }: 
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system: let
        inherit (nixpkgs) lib;
    in {
        devShells = let
            pkgs = import nixpkgs {
                inherit system;
                config = { allowUnfree = true; };
            };
            toolchain = fenix.packages.${system}.complete.withComponents [
                "cargo"
                "rustc"
            ];
        in rec {
            default = pkgs.mkShell {
                name = "rust";
                buildInputs = with pkgs; [
                    toolchain
                ];
            };
        };
    });
}
