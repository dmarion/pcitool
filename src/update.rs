use semver::Version;
use serde::Deserialize;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

const UPDATE_TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    version: String,
}

pub struct UpdateChecker {
    rx: Receiver<Option<String>>,
}

impl UpdateChecker {
    pub fn start() -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let result = check_update_sync();
            let _ = tx.send(result);
        });

        Self { rx }
    }

    pub fn check_and_report(&self) {
        if let Ok(Some(new_version)) = self.rx.recv_timeout(Duration::from_millis(10)) {
            println!(
                "\n\x1b[32mA new version of {} is available: v{}\x1b[0m",
                env!("CARGO_PKG_NAME"),
                new_version
            );
            println!("To update, run:");
            println!("  cargo install --git {}", env!("CARGO_PKG_REPOSITORY"));
            println!();
            use std::io::Write;
            let _ = std::io::stdout().flush();
        }
    }
}

fn check_update_sync() -> Option<String> {
    let current_version_str = env!("CARGO_PKG_VERSION");
    let current_version = Version::parse(current_version_str).ok()?;

    let client = reqwest::blocking::Client::builder()
        .timeout(UPDATE_TIMEOUT)
        .user_agent("pcitool-updater")
        .build()
        .ok()?;

    let repo_url = env!("CARGO_PKG_REPOSITORY").trim_end_matches('/');
    let remote_url =
        repo_url.replace("github.com", "raw.githubusercontent.com") + "/master/Cargo.toml";

    let resp = client.get(&remote_url).send().ok()?;
    if !resp.status().is_success() {
        return None;
    }

    let text = resp.text().ok()?;

    let cargo: CargoToml = toml::from_str(&text).ok()?;
    let remote_version = Version::parse(&cargo.package.version).ok()?;

    if remote_version > current_version {
        Some(remote_version.to_string())
    } else {
        None
    }
}
