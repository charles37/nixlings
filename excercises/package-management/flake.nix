# I AM NOT DONE
{
  description = "Custom package set";

  outputs = {
    self,
    nixpkgs,
  }: {
    packages.default = nixpkgs.buildEnv {
      name = "custom-packages";
      paths = with nixpkgs; [
        # Add the necessary packages here
      ];
    };
  };
}
