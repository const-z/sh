fn main() {
    println!("cargo:rustc-link-lib=static=c_socket_lib");
    println!("cargo:rustc-link-search=target/debug");
}
