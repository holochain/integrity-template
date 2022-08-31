let
  holonixPath = (import ./nix/sources.nix).holonix;
  holonix = import (holonixPath) {
    rustVersion = { version = "1.63.0"; };
    holochainVersionId = "main";
    include = {
      node = false;
      happ = false;
      test = false;
      release = false;
      scaffolding = false;
    };
  };
  nixpkgs = holonix.pkgs;
in
nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = with nixpkgs; [
    nodejs-16_x
    sqlite
  ];
}
