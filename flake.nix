{
  inputs = {
    nearsk = {
      url = "github:nix-community/naersk";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        fenix.follows = "fenix";
      };
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    {
      nearsk,
      fenix,
      nixpkgs,
      utils,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let

        pkgs = import nixpkgs {
          inherit system;
        };

        # inherit (pkgs) lib;

        toolchain = fenix.packages.${system}.stable.toolchain;

        nearsk' = pkgs.callPackage nearsk {
          cargo = toolchain;
          rustc = toolchain;
        };

        cli = nearsk'.buildPackage {
          name = "cli";
          pname = "palette-mapper";
          src = ./.;
        };
      in
      {
        packages = {
          inherit cli;
          default = cli;
        };

        devShells = {
          default = pkgs.mkShell {
            packages = with pkgs; [
              toolchain

              # benchmarking and testing
              cargo-criterion
              cargo-insta

              # tools i like using
              watchexec
            ];
          };
        };
      }
    );
}
