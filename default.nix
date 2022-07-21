let
  holonixPath = (import ./nix/sources.nix).holonix;
  holonix = import (holonixPath) {
    holochainVersionId = "develop";
  };
  nixpkgs = holonix.pkgs;
in
nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = with nixpkgs; [
    nodejs-16_x
  ];
}
