use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = &env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .derive_default(true)
        .impl_debug(true)
        .default_alias_style(bindgen::AliasVariation::NewTypeDeref)
        .header(format!("{crate_dir}/src/foo.h"))
        .allowlist_file(".*foo.h.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("foo_bindings.rs"))
        .expect("Couldn't write bindings");
}
