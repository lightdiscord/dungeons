let

  nixpkgs-mozilla = builtins.fetchGit {
    url = "git@github.com:mozilla/nixpkgs-mozilla.git";
    rev = "5300241b41243cb8962fad284f0004afad187dad";
  };

  nixpkgs = import <nixpkgs> { overlays = [ (import nixpkgs-mozilla) ]; };

in nixpkgs.mkShell {
  buildInputs = [
    (nixpkgs.rustChannelOf { date = "2020-01-22"; channel = "nightly"; }).rust

    (nixpkgs.writeShellScriptBin "dev-watch" ''
      cargo install cargo-watch
      env RUST_LOG=''${1-trace} cargo watch -x test -x run
    '')
  ];
}
