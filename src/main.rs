mod cli;

use cli::parse_args;

fn main() {
    let args = parse_args();
    match args.command {
        Some(command) => match command {
            cli::Command::Upload(upload) => {
                println!("Uploading file: {}", upload.file);
                if let Some(folder) = upload.folder {
                    println!("To folder: {}", folder);
                }
            }
            cli::Command::List(_) => {
                println!("Listing remotes");
            }
        },
        None => {
            println!("No command provided");
        }
    }
}