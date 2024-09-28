{
  description = "";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.stdenv.mkDerivation {
        name = "shell";
        nativeBuildInputs = with pkgs; [
          pkg-config
          gcc
          rustc
          cargo
          rustfmt
          clippy
          rust-analyzer

          glib
          pango
          gdk-pixbuf
          gtk3
        ];
      };

      packages.${system}.default = pkgs.rust.packages.stable.rustPlatform.buildRustPackage {
        name = "hydrated_sloth";
        src = ./.;

        cargoHash = "sha256-1VwwBRmk87IpZjoR1cfQ5R+A26qtd2mMa7Shm0/Xd2g=";

        nativeBuildInputs = with pkgs; [
          pkg-config
          gcc
          rustc
          cargo
        ];

        buildInputs = with pkgs; [
          glib
          pango
          gdk-pixbuf
          gtk3
        ];
      };
    };
}
