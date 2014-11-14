use std::io;
use std::io::Command;
use std::os;

fn run(command: &str, args: &[&str]) -> io::IoResult<io::process::ProcessExit> {
    try!(writeln!(std::io::stderr(), "running: {}", args));

    let mut command = Command::new(command);

    for arg in args.iter() {
        command.arg(*arg);
    }

    let result = command.status();

    try!(writeln!(std::io::stderr(), "result: {}", result));

    result
}

fn main() {
    let out_dir = os::getenv("OUT_DIR").unwrap();

    if !run("capnpc", ["-o", "./target/capnpc-rust", "../src/log.capnp"]).unwrap().success() {
        panic!("command failed to run");
    }

        /*
    /*
    Command::new("capnpc")
        .arg("-o").arg("./target/capnpc-rust")
        .arg("test.capnp")
        .status()
        .unwrap();
        */
    */

    println!("cargo:rustc-flags=-L {}", out_dir);
}
