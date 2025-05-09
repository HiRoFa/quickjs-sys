use std::path::{Path, PathBuf};

use std::env;

fn exists(path: impl AsRef<Path>) -> bool {
    PathBuf::from(path.as_ref()).exists()
}

const LIB_NAME: &str = "quickjs";

#[cfg(all(not(feature = "bellard"), not(feature = "quickjs-ng")))]
fn main() {
    panic!("Invalid config for crate libquickjs-sys: must enable either the 'bellard' or the 'quickjs-ng' feature");
}

#[cfg(any(feature = "bellard", feature = "quickjs-ng"))]
fn main() {
    #[cfg(feature = "bellard")]
    let embed_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("bellard");
    #[cfg(feature = "quickjs-ng")]
    let embed_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("quickjs-ng");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let code_dir = out_path.join("quickjs");
    if exists(&code_dir) {
        std::fs::remove_dir_all(&code_dir).unwrap();
    }
    copy_dir::copy_dir(embed_path.join("quickjs"), &code_dir)
        .expect("Could not copy quickjs directory");

    std::fs::copy(
        embed_path.join("static-functions.c"),
        code_dir.join("static-functions.c"),
    )
    .expect("Could not copy static-functions.c");

    eprintln!("Compiling quickjs...");
    let quickjs_version =
        std::fs::read_to_string(code_dir.join("VERSION")).expect("failed to read quickjs version");

    #[cfg(feature = "bellard")]
    let files = [
        "cutils.c",
        "dtoa.c",
        "libregexp.c",
        "libunicode.c",
        "quickjs.c",
        // Custom wrappers.
        "static-functions.c",
    ];
    #[cfg(feature = "quickjs-ng")]
    let files = [
        "cutils.c",
        "xsum.c",
        "libregexp.c",
        "libunicode.c",
        "quickjs.c",
        // Custom wrappers.
        "static-functions.c",
    ];

    cc::Build::new()

        .files(
            files
            .iter()
            .map(|f| code_dir.join(f)),
        )
        .define("_GNU_SOURCE", None)
        .define(
            "CONFIG_VERSION",
            format!("\"{}\"", quickjs_version.trim()).as_str(),
        )
        .define("CONFIG_BIGNUM", None)
        // The below flags are used by the official Makefile.
        .flag_if_supported("-Wchar-subscripts")
        .flag_if_supported("-Wno-array-bounds")
        .flag_if_supported("-Wno-format-truncation")
        .flag_if_supported("-Wno-missing-field-initializers")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wundef")
        .flag_if_supported("-Wuninitialized")
        .flag_if_supported("-Wunused")
        .flag_if_supported("-Wwrite-strings")
        .flag_if_supported("-funsigned-char")
        // Below flags are added to supress warnings that appear on some
        // platforms.
        .flag_if_supported("-Wno-cast-function-type")
        .flag_if_supported("-Wno-implicit-fallthrough")
        .flag_if_supported("-Wno-enum-conversion")
        // cc uses the OPT_LEVEL env var by default, but we hardcode it to -O2
        // since release builds use -O3 which might be problematic for quickjs,
        // and debug builds only happen once anyway so the optimization slowdown
        // is fine.
        .opt_level(2)
        .compile(LIB_NAME);

    std::fs::copy(embed_path.join("bindings.rs"), out_path.join("bindings.rs"))
        .expect("Could not copy bindings.rs");
}
