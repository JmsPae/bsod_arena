{ # Shitty dev enviroment
inputs = {
  nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  flake-utils.url = "github:numtide/flake-utils";
  rust-overlay.url = "github:oxalica/rust-overlay";
};
outputs = { self, nixpkgs, flake-utils, rust-overlay }:
flake-utils.lib.eachDefaultSystem
(system:
let
  overlays = [ (import rust-overlay) ];
  pkgs = import nixpkgs {
    inherit system overlays;
  };
  rustToolchain = pkgs.pkgsBuildHost.rust-bin.stable.latest.default.override {
    extensions = ["rust-src"];
  };

  buildInputs = with pkgs; [ 
    stdenv.cc.cc
    rustToolchain pkg-config rust-analyzer lld mold clang
    systemd
    alsa-lib 
    libGL vulkan-loader
    xorg.libX11 xorg.libXi xorg.libXcursor xorg.libXrandr vulkan-tools vulkan-headers vulkan-validation-layers
    wayland
    libxkbcommon
  ];
in
  with pkgs;
  {
    devShells.default = mkShell {
      buildInputs = buildInputs;
      shellHook = ''
      export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath buildInputs}"
      echo "[target.x86_64-unknown-linux-gnu]
      linker = \"clang\"  
      rustflags = [\"-C\", \"link-arg=-fuse-ld=${pkgs.mold}/bin/mold\"]" > .cargo/config.toml

      '';
    };
  }
  );
}
