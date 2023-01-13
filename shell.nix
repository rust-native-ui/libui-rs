with import <nixpkgs> {};
clangStdenv.mkDerivation {
  name = "iui-nix-shell";
  nativeBuildInputs = with pkgs; [
    cargo
    rustc
    gtk3
    pkgconfig
  ];
  shellHook = ''
export LIBCLANG_PATH="${pkgs.llvmPackages_latest.libclang.lib}/lib";
export LLVM_CONFIG_PATH="${pkgs.llvm}/bin/llvm-config";
 export BINDGEN_EXTRA_CLANG_ARGS="$(< ${stdenv.cc}/nix-support/libc-crt1-cflags)\
          $(< ${stdenv.cc}/nix-support/libc-cflags) \
          $(< ${stdenv.cc}/nix-support/cc-cflags) \
          $(< ${stdenv.cc}/nix-support/libcxx-cxxflags) \
          ${
            lib.optionalString stdenv.cc.isClang
            "-idirafter ${stdenv.cc.cc}/lib/clang/${
              lib.getVersion stdenv.cc.cc
            }/include"
          } \
          ${
            lib.optionalString stdenv.cc.isGNU
            "-isystem ${stdenv.cc.cc}/include/c++/${
              lib.getVersion stdenv.cc.cc
            } -isystem ${stdenv.cc.cc}/include/c++/${
              lib.getVersion stdenv.cc.cc
            }/${stdenv.hostPlatform.config} -idirafter
${stdenv.cc.cc}/lib/gcc/${stdenv.hostPlatform.config}/${
              lib.getVersion stdenv.cc.cc
            }/include"
          } \
        "
'';
}
