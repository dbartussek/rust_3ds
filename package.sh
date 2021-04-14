. compile.sh

smdhtool --create "rust_3ds" "Description" "Author" "./icon.png" rust_3ds.smdh
3dsxtool ./target/3ds/debug/rust_3ds rust_3ds.3dsx --smdh=rust_3ds.smdh
