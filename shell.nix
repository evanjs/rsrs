  with import <nixpkgs> {};

  stdenv.mkDerivation {
    name = "rust-env";
    buildInputs = [
        # Additional Dependencies
        pkgconfig openssl
    ];

    # Environment Variables
    RUST_BACKTRACE=1;
  }
