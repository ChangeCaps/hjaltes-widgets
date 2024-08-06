{ pkgs, lute }:

pkgs.stdenv.mkDerivation {
  name = "hjaltes-widgets";
  version = "0.1.0";

  src = ./.;

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
    lute build hjaltes-widgets
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp out/hjaltes-widgets/hjaltes-widgets $out/bin
  '';
}
