fn main()
{
    println!("cargo:rustc-link-lib=dylib=mothra");
    println!("cargo:rustc-link-search=native=./target/release");
}