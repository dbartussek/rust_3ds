. compile.sh

smdhtool --create "rust_3ds" "Description" "Author" "./icon.png" rust_3ds.smdh
3dsxtool ./rust_3ds.elf rust_3ds.3dsx --smdh=rust_3ds.smdh
