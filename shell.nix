let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "moz_overlay_shell";
    buildInputs = [
    # (nixpkgs.rustChannelOf {
      nixpkgs.latest.rustChannels.stable.rust
      pkgconfig
      alsaLib
      SDL2
      SDL2_image
      SDL2_mixer
      SDL2_gfx
    ];

    RUST_BACKTRACE = 1;

    # this needs to be added for games with sound to run in a pure environment
    PULSE_SERVER="unix:/run/user/1000/pulse/native";
  }
