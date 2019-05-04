let
  nixpkgs-mozilla =
    import (fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);

  pkgs =
    import <nixpkgs> { overlays = [ nixpkgs-mozilla ]; };

  rust = (pkgs.rustChannelOf { date = "2019-04-21"; channel = "nightly"; }).rust.override {
    extensions = [
      "clippy-preview"
      "rust-src"
      "rls-preview"
      "rustfmt-preview"
    ];
  };

in
pkgs.mkShell {
  buildInputs = [
    rust
  ];
}
