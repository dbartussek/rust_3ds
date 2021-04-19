cargo fmt

cargo xbuild --target 3ds.json

arm-none-eabi-gcc -specs=3dsx.specs -g -march=armv6k -mtune=mpcore -mfloat-abi=hard -mtp=soft -Wl,-Map,rust_3ds.map -Ltarget/3ds/debug  -LC:/devkitpro/libctru/lib -lrust_3ds -lctru -lm -o rust_3ds.elf

llvm-objdump -d -C rust_3ds.elf > rust_3ds.s
