{
  description = "Hjalte's NixOS Widgets";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    lute = {
      url = "github:ChangeCaps/lute";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, lute, ... }: 
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in {
        packages = rec {
          hjaltes-widgets = import ./. { 
            inherit pkgs; 
            inherit lute;
          };
          default = hjaltes-widgets;
        };

        homeManagerModules = import ./home.nix self;
      }
    );
}
