use std::fs;
use std::path::Path;

#[cfg(target_os = "windows")]
pub fn remove_tailscale_files() -> Result<(), String> {
    let paths = vec![
        r"C:\ProgramData\Tailscale",
        &format!(r"C:\Users\{}\AppData\Local\Tailscale",
            std::env::var("USERNAME").unwrap_or_else(|_| "".to_string())),
    ];

    remove_files(&paths)
}

#[cfg(target_os = "linux")]
pub fn remove_tailscale_files() -> Result<(), String> {
    let paths = vec![
        "/var/lib/tailscale/tailscaled.state",
    ];

    remove_files(&paths)
}

fn remove_files(paths: &[&str]) -> Result<(), String> {
    let mut had_errors = false;
    let mut permission_error = false;

    for path in paths {
        if Path::new(path).exists() {
            let result = if Path::new(path).is_dir() {
                fs::remove_dir_all(path)
            } else {
                fs::remove_file(path)
            };

            match result {
                Ok(_) => {
                    println!("  [✓] Removed: {}", path);
                }
                Err(e) => {
                    if is_permission_error(&e) {
                        permission_error = true;
                        println!("  [!] Permission denied: {}", path);
                    } else {
                        had_errors = true;
                        println!("  [!] Failed to remove {}: {}", path, e);
                    }
                }
            }
        } else {
            println!("  [-] Not found: {}", path);
        }
    }

    if permission_error {
        return Err("[!] Insufficient permissions. Please run with administrator privileges.".to_string());
    }

    if had_errors {
        return Err("Some files could not be removed.".to_string());
    }

    Ok(())
}

fn is_permission_error(error: &std::io::Error) -> bool {
    use std::io::ErrorKind;
    matches!(error.kind(), ErrorKind::PermissionDenied)
}

pub fn show_completion_message() {
    println!("\n  Completed. Please re-register your device to reactivate the machine.\n");
}
