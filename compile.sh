cargo fmt

RUSTFLAGS="-C link-args=-T3dsx.ld -C link-args=-Map=rust_3ds.map" cargo xbuild --target 3ds.json

llvm-objdump -d -C target/3ds/debug/rust_3ds > rust_3ds.s
