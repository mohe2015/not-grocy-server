{
  description = "not-grocy-server's development flake";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs = nixpkgs.legacyPackages.${system}; in
        {
          devShell = pkgs.mkShell {
            nativeBuildInputs = [
              pkgs.pkg-config
              pkgs.diesel-cli
            ];

            buildInputs = [
                pkgs.sqlite
                pkgs.postgresql
            ];
          };
        }
      );
}
