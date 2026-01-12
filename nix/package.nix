{
  lib,
  naersk,
  pkg-config,
  openssl,
}:

naersk.buildPackage {
  pname = "osatui";
  version = "0.1.0";
  src = lib.cleanSource ../.;
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  meta = with lib; {
    description = "Terminal UI for student schedules";
    homepage = "https://github.com/Bircoder432/osatui";
    license = licenses.mit;
    platforms = platforms.linux;
    mainProgram = "osatui";
  };
}
