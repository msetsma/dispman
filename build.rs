use clap::CommandFactory;
use std::fs;
use std::path::PathBuf;

include!("src/cli.rs");

fn main() {
    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=man/footer.1");
    println!("cargo:rerun-if-changed=build.rs");

    let cmd = Cli::command();
    let man = clap_mangen::Man::new(cmd);

    let mut buf: Vec<u8> = Vec::new();
    man.render_title(&mut buf).expect("render title");
    man.render_name_section(&mut buf).expect("render name");
    man.render_synopsis_section(&mut buf).expect("render synopsis");
    man.render_description_section(&mut buf)
        .expect("render description");
    man.render_options_section(&mut buf).expect("render options");
    man.render_subcommands_section(&mut buf)
        .expect("render subcommands");

    let footer = fs::read("man/footer.1").expect("read man/footer.1");
    buf.extend_from_slice(&footer);

    let out_path = PathBuf::from("man/dispman.1");
    let existing = fs::read(&out_path).ok();
    if existing.as_deref() != Some(buf.as_slice()) {
        fs::write(&out_path, &buf).expect("write man/dispman.1");
    }
}
