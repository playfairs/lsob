{
  description = "l_SOB image clarity destruction utility";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource ./.;
        commonArgs = {
          inherit src;
          pname = "lsob";
          version = "0.1.0";
          strictDeps = true;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.libGL ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [ pkgs.libxkbcommon pkgs.wayland pkgs.libxcursor pkgs.libxi pkgs.libxrandr ];
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        lsob = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
      in {
        packages.default = lsob;
        packages.lsob = lsob;
        checks = {
          build = lsob;
          fmt = craneLib.cargoFmt { inherit src; };
          clippy = craneLib.cargoClippy (commonArgs // { inherit cargoArtifacts; cargoClippyExtraArgs = "-- -D warnings"; });
        };
        devShells.default = craneLib.devShell {
          packages = [ pkgs.rustc pkgs.cargo pkgs.rustfmt pkgs.clippy pkgs.pkg-config ];
          inputsFrom = [ lsob ];
        };
      });
}
