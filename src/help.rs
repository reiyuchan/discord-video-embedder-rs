use std::env;

pub fn help() -> () {
    let efpath = env::current_exe().unwrap();

    println!(
        "Usage:\n {0} upload [filepath]\n {0} embed [url]",
        efpath.file_name().unwrap().to_str().unwrap(),
    )
}
