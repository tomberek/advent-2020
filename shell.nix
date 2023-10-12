let
  nixpkgs = import <nixpkgs> {};
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "rust";
    buildInputs = [ 
      clippy
      openssl pkg-config rustup cargo-flamegraph cmake zlib ];
  }
