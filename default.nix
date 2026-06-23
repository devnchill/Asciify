{
  lib,
  rustPlatform,
  glib,
  pkg-config,
  ffmpeg,
  makeWrapper,
}:

rustPlatform.buildRustPackage {
  pname = "asciify";
  version = "0.1.0";

  src = lib.cleanSourceWith {
    filter =
      name: type:
      let
        baseName = baseNameOf (toString name);
      in
      baseName != "demo";
    src = lib.cleanSource ./.;
  };

  cargoLock.lockFile = ./Cargo.lock;

  buildInputs = [ glib ];
  nativeBuildInputs = [
    pkg-config
    makeWrapper
  ];

  postInstall = ''
    wrapProgram $out/bin/asciify \
      --prefix PATH : ${lib.makeBinPath [ ffmpeg ]}
  '';

  meta = {
    description = "Convert images and videos into ASCII art";
    homepage = "https://github.com/devnchill/Asciify";
    license = lib.licenses.asl20;
    mainProgram = "asciify";
  };
}
