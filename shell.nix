  with import <nixpkgs> {};

  stdenv.mkDerivation {
    name = "rust-env";
    buildInputs = [
        # Additional Dependencies
        openssl
    ];

    nativeBuildInputs = [ 
      pkg-config
    ];

    # Environment Variables
    # RUST_BACKTRACE=1
  }
