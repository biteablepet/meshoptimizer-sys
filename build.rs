use std::{
    env,
    path::PathBuf,
};

const MESHOPTIMIZER_SOURCE: &[&str] = &[
    "vendor/meshoptimizer/src/allocator.cpp",
    "vendor/meshoptimizer/src/clusterizer.cpp",
    "vendor/meshoptimizer/src/indexanalyzer.cpp",
    "vendor/meshoptimizer/src/indexcodec.cpp",
    "vendor/meshoptimizer/src/indexgenerator.cpp",
    "vendor/meshoptimizer/src/meshletcodec.cpp",
    "vendor/meshoptimizer/src/overdrawoptimizer.cpp",
    "vendor/meshoptimizer/src/partition.cpp",
    "vendor/meshoptimizer/src/quantization.cpp",
    "vendor/meshoptimizer/src/rasterizer.cpp",
    "vendor/meshoptimizer/src/simplifier.cpp",
    "vendor/meshoptimizer/src/spatialorder.cpp",
    "vendor/meshoptimizer/src/stripifier.cpp",
    "vendor/meshoptimizer/src/vcacheoptimizer.cpp",
    "vendor/meshoptimizer/src/vertexcodec.cpp",
    "vendor/meshoptimizer/src/vertexfilter.cpp",
    "vendor/meshoptimizer/src/vfetchoptimizer.cpp",
];

fn main() {
    let mut build = cc::Build::new();
    build.files(MESHOPTIMIZER_SOURCE);

    if cfg!(feature = "lto") {
        if build.get_compiler().is_like_clang() {
            build.flag("-flto");
        } else {
            panic!("LTO feature requires Clang");
        }
    }

    let is_debug = match env::var("DEBUG") {
        Ok(s) => match &*s.to_ascii_lowercase() {
            "true" | "0" => true,
            "false" | "1" => false,
            _ => panic!("Unknown value for DEBUG env var"),
        },
        Err(_) => false
    };
    if !is_debug {
        build.define("NDEBUG", None);
    }

    build.cpp(true);
    build.compile("meshoptimizer");

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
