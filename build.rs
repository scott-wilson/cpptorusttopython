fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("cpp/lib.cpp")
        .flag_if_supported("-std=c++11")
        .compile("cpptorusttopython");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=cpp/lib.h");
    println!("cargo:rerun-if-changed=cpp/lib.cpp");
}
