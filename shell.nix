with import <nixpkgs> {};
  mkShell {
    buildInputs = [
      p7zip
    ];
    PKG_CONFIG_PATH = "${wayland.dev}/lib/pkgconfig" + "${openssl.dev}/lib/pkgconfig";
    LD_LIBRARY_PATH = lib.makeLibraryPath [wayland openssl.dev];
  }
