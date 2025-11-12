use std::{
    env,
    path::PathBuf
};

fn main() {
    const HEADERS_DIR: &str = "../bee2/include/bee2";
    println!("cargo:rerun-if-changed={}/", HEADERS_DIR);

    // Генерация биндингов
    let mut builder = bindgen::Builder::default()
        .clang_arg("-Ibee2")
        .allowlist_recursively(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    for entry in walkdir::WalkDir::new(HEADERS_DIR)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "h"))
    {
        println!("cargo:rerun-if-changed={}", entry.path().display());
        builder = builder.header(entry.path().to_str().unwrap());
    }

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let bee2_lib = env::var("BEE2_LIB")
        .unwrap_or_else(|_| "/usr/local/lib/libbee2_static.a".to_string());

    println!("cargo:rustc-link-arg={}", bee2_lib);
    println!("cargo:rerun-if-env-changed=BEE2_LIB");
}
