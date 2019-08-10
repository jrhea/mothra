use std::env;

fn main()
{
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search={}/target/release/", project_dir); // the "-L" flag
    println!("cargo:rustc-link-lib=libp2p_receive_gossip"); // the "-l" flag
}