let
  nixpkgs-mozilla =
    import (fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);

  pkgs =
    import <nixpkgs> { overlays = [ nixpkgs-mozilla ]; };

  rustChannel =
    pkgs.rustChannelOf { date = "2019-04-21"; channel = "nightly"; };

  rust = rustChannel.rust.override {
    extensions = [
      "clippy-preview"
      "rls-preview"
      "rustfmt-preview"
    ];
  };

in
pkgs.mkShell {
  buildInputs = [
    rust
  ];
  shellHook = ''
    # https://github.com/rust-lang/rfcs/issues/2324#issuecomment-429521575
    mkdir -p ./target/doc
    cp -R --no-preserve=mode,ownership --dereference ${rustChannel.rust-docs}/share/doc/rust/html/* ./target/doc/
  '';
}
