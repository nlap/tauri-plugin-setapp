use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("apple-darwin") {
        println!("cargo:rustc-link-lib=static=SetappBridge");

        // Swift runtime
        println!("cargo:rustc-link-lib=dylib=swiftCore");
        println!("cargo:rustc-link-lib=dylib=swiftCompatibility56");
        println!("cargo:rustc-link-lib=dylib=swiftCompatibilityConcurrency");

        println!("cargo:rustc-link-search=lib/");
        println!("cargo:rustc-link-search=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx");
    }
}
