let
  holonixPath = (import ./nix/sources.nix).holonix;
  holonix = import (holonixPath) {
    rustVersion = { 
      track = "stable";
      version = "1.64.0"; 
    };
    holochainVersionId = "v0_0_175";
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
