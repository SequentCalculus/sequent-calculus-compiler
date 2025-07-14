{
  description = "Sequent-Calculus-Compiler";

  inputs = { nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable"; };

  outputs = { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      rustPlatform = pkgs.rustPlatform;
    in {
      devShells.${system}.default =
        pkgs.mkShell { buildInputs = [ self.packages.${system}.default ]; };

      packages.${system}.default = let deps = [ pkgs.yasm pkgs.gcc ];
      in rustPlatform.buildRustPackage {
        pname = "Sequent-Calculus-Compiler";
        nativeBuildInputs = [ pkgs.makeWrapper ];
        BuildInputs = deps;
        version = "0.1.0";
        src = ./.;
        cargoDeps = rustPlatform.importCargoLock {
          lockFile = ./Cargo.lock;
          outputHashes = {
            "codespan-0.11.1" =
              "sha256-Wq99v77bqSGIOK/iyv+x/EG1563XSeaTDW5K2X3kSXU=";
          };
        };
        postFixup = ''
          mv $out/bin/scc $out/bin/scc-unwrapped
          makeWrapper $out/bin/scc-unwrapped $out/bin/scc \
          --set PATH "${pkgs.lib.makeBinPath deps}";
        '';

      };

    };
}
