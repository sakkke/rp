use std::process::Command;

pub fn clone(repository: &str, directory: &str) -> Result<(), String> {
    let status = Command::new("git")
        .arg("clone")
        .arg("-q")
        .arg(repository)
        .arg(directory)
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to clone".to_string())
    }
}
