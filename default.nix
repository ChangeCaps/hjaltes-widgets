{ pkgs, lute }:

pkgs.stdenv.mkDerivation {
  name = "hjaltes-widgets";
  version = "0.1.0";
  src = ./.;

  outputs = ["out"];

  nativeBuildInputs = [
    pkgs.clang-tools
    pkgs.clang
    pkgs.pkg-config
    lute.packages.${pkgs.system}.lute
  ];

  buildInputs = [
    pkgs.gtk4
    pkgs.gtk4-layer-shell
    pkgs.pulseaudio
  ];

  buildPhase = ''
    lute build hjaltes-widgets --release
  '';

  installPhase = ''
    lute install hjaltes-widgets --nix
  '';
}
