use std::path::Path;

pub fn basename(s: &str) -> Option<&str> {
    Path::new(s)
        .file_name()
        .and_then(|name| name.to_str())
}
