# -------------------------------------------------------------
# A Nix shell with latest stable Rust and cargo watch installed
# -------------------------------------------------------------
with import <nixpkgs> {};
let src = fetchFromGitHub {
      owner = "mozilla";
      repo = "nixpkgs-mozilla";
      # commit from: 2019-05-15
      rev = "9f35c4b09fd44a77227e79ff0c1b4b6a69dff533";
      sha256 = "18h0nvh55b5an4gmlgfbvwbyqj91bklf1zymis6lbdh75571qaz0";
   };
in
with import "${src.out}/rust-overlay.nix" pkgs pkgs;
stdenv.mkDerivation {
  name = "rust-env";
  buildInputs = [
    # Note: to use use nightly, just replace `stable` with `nightly`
    latest.rustChannels.stable.rust

    # Add some extra dependencies from `pkgs`
    pkgconfig cargo-watch
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;

  shellHook = ''
  '';
}
