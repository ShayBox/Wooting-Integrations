pub mod integrations;
pub mod wooting;

use std::process::Command;

#[must_use]
#[cfg(target_os = "linux")]
pub fn is_wootility_running() -> bool {
    match Command::new("pgrep").arg("wootility-lekker").output() {
        Ok(output) => !output.stdout.is_empty(),
        Err(_) => false,
    }
}

#[must_use]
#[cfg(target_os = "windows")]
pub fn is_wootility_running() -> bool {
    match Command::new("tasklist")
        .arg("/FI")
        .arg("imagename eq wootility-lekker.exe")
        .output()
    {
        Ok(output) => !output.stdout.is_empty(),
        Err(_) => false,
    }
}
