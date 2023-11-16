use std::io::Write;
use std::{
    fs::{read_to_string, File},
    iter::FromIterator,
    path::PathBuf,
};

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
		
    let template_path = PathBuf::from_iter(["badges", "crates.io.template.svg"]);
    let template_svg_content = read_to_string(&template_path).expect("read template");

    let svg_content =
        template_svg_content.replace("CARGO_PKG_VERSION", std::env!("CARGO_PKG_VERSION"));

    let output_path = PathBuf::from_iter(["badges", "crates.io.svg"]);
    writeln!(
        File::create(&output_path).expect("open output"),
        "{svg_content}"
    )
    .expect("write output");
}
