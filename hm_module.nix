self: { config, pkgs, lib, ... }:

let 
  cfg = config.programs.hjaltes-widgets;
in {
  options.programs.hjaltes-widgets = {
    enable = lib.mkEnableOption "Hjaltes Widgets";

    package = lib.mkOption {
      description = "The Hjalte's Widgets package";
      type = lib.types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.hjaltes-widgets;
    };

    style = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
    };
  };

  config = lib.mkIf (cfg.enable) {
    home.packages = [cfg.package];

    xdg.configFile."hjaltes-widgets/style.css".text = cfg.style;
  };
}
