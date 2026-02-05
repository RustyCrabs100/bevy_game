fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    let target = std::env::var("TARGET").unwrap();

    if profile == "debug" {
        if std::env::var("CARGO_FEATURE_DEBUG").is_ok() {
            println!("cargo:rustc-cfg=__debug__");
            println!("cargo:rustc-check-cfg=cfg(__debug__)")
        }

        println!("cargo:warning=Debug Mode Enabled");
    }

    if target == "wasm32-unknown-unknown" {
        println!("cargo:rustc-cfg=__wasm32__");
        println!("cargo:rustc-check-cfg=cfg(__wasm32__)");

        println!("cargo:rustc-cfg=web_sys_unstable_apis");
        println!("cargo:rustc-cfg=webgpu");

        println!("cargo:rustc-cfg=wasm_target");
        println!("cargo:rustc-cfg=wasm_threads");
        println!("cargo:rustc-cfg=wasm_simd");
        println!("cargo:rustc-cfg=wasm_bulk_memory");

        println!("cargo:warning=Building with WASM Optimizations");
        println!("cargo:warning=__wasm32__ cfg has been enabled");
    } else {
        println!("cargo:rustc-cfg=__desktop__");
        println!("cargo:rustc-check-cfg=cfg(__desktop__)");

        println!("cargo:warning=Enabling __desktop__ cfg");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
