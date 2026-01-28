fn main() {
    println!("cargo:rustc-link-lib=dylib=c_socket_lib");
    println!("cargo:rustc-link-search=target/debug");
}
