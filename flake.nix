{
  description = "Hjalte's NixOS Widgets";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";

    lute = {
      url = "github:ChangeCaps/lute";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, systems, lute, ... }: {
    packages = nixpkgs.lib.genAttrs (import systems) (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in rec {
        hjaltes-widgets = import ./. { 
          inherit pkgs; 
          inherit lute;
        };
        default = hjaltes-widgets; 
      }
    );

    homeManagerModules = rec {
      hjaltes-widgets = import ./hm_module.nix self;
      default = hjaltes-widgets;
    };
  };
}
