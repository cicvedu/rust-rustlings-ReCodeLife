
//use std::env;
fn main() {
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    //env::set_var("TEST_FOO", timestamp); 
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:rustc-cfg=feature=\"pass\"");
}