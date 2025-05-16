use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn exists(path: impl AsRef<Path>) -> bool {
    PathBuf::from(path.as_ref()).exists()
}

const LIB_NAME: &str = "quickjs";

#[cfg(all(not(feature = "bellard"), not(feature = "quickjs-ng")))]
fn main() {
    panic!("Enable either the 'bellard' or the 'quickjs-ng' feature");
}

#[cfg(any(feature = "bellard", feature = "quickjs-ng"))]
fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    #[cfg(feature = "bellard")]
    let embed_path = manifest_dir.join("bellard");
    #[cfg(feature = "quickjs-ng")]
    let embed_path = manifest_dir.join("quickjs-ng");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let code_dir = out_path.join("quickjs");

    if exists(&code_dir) {
        fs::remove_dir_all(&code_dir).unwrap();
    }

    // Copy quickjs source directory
    copy_dir::copy_dir(embed_path.join("quickjs"), &code_dir).expect("Failed to copy quickjs dir");

    fs::copy(
        embed_path.join("static-functions.c"),
        code_dir.join("static-functions.c"),
    )
    .expect("Failed to copy static-functions.c");

    // Copy wrapper.h
    let wrapper_out = out_path.join("wrapper.h");
    fs::copy(embed_path.join("wrapper.h"), &wrapper_out).expect("Failed to copy wrapper.h");

    let quickjs_version =
        fs::read_to_string(code_dir.join("VERSION")).expect("Missing VERSION file");

    #[cfg(feature = "bellard")]
    let files = [
        "cutils.c",
        "dtoa.c",
        "libregexp.c",
        "libunicode.c",
        "quickjs.c",
        "static-functions.c",
    ];
    #[cfg(feature = "quickjs-ng")]
    let files = [
        "cutils.c",
        "xsum.c",
        "libregexp.c",
        "libunicode.c",
        "quickjs.c",
        "static-functions.c",
    ];

    cc::Build::new()
        .files(files.iter().map(|f| code_dir.join(f)))
        .define("_GNU_SOURCE", None)
        .define(
            "CONFIG_VERSION",
            format!("\"{}\"", quickjs_version.trim()).as_str(),
        )
        .define("CONFIG_BIGNUM", None)
        .flag_if_supported("-Wno-array-bounds")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wundef")
        .flag_if_supported("-Wuninitialized")
        .flag_if_supported("-Wunused")
        .flag_if_supported("-Wwrite-strings")
        .flag_if_supported("-funsigned-char")
        .flag_if_supported("-Wno-cast-function-type")
        .opt_level(2)
        .compile(LIB_NAME);

    let wrapper_h = embed_path.join("wrapper.h");

    let bindings = bindgen::Builder::default()
        .header(wrapper_h.to_string_lossy())
        .blocklist_item("FP_NORMAL")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .clang_arg(format!("-I{}", embed_path.join("quickjs").display()))
        .generate()
        .expect("Failed to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
