fn main() {
    // println!("cargo:rerun-if-changed=build.rs");
    // println!("cargo:rerun-if-changed=your_static_library.a");

  if false {
    println!("cargo:rustc-link-lib=static=zsv");
    println!("cargo:rustc-link-search=native=/nix/store/xkqbwjrd4cj3pxv2npvia8dhqg3lrcy1-zsv-0.3.8-alpha/lib");
  }
  if true {
    println!("cargo:rustc-link-lib=static=zsv");
    println!("cargo:rustc-link-search=native=/home/marc/projects-checked-out/rust/rust-jobvers-data-mixer/zsv-installed/lib");
  }
}
