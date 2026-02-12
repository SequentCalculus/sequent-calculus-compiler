{
  description = "Sequent-Calculus-Compiler";

  inputs = { 
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable"; 
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustPlatform = pkgs.rustPlatform;
      in {
        devShells = {
          sccShell = pkgs.mkShell { buildInputs = [ self.packages.${system}.default ]; };
          default = self.devShells.${system}.sccShell;
        };

        packages = {
            sccPack = 
              let deps = [ pkgs.yasm pkgs.gcc ];
              in rustPlatform.buildRustPackage {
                pname = "Sequent-Calculus-Compiler";
                nativeBuildInputs = [ pkgs.makeWrapper ];
                BuildInputs = deps;
                version = "0.1.0";
                src = ./.;
                cargoDeps = rustPlatform.importCargoLock {
                  lockFile = ./Cargo.lock;
                };
                postFixup = ''
                  mv $out/bin/scc $out/bin/scc-unwrapped
                  makeWrapper $out/bin/scc-unwrapped $out/bin/scc \
                  --set PATH "${pkgs.lib.makeBinPath deps}";
                '';
              };
          default = self.packages.${system}.sccPack;
        };

        apps = {
          sccApp = {
            type = "app";
            program = "${self.packages.${system}.default}/bin/scc";
          };
          default = self.apps.${system}.sccApp;
        };
      });
}
