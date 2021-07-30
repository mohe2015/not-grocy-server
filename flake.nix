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
              pkgs.bashInteractive # fix nested shells
              pkgs.pkg-config
              pkgs.diesel-cli
              pkgs.nixpkgs-fmt
              pkgs.sqlitebrowser
              pkgs.rustup
              pkgs.rust-analyzer
            ];

            buildInputs = [
              pkgs.sqlite
              pkgs.postgresql
              pkgs.mariadb
              pkgs.openssl
            ];
          };

          # sudo nixos-container create not-grocy --flake .#x86_64-linux  # don't ask - just choose your architecture
          # psql -h not-grocy -U not-grocy
          # mysql -h not-grocy -u not-grocy -p
          nixosConfigurations = nixpkgs.lib.nixosSystem {
            inherit system;
            modules = [
              ({ config, ... }: {
                boot.isContainer = true;

                networking.hostName = "not-grocy";

                services.mysql = {
                  enable = true;
                  package = pkgs.mariadb;
                };

                systemd.services.mysql-not-grocy-init = {
                  after = [ "mysql.service" ];
                  wantedBy = [ "multi-user.target" ];

                  serviceConfig = {
                    Type = "oneshot";
                    ExecStart = pkgs.writeShellScript "crdt-init.sh" ''
                    (
                      echo "CREATE DATABASE IF NOT EXISTS \`not-grocy\`;"
                      echo "CREATE USER IF NOT EXISTS 'not-grocy'@'%' IDENTIFIED BY 'not-grocy';"
                      echo "GRANT ALL PRIVILEGES ON \`not-grocy\`.* TO 'not-grocy'@'%';"
                    ) | ${config.services.mysql.package}/bin/mysql -N
                    '';
                  };
                };

                services.postgresql = {
                  enable = true;
                  package = pkgs.postgresql_13;
                  enableTCPIP = true;
                  authentication = "hostnossl all all 10.233.2.1 255.255.255.255 scram-sha-256";
                  settings = {
                    "password_encryption" = "scram-sha-256";
                  };
                };

                systemd.services.not-grocy-init = {
                  after = [ "postgresql.service" ];
                  wantedBy = [ "multi-user.target" ];

                  serviceConfig = {
                    Type = "oneshot";
                    User = "postgres";
                    Group = "postgres";
                    ExecStart = let psqlSetupCommands = pkgs.writeText "crdt-init.sql" ''
                      SELECT 'CREATE ROLE "not-grocy" LOGIN PASSWORD ''\'''\'not-grocy''\'''\''
                      WHERE
                      NOT
                      EXISTS
                      (SELECT FROM pg_roles WHERE rolname = '
                      not-grocy')\gexec
                    SELECT 'CREATE DATABASE "not-grocy" OWNER "not-grocy" TEMPLATE template0 ENCODING UTF8' WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'not-grocy')\gexec
                    \c 'not-grocy'
                    ''; in "${config.services.postgresql.package}/bin/psql -f ${psqlSetupCommands}";
                  };
                };

                networking.firewall.allowedTCPPorts = [ 5432 3306 ];

                system.stateVersion = "21.11";
          })
        ];
      };
        }
      );
}


