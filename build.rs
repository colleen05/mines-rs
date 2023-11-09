use copy_to_output::copy_to_output;
use std::env;

fn copy_resources(resources_path: &str) {
    println!("cargo:rerun-if-changed={resources_path}/*");

    copy_to_output(
        resources_path,
        &env::var("PROFILE").expect("Could not get build type."),
    )
    .expect("Could not copy resources.");
}

fn main() {
    copy_resources("./resources/");
}
