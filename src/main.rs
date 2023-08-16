mod git;
mod io;
mod rp;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = io::read_config("./rp.json")?;

    let mut errors = 0;

    for repository in &config.repositories {
        if let Some(remote_name) = &repository.remote {
            let remote = match rp::get_remote(&config, remote_name) {
                Some(r) => r,
                None => {
                    eprintln!("Error: Remote '{}' not found for repository '{}'. Skipping.", remote_name, repository.name);
                    continue;
                },
            };

            let directory = format!("{}/{}", remote_name, repository.name);
            let repository_url = format!("{}{}", remote, repository.name);

            if git::clone(&repository_url, &directory).is_err() {
                errors += 1;
            }
        } else {
            let directory = match utils::basename(&repository.name) {
                Some(r) => r,
                None => {
                    eprintln!("Error: Invalid repository '{}'. Skipping.", repository.name);
                    continue;
                },
            };

            if git::clone(&repository.name, directory).is_err() {
                errors += 1;
            }
        }
    }

    if errors > 0 {
        eprintln!("Error: Total error count is {}.", errors);
        std::process::exit(123);
    }

    Ok(())
}
