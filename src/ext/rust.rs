use crate::ext::util::{exists, run_command};

pub fn do_rust(args: &Vec<&str>) -> bool {
    // Check if there is a `Cargo.toml`, if not, quit.
    if !exists("Cargo.toml") {
        return false;
    }

    let first = args.get(0);

    match first {
        Some(val) => match *val {
            "run" => {
                run_command("cargo", vec!["run"]);
                return true;
            }
            "build" => {
                run_command("cargo", vec!["build"]);
                return true;
            }
            _ => {}
        },
        _ => {}
    }

    return false;
}
