use std::io::BufRead;
use std::io::IsTerminal;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use owo_colors::OwoColorize;

/// Environment variable that holds the Kilo API key. Must match
/// `KILO_ENV_KEY` in `codex-model-provider-info`.
const KILO_ENV_KEY: &str = "KILO_API_KEY";

/// File name (relative to the Codex home directory) where a key entered during
/// first-run setup is persisted so later starts do not re-prompt.
const KILO_KEY_FILE_NAME: &str = "kilo_api_key";

/// Resolve the Kilo API key for the current process.
///
/// Priority: the `KILO_API_KEY` environment variable, then a key previously
/// persisted to `{codex_home}/kilo_api_key`. When `allow_prompt` is true, stdin
/// and stdout are terminals, and neither source has a key, the user is asked to
/// paste their Kilo API key and it is persisted for future runs.
pub fn resolve_kilo_api_key(codex_home: &Path, allow_prompt: bool) {
    if env_key().is_some() {
        return;
    }
    if let Some(key) = read_key_file(codex_home) {
        set_env_key(key);
        return;
    }
    if allow_prompt && std::io::stdin().is_terminal() && std::io::stdout().is_terminal() {
        prompt_and_persist_key(codex_home);
    }
}

fn env_key() -> Option<String> {
    std::env::var(KILO_ENV_KEY).ok().filter(|value| !value.trim().is_empty())
}

fn key_file_path(codex_home: &Path) -> PathBuf {
    codex_home.join(KILO_KEY_FILE_NAME)
}

fn read_key_file(codex_home: &Path) -> Option<String> {
    let value = std::fs::read_to_string(key_file_path(codex_home)).ok()?;
    let value = value.trim().to_string();
    if value.is_empty() { None } else { Some(value) }
}

fn set_env_key(key: String) {
    // SAFETY: `KILO_API_KEY` is not read concurrently by other threads at
    // process startup, before any runtime has been built.
    unsafe { std::env::set_var(KILO_ENV_KEY, key) }
}

fn prompt_and_persist_key(codex_home: &Path) {
    println!(
        "{}",
        "Welcome to Codex (Kilo build)!".cyan().bold()
    );
    println!(
        "No Kilo API key was found in the {} environment variable.",
        KILO_ENV_KEY.cyan()
    );
    print!("Enter your Kilo API key (get one at https://app.kilo.ai): ");
    std::io::stdout().flush().ok();
    let mut line = String::new();
    if std::io::stdin().read_line(&mut line).is_err() {
        return;
    }
    let key = line.trim().to_string();
    if key.is_empty() {
        println!(
            "No key entered. Set {} and restart to use the Kilo provider.",
            KILO_ENV_KEY.cyan()
        );
        return;
    }
    if std::fs::create_dir_all(codex_home).is_err() {
        eprintln!(
            "Could not create {} to store your key.",
            codex_home.display()
        );
        return;
    }
    let path = key_file_path(codex_home);
    if let Err(err) = std::fs::write(&path, key.as_bytes()) {
        eprintln!("Failed to save Kilo API key to {}: {err}", path.display());
        return;
    }
    restrict_key_file_permissions(&path);
    println!("Saved your Kilo API key to {}.", path.display());
    set_env_key(key);
}

#[cfg(unix)]
fn restrict_key_file_permissions(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
}

#[cfg(not(unix))]
fn restrict_key_file_permissions(_path: &Path) {}