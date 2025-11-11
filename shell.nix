{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.gtk4
    pkgs.gtk4-layer-shell
    pkgs.pkg-config
    pkgs.dbus
    pkgs.libadwaita
    pkgs.librsvg
  ];
}
