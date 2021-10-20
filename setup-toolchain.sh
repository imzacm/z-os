#! /bin/bash
set -e

if ! command -v rustup &> /dev/null
then
  echo "Installing rustup"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
else
  echo "Updating rustup"
  rustup update
fi

# TODO: Ensure the following are installed: gcc, g++, make, bison, flex, libgmp, libmpc, libmpfr, texinfo

BASE_DIR="$(pwd)/toolchain"
DOWNLOAD_DIR="$BASE_DIR/sources"

export PREFIX="$BASE_DIR/prefix"
export TARGET=i686-elf
export PATH="$PREFIX/bin:$PATH"

mkdir -p "$DOWNLOAD_DIR" "$PREFIX"
pushd "$DOWNLOAD_DIR"

echo "Downloading, building and installing nasm"
NASM_SRC_URL="https://www.nasm.us/pub/nasm/releasebuilds/2.15.05/nasm-2.15.05.tar.xz"
rm -rf nasm-2.15.05 build-nasm-2.15.05
wget "$NASM_SRC_URL"
tar -xf nasm-2.15.05.tar.xz
mkdir build-nasm-2.15.05
pushd build-nasm-2.15.05
../nasm-2.15.05/configure --prefix="$PREFIX"
make
make strip
make install
popd

echo "Testing nasm"
nasm --version

echo "Downloading, building and installing binutils"
BINUTILS_SRC_URL="https://ftp.gnu.org/gnu/binutils/binutils-2.37.tar.xz"
rm -rf binutils-2.37 build-binutils-2.37
wget "$BINUTILS_SRC_URL"
tar -xf binutils-2.37.tar.xz
mkdir build-binutils-2.37
pushd build-binutils-2.37
../binutils-2.37/configure --target=$TARGET --prefix="$PREFIX" --with-sysroot --disable-nls --disable-werror
make
make install
popd

echo "Testing binutils"
$TARGET-ld --version

echo "Downloading, building and installing gcc"
GCC_SRC_URL="https://ftp.gnu.org/gnu/gcc/gcc-11.2.0/gcc-11.2.0.tar.xz"
rm -rf gcc-11.2.0 build-gcc-11.2.0
wget "$GCC_SRC_URL"
tar -xf gcc-11.2.0.tar.xz
mkdir build-gcc-11.2.0
pushd build-gcc-11.2.0
../gcc-11.2.0/configure --target=$TARGET --prefix="$PREFIX" --disable-nls --enable-languages=c,c++ --without-headers
make all-gcc
make all-target-libgcc
make install-gcc
make install-target-libgcc
popd

echo "Testing gcc"
$TARGET-gcc --version
