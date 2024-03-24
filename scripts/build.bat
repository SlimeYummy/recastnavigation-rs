@echo off

set CC=clang-cl
set CXX=clang-cl
set CFLAGS=/clang:-flto=thin /clang:-fuse-ld=lld-link
set CXXFLAGS=/clang:-flto=thin /clang:-fuse-ld=lld-link /EHsc
set AR=llvm-lib

SET RUSTFLAGS=-Clinker-plugin-lto -Clinker=lld-link -Clink-arg=-fuse-ld=lld-link

cargo build --all-features --target x86_64-pc-windows-msvc %*
