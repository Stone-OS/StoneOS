use std::fs;

const LD_SCRIPT_PATH: &str = "./src/bsp/rpi4/memory";
const KERNEL_LINKER_SCRIPT: &str = "kernel.ld";

fn main() {
    if fs::read_dir(LD_SCRIPT_PATH).unwrap().next().is_some() {
        println!("cargo:rustc-link-arg=--library-path={}", LD_SCRIPT_PATH);
        println!("cargo:rustc-link-arg=--script={}", KERNEL_LINKER_SCRIPT);

        let files = fs::read_dir(LD_SCRIPT_PATH)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|d| {
                if let Some(e) = d.path().extension() {
                    e == "ld"
                } else {
                    false
                }
            });

        files.for_each(|f| println!("cargo:rerun-if-changed={}", f.path().display()));
    }
}