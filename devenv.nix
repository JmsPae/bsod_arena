{ pkgs, lib, config, inputs, ... }:
let
  deps = with pkgs; [
    stdenv.cc.cc
    pkg-config
    systemd
    alsa-lib 
    libGL vulkan-loader
    xorg.libX11 xorg.libXi xorg.libXcursor xorg.libXrandr vulkan-tools vulkan-headers vulkan-validation-layers
    wayland
    libxkbcommon
  ];
in {
  # https://devenv.sh/basics/
  env.GREET = "bsod_arena dev";

  # https://devenv.sh/packages/
  packages = with pkgs; [ 
    clang lld mold
  ] ++ deps;

  # https://devenv.sh/scripts/
  languages.rust = {
    enable = true;
    # https://devenv.sh/reference/options/#languagesrustchannel
    channel = "nightly";

    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  enterShell = ''
  export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath deps}"
  echo "[target.x86_64-unknown-linux-gnu]
  linker = \"clang\"  
  rustflags = [\"-C\", \"link-arg=-fuse-ld=${pkgs.mold}/bin/mold\"]" > .cargo/config.toml
  '';
}
