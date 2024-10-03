extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    // Options
    let build_c_bindings = env::var("INPUTTINO_BUILD_C_BINDINGS").unwrap_or("FALSE".to_string()) == "TRUE";
    let build_static = env::var("INPUTTINO_BUILD_STATIC").unwrap_or("FALSE".to_string()) == "TRUE";
    let libdir_path = env::var("INPUTTINO_LIBDIR_PATH").unwrap_or("../../");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut bindings = bindgen::Builder::default()
        .use_core()
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        // Set the INPUTTINO_STATIC_DEFINE macro
        .clang_arg(if build_static { "-D INPUTTINO_STATIC_DEFINE=1" } else { "" })
        // The input header we would like to generate bindings for.
        .header("wrapper.hpp");

    if build_c_bindings {
        let libdir_path = PathBuf::from(libdir_path)
            // Canonicalize the path as `rustc-link-search` requires an absolute
            // path.
            .canonicalize()
            .expect("cannot canonicalize path");

        // Compile the library using CMake
        let dst = Config::new(libdir_path)
            .target("libinputtino")
            .define("BUILD_SHARED_LIBS", if build_static { "OFF" } else { "ON" })
            .define("LIBINPUTTINO_INSTALL", "ON")
            .define("BUILD_TESTING", "OFF")
            .define("BUILD_SERVER", "OFF")
            .define("BUILD_C_BINDINGS", "ON")
            .profile("Release")
            .define("CMAKE_CONFIGURATION_TYPES", "Release")
            .build();

        println!("cargo:rustc-link-search=native={}/lib", dst.display());
        bindings = bindings.clang_arg(format!("-I{}/include/", dst.display()))
    } else {
        let lib = pkg_config::probe_library("libinputtino").unwrap();
        bindings = bindings.clang_arg(format!("-I{}", lib.include_paths[0].display()));
    }

    // Dependencies
    if !build_static {
        println!("cargo:rustc-link-lib=evdev");
        println!("cargo:rustc-link-lib=stdc++");
    }

    println!("cargo:rustc-link-lib={}libinputtino", if build_static { "static=" } else { "" });

    let out = bindings.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    out
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
