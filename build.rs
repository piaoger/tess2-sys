#[cfg(feature = "update-bindings")]
fn generate_bindings() {
    use std::env;
    use std::path::PathBuf;
    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    bindgen::Builder::default()
        .header("./native/Include/tesselator.h")
        .generate_inline_functions(true)
        .generate_comments(true)
        .rustfmt_bindings(true)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        //.default_enum_style(bindgen::EnumVariation::ModuleConsts) // Bitfield,ModuleConsts,Consts,Rust { non_exhaustive: false }
        .generate()
        .expect("Unable to generate bindings.")
        .write_to_file(out_path.join("src").join("bindings.rs"))
        .expect("Couldn't write bindings.");
}

#[cfg(not(feature = "update-bindings"))]
fn generate_bindings() {
    // nothing
}

fn main() {
    let files = vec![
        "./native/Source/bucketalloc.c",
        "./native/Source/dict.c",
        "./native/Source/geom.c",
        "./native/Source/mesh.c",
        "./native/Source/priorityq.c",
        "./native/Source/sweep.c",
        "./native/Source/tess.c",
    ];

    cc::Build::new()
        // .cpp(true)
        .include("./native/Include")
        .files(&files)
        .flag_if_supported("-O2")
        // .flag_if_supported("-std=c++11")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wno-null-dereference")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("libtess2.a");

    generate_bindings();
}
