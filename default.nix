{
  rustPlatform,
  glib,
  pkg-config,
}:

rustPlatform.buildRustPackage {
  name = "asciify";
  buildInputs = [ glib ];
  nativeBuildInputs = [ pkg-config ];
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
