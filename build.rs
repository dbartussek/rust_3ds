fn main() {
    cc::Build::new()
        .target("arm-none-eabihf")
        .compiler("clang")
        .file("src/start.s")
        .compile("start");
}
