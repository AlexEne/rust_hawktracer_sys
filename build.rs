#[cfg(feature = "bindgen")]
extern crate bindgen;
extern crate cmake;

#[cfg(feature = "pkg-config")]
extern crate pkg_config;
extern crate itertools;

use std::env;
use std::path::PathBuf;
use itertools::Itertools;

//In order to build 64 bit hawktracer:
// mkdir build
// cd build
// cmake .. -G "Visual Studio 15 2017 Win64" -T v141,host=x64
// cmake --build .
fn main() {
    let (main_header_path, header_paths) = if cfg!(feature = "pkg-config") {
        pkg_config()
    } else {
        build_project();
        let mut extra_include_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        extra_include_path.push("build");
        extra_include_path.push("lib");
        extra_include_path.push("include");
        ("hawktracer/lib/include/hawktracer.h", vec![extra_include_path, PathBuf::from("./hawktracer/lib/include/")])
    };

    println!("cargo:include={}", header_paths.iter().map(|p| p.display().to_string()).join(";"));

    generate_bindings(main_header_path, header_paths);

    let mut build_output_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    build_output_path.push("build");
    build_output_path.push("lib");
    let mut second_build_output_path = build_output_path.clone();
    second_build_output_path.push("RelWithDebInfo");
    let target = env::var("TARGET").unwrap();

    if target.contains("pc-windows") {
        #[cfg(debug_assertions)]
        {
            build_output_path.push("Debug");
        }

        #[cfg(not(debug_assertions))]
        {
            use std::path::Path;
            build_output_path.push("Release");
            if !Path::new(&build_output_path).exists() {
                //If debug = true is specified, then this generates RelWithDebInfo.
                if Path::new(&second_build_output_path).exists() {
                    build_output_path = second_build_output_path;
                }
            }
        }
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    }

    println!(
        "cargo:rustc-link-search=all={}",
        build_output_path.display()
    );
    if cfg!(feature = "pkg-config") {
        println!("cargo:rustc-link-lib=hawktracer");
    } else {
        println!("cargo:rustc-link-lib=static=hawktracer");
    }
}

fn build_project() {
    let configuration_type = {
        #[cfg(debug_assertions)]
        {
            "Debug"
        }
        #[cfg(not(debug_assertions))]
        {
            "Release"
        }
    };
    cmake::Config::new("hawktracer")
        .define("CMAKE_BUILD_TYPE", configuration_type)
        .define("BUILD_STATIC_LIB", "ON")
        .build_target("hawktracer")
        .build();
}

#[cfg(feature = "generate_bindings")]
fn generate_bindings(main_header_path: &str, header_paths: Vec<PathBuf>) {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .header(main_header_path)
        .clang_args(
            header_paths
                .into_iter()
                .map(|path| format!("-I{}", path.display())),
        )
        .blacklist_type("max_align_t")
        .generate()
        .expect("Unable to generate bindings");
    println!("Manifest dir: {:?}", manifest_dir);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(feature = "generate_bindings"))]
fn generate_bindings(_main_header_path: &str, _header_paths: Vec<PathBuf>) {
    use std::fs;
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(
        crate_path.join("pregenerated_bindings.rs"),
        out_path.join("bindings.rs"),
    )
    .expect("Couldn't find pregenerated bindings!");
}

#[cfg(feature = "pkg-config")]
fn pkg_config() -> (&'static str, Vec<PathBuf>) {
    let library = pkg_config::Config::new()
        .cargo_metadata(!cfg!(feature = "non-cargo"))
        .probe("hawktracer")
        .expect("Can't probe for hawktracer in pkg-config");
    ("wrapper.h", library.include_paths)
}

#[cfg(not(feature = "pkg-config"))]
fn pkg_config() -> (&'static str, Vec<PathBuf>) {
    unimplemented!()
}
