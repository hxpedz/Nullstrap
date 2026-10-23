{
  description = "Nullstrap, a Roblox bootstrapper that launches Roblox through Sober on Linux";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.self.submodules = true;

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      version = builtins.head (
        builtins.elemAt (builtins.split "<NullstrapVersion>([0-9.]+)</NullstrapVersion>" (builtins.readFile ./Directory.Build.props)) 1
      );
    in
    {
      packages = forAllSystems (pkgs: rec {
        nullstrap = pkgs.callPackage ./nix/package.nix { };
        nullstrap-git = nullstrap.overrideAttrs {
          inherit version;
          src = self;
        };
        default = nullstrap;
      });

      overlays.default = final: prev: {
        nullstrap = self.packages.${final.stdenv.hostPlatform.system}.nullstrap;
      };

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          inputsFrom = [ self.packages.${pkgs.stdenv.hostPlatform.system}.nullstrap-git ];
          packages = [
            pkgs.cargo
            pkgs.rustc
          ];
        };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt);
    };
}
