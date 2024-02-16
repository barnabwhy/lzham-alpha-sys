#[cfg(feature = "generate_binding")]
use std::path::PathBuf;

#[cfg(feature = "generate_binding")]
fn generate_bindings() {
    const ALLOW_UNCONVENTIONALS: &'static str = "#![allow(non_upper_case_globals)]\n\
                                                 #![allow(non_camel_case_types)]\n\
                                                 #![allow(non_snake_case)]\n\
                                                 #![allow(improper_ctypes)]\n";

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .raw_line(ALLOW_UNCONVENTIONALS);

    let binding_target_path = PathBuf::new().join("src").join("lib.rs");

    // We need to override the target and sysroot for CLang on Windows GNU;
    // see https://github.com/rust-lang/rust-bindgen/issues/1760
    #[cfg(all(windows, target_env = "gnu"))]
    {
        let target = env::var("TARGET").unwrap();

        let bits = if cfg!(target_pointer_width = "32") {
            32
        } else {
            64
        };

        let target_arg = format!("--target={}", target);
        let sysroot_arg = format!(r#"--sysroot=C:\msys64\mingw{}\"#, bits);

        bindings = bindings.clang_args(&[&target_arg, &sysroot_arg]);
    }

    bindings
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(binding_target_path)
        .expect("Could not write binding to the file at `src/lib.rs`");

    println!("cargo:info=Successfully generated binding.");
}

fn main() {
    #[cfg(feature = "generate_binding")]
    generate_bindings();

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");

    let dst = cmake::build("lzham_alpha");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    let linking_text = "static";

    println!("cargo:rustc-link-lib={}=lzhamdecomp", linking_text);
    println!("cargo:rustc-link-lib={}=lzhamcomp", linking_text);
    println!("cargo:rustc-link-lib={}=lzhamdll", linking_text);
}
