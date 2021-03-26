cargo fmt

RUSTFLAGS="-C link-args=-T3dsx.ld -C link-args=-Map=3ds.map" cargo xbuild --target 3ds.json
