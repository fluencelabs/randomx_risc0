fn main() {
        cc::Build::new()
            .object("src/randomx_combined.o")
            .compile("randomx_1");
}
