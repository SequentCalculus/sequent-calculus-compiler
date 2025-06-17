{
  description="Grokking";

  inputs={
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs,... }:
    let 
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      rustPlatform = pkgs.rustPlatform; 
    in {
      devShells.${system}.default = pkgs.mkShell{
        buildInputs = [
          pkgs.cargo
          pkgs.yasm
        ];        
      };

      packages.${system}.default = rustPlatform.buildRustPackage {
        pname="Grokking";
        version="0.1.0";
        src=./.;
        cargoVendorDir="vendor";
        cargoDeps = rustPlatform.importCargoLock {
          lockFile = ./Cargo.lock;
          #outputHashes={
          #"https://github.com/brendanzab/codespan"
          #  "codespan-0.11.1"="sha256-Wq99v77bqSGIOK/iyv+x/EG1563XSeaTDW5K2X3kSXU=";
          #};
        }; 
      };

    };
}
