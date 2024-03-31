{
  description = "A wireguard configuration manager for nixos";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachSystem [
      "x86_64-linux"
      "aarch64-linux"
    ]
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          version = builtins.substring 0 8 self.lastModifiedDate;
        in
        {
          packages = {
            default = pkgs.rustPlatform.buildRustPackage {
              pname = "nixguard";
              inherit version;
              src = ./.;

              cargoHash = "sha256-lUhTaYNtgAD88SL3FWyclNaHZ7qI4rNGN5/Gn19huZ4=";
            };
          };

          apps.default = utils.lib.mkApp { drv = self.packages.${system}.default; };

          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [ go gopls gotools go-tools ];
          };
        });
}

