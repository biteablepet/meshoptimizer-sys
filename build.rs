use std::{
    env,
    path::PathBuf,
};

fn main() {
    let mut meshoptimizer_cmake = cmake::Config::new(
        PathBuf::from(
            env::var("CARGO_MANIFEST_DIR")
                .expect("CARGO_MANIFEST_DIR should be set while building"),
        )
        .join("vendor/meshoptimizer"),
    );
    meshoptimizer_cmake.cxxflag("-fkeep-inline-functions");

    if cfg!(feature = "lto") {
        // LTO feature requires Clang
        // FIXME: check cmake's cc somehow to see what compiler will be used
        meshoptimizer_cmake.cxxflag("-flto");
    }

    let meshoptimizer_build_dest = meshoptimizer_cmake.build().join("lib");

    println!(
        "cargo:rustc-link-search=native={}",
        meshoptimizer_build_dest.display()
    );
    println!("cargo:rustc-link-lib=static=meshoptimizer");

    bindgen::Builder::default()
        .header("wrapper.hpp")
        // allowlist the exports we want
        .allowlist_var("MESHOPTIMIZER_VERSION")
        .allowlist_item(r"^meshopt_.*$")
        .blocklist_item(r"^meshopt_Allocator.*$")
        // no-std
        .use_core()
        // flags and bitfields
        .rustified_enum("meshopt_EncodeExpMode")
        .bitfield_enum(r"^meshopt_Simplify.*$")
        // slightly cleaner wrappers
        .generate_cstr(true)
        .generate_inline_functions(true)
        .wrap_unsafe_ops(true)
        // generation
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("should generate bindings")
        .write_to_file(
            PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR should be set while building"))
                .join("bindings.rs"),
        )
        .expect("should write bindings");
}
