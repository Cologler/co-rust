use std::error::Error;
use std::process::{Command, exit};
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args()
        .skip(1) // ignore self
        .collect::<Vec<_>>();

    if args.len() > 0 {
        let mut cmd = Command::new(&args[0]);
        let code = cmd.args(args.iter().skip(1)).spawn()?.wait()?.code();
        if let Some(code) = code {
            if code != 0 {
                eprintln!("Program exit with code({}). Press Enter to continue...", code);
                std::io::stdin().bytes().next();
            }
            exit(code);
        }
        return Err("Command exited with no exit code".into());
    }

    Ok(())
}
