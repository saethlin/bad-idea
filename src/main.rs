use bstr::ByteSlice;
use std::process::Command;
use walkdir::WalkDir;
use rand::Rng;

fn check() {
    let mut cmd = Command::new("cargo");
    let cmd = cmd
        .arg("+nightly")
        .arg("check")
        .arg("--all-features");
    let out = cmd.output().unwrap();
    if out.stderr.contains_str("thread 'rustc' panicked") {
        eprintln!("ICE Found!");
        std::process::exit(1);
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    check();
    // Find all source files in the project(probably)
    let files: Vec<_> = WalkDir::new("src")
        .into_iter()
        .map(|e| e.unwrap().path().to_owned())
        .collect();

    loop {
        // Modify a random byte in a random file
        let path = &files[rng.gen::<usize>() % files.len()];
        let mut contents = std::fs::read(path).unwrap();
        let b = &mut contents[rng.gen::<usize>()];
        *b = b.wrapping_add(1);
        std::fs::write(path, contents).unwrap();
        check();
    }
}
