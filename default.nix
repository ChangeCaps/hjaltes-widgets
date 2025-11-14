{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage {
  pname = "hjaltes-widgets";
  version = "0.1.0";
  src = pkgs.lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "ori-0.1.0" = "sha256-d/pS7IWa1t0dp5yyex5mnhoCQ9eiZHwkHKlF/IpBd5o=";
    };
  };

  nativeBuildInputs = [
    pkgs.pkg-config
  ];

  buildInputs = [
    pkgs.gtk4
    pkgs.gtk4-layer-shell
    pkgs.dbus
    pkgs.libadwaita
    pkgs.librsvg
  ];
}
