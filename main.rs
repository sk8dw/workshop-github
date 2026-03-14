
fn main() {

    #[cfg(target_os = "macos")]
    compile_error!("❌ This program is intentionally unsupported on macOS.");
    println!("Very important rust code");
    
}
