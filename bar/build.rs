fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let text = std::fs::read_to_string("../shared/baz.txt").unwrap();
    std::fs::write(
        format!("{out_dir}/hello.rs"),
        format!(r#"pub fn hello() {{ print!("{text}") }}"#),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=../shared/baz.txt");
}
