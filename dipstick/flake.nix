{
  description = "Nix shard fallback for Social Media Ethics Monitor (development shell + container build helpers)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    chainguard.url = "github:chainguard/chainguard";
  };

  outputs = { self, nixpkgs, chainguard }:
  let
    system = "x86_64-linux";
  in
  {
    devShells.${system}.default = nixpkgs.lib.mkShell {
      buildInputs = with nixpkgs.legacyPackages.${system}; [
        cargo
        deno
        elixir
        julia
        gnat
        docker
      ];
      shellHook = ''
        echo "Chainguard dev shell active (Nix fallback)."
        echo "Use chainguard/selur-compose binaries built via ${chainguard.outputs.packages.${system}.chainguard}"
      '';
    };
    packages.${system}.selur-compose-client = chainguard.outputs.packages.${system}.chainguard;
  };
