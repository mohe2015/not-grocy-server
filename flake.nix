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

          # TODO https://github.com/kolloch/crate2nix
          # TODO https://github.com/nix-community/naersk
          packages = rec {
            not-grocy-server = pkgs.rustPlatform.buildRustPackage rec {
              pname = "not-grocy-server";
              version = "0.1.0";
              src = ./.;

              nativeBuildInputs = [ pkgs.pkg-config ];
              buildInputs = [
                pkgs.sqlite
                pkgs.postgresql.lib # https://github.com/NixOS/nixpkgs/issues/61580
                pkgs.mysql57.connector-c
                pkgs.openssl
              ];

              cargoLock = {
                lockFile = ./Cargo.lock;
                outputHashes = {
                  "barrel-0.6.6-alpha.0" = "sha256-dhTctD8CQFhTUDbzp8QAilvpCZ8PMEzlXg4kDXy/3cw=";
                };
              };
            };

            container =
              # 743M
              # ls -Llh result
              # du -sh result/
              # nix path-info -rsSh .#not-grocy-server
              # the following command is epic
              # nix why-depends .#not-grocy-server /nix/store/gmsv7hm0wd5siyhi4nsbn1aqpbcbi0cl-perl-5.32.1
              # docker load --input result
              # docker images
              # docker run -it projektwahl-sveltekit
              # nodejs is probably 71MB but we also need lots of system libs
              # https://github.com/NixOS/nixpkgs/blob/master/pkgs/build-support/replace-dependency.nix
              pkgs.dockerTools.buildLayeredImage {
                name = "not-grocy-server";
                tag = "latest";

                contents = [ not-grocy-server ];

                config = {
                  # https://engineeringblog.yelp.com/2016/01/dumb-init-an-init-for-docker.html#process-behavior-inside-docker-containers
                  Cmd = [ "${pkgs.dumb-init}/bin/dumb-init" "${not-grocy-server}/bin/server" ];
                  WorkingDir = not-grocy-server;
                };
              };
          };

          # sudo nixos-container create not-grocy --flake .#x86_64-linux  # don't ask - just choose your architecture
          # psql -h not-grocy -U not-grocy
          # mysql -h not-grocy -u not-grocy -p
          # CREATE TABLE IF NOT EXISTS `products` (`id` INTEGER PRIMARY KEY NOT NULL, `product_id` INTEGER REFERENCES products(id) NOT NULL);
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


