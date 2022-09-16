use std::env;
use std::path::Path;
use ethers::contract::Abigen;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("multicall2.rs");
    Abigen::new("Multicall2", "./src/multicall2_abi.json").unwrap()
        .generate().unwrap()
        .write_to_file(dest_path).unwrap();

    let dest_path = Path::new(&out_dir).join("multicall.rs");
    Abigen::new("Multicall", "./src/multicall_abi.json").unwrap()
        .generate().unwrap()
        .write_to_file(dest_path).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
