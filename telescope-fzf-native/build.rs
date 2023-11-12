use std::path::Path;

const SRC_DIR: &str = "./src";

const TELESCOPE_FZF_NATIVE_DIR: &str = "./telescope-fzf-native.nvim";

fn main() {
    let bindings = bindgen::Builder::default()
        .header(
            Path::new(TELESCOPE_FZF_NATIVE_DIR)
                .join("src")
                .join("fzf.h")
                .display()
                .to_string(),
        )
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(Path::new(SRC_DIR).join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut builder = cc::Build::new();

    let build = builder
        .file(
            Path::new(TELESCOPE_FZF_NATIVE_DIR)
                .join("src")
                .join("fzf.c"),
        )
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-parameter");

    build.compile("telescope-fzf-native");
}
