# I AM NOT DONE
{
  description = "Hello, Nix!";

  outputs = {
    self,
    nixpkgs,
  }: {
    packages.default = nixpkgs.stdenv.mkDerivation {
      name = "hello-nix";
      src = self;
      buildPhase = "gcc -o hello hello.c";
      installPhase = "mkdir -p $out/bin; cp hello $out/bin/";
    };
  };
}
