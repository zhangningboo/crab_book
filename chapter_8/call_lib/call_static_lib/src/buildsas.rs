
// build.rs
fn main() {
    println!("cargo:rustc-link-lib=static=lib_demo");
    println!("cargo:rustc-link-search=native={}", "/home/zhangningboo/workspace/workspace-rust/crab_book/chapter_8/call_lib/lib_demo/target/release");
}