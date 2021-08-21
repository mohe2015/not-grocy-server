{
  description = "not-grocy-server's development flake";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        # https://github.com/NixOS/nixpkgs/issues/131557
        let pkgs = nixpkgs.legacyPackages.${system}; in # pkgsStatic
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
            libkrb5 = pkgs.krb5.overrideAttrs (old: {
              type = "lib";
              postInstall = ''
                ${old.postInstall}
                ls -R $out
                ls -R $dev
                rm -R $out/bin
                rm -R $out/sbin
              '';
            });

            libossp_uuid = pkgs.libossp_uuid.overrideAttrs (old: {
              postInstall = ''
                rm $out/bin/uuid-config
              '';
            });

            # TODO don't musl it's probably slow (especially memory allocation). try to reduce other things instead
            not-grocy-server = pkgs.rustPlatform.buildRustPackage rec {
              pname = "not-grocy-server";
              version = "0.1.0";
              src = pkgs.nix-gitignore.gitignoreSource [ ./.gitignore "kubernetes" "flake.nix" "result" ] ./.;

              nativeBuildInputs = [ pkgs.pkg-config ];
              buildInputs = [
                pkgs.sqlite
                (((pkgs.postgresql.override {
                  inherit libkrb5;
                  inherit libossp_uuid;
                  enableSystemd = false;
                }).overrideAttrs (old: {
                  doCheck = false; # would need to patch out two tests
                })).lib) # https://github.com/NixOS/nixpkgs/issues/61580
                (pkgs.mariadb-connector-c.override {
                  curl = pkgs.curl.override { # TODO use minimal curl / only libs
                    inherit libkrb5;
                  };
                })
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
/*
/nix/store/lld6ww2bivyz8ylhmf4xn3i7s7fk41m8-libkrb5-1.18
        └───bin/compile_et: …#!/nix/store/b45zavallnsvqwjs9wg9xw167jcs0935-bash-4.4-p23/bin/sh.#.#.AWK=gaw…
            → /nix/store/b45zavallnsvqwjs9wg9xw167jcs0935-bash-4.4-p23
*/
              # 64.7M
              # nix path-info -rsSh .#not-grocy-server
              # the following command is epic
              # nix why-depends .#not-grocy-server /nix/store/gmsv7hm0wd5siyhi4nsbn1aqpbcbi0cl-perl-5.32.1
              # docker load --input result
              # docker images
              # docker run -it not-grocy-server:latest
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

                services.radicale = {
                  enable = true;
                  settings = {
                    server = {
                      hosts = "0.0.0.0:5232";
                    };
                  };
                };

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

                networking.firewall.allowedTCPPorts = [ 5432 3306 5232 ];

                system.stateVersion = "21.11";
          })
        ];
      };
        }
      );
}


