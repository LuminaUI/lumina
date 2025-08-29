use crate::util::get_package_info::get_package_info;
use std::env;
use std::env::current_dir;
use std::path::PathBuf;

pub enum PackageRunners {
    PnpmDlx,
    Bunx,
    Npx,
    YarnDlx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManagerKind {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageManager {
    pub kind: PackageManagerKind,
    pub version: Option<String>,
}

impl PackageRunners {
    pub fn as_str(&self) -> &'static str {
        match self {
            PackageRunners::PnpmDlx => "pnpm dlx",
            PackageRunners::Bunx => "bunx",
            PackageRunners::Npx => "npx",
            PackageRunners::YarnDlx => "yarn dlx",
        }
    }
}

// TODO: Eventually detect from lock file & possibly PATH later on.
pub fn get_package_manager(cwd: &PathBuf) -> Option<PackageManager> {
    if let Some(pm) = detect_from_user_agent() {
        return Some(pm);
    }
    if let Some(pm) = detect_from_package_json() {
        return Some(pm);
    }

    None
}

pub fn get_package_runner(cwd: &PathBuf) -> Option<PackageRunners> {
    let package_manager = get_package_manager(cwd)?;

    match package_manager.kind {
        PackageManagerKind::Npm => Some(PackageRunners::Npx),
        PackageManagerKind::Yarn => Some(PackageRunners::YarnDlx),
        PackageManagerKind::Pnpm => Some(PackageRunners::PnpmDlx),
        PackageManagerKind::Bun => Some(PackageRunners::Bunx),
    }
}

fn detect_from_user_agent() -> Option<PackageManager> {
    let ua = env::var("npm_config_user_agent").ok()?;

    let first_token = ua.split_whitespace().next()?;
    let mut parts = first_token.split('/');
    let name = parts.next()?;
    let version = parts.next().map(|s| s.to_string());

    match name {
        "npm" => Some(PackageManager {
            kind: PackageManagerKind::Npm,
            version,
        }),
        "pnpm" => Some(PackageManager {
            kind: PackageManagerKind::Pnpm,
            version,
        }),
        "yarn" => Some(PackageManager {
            kind: PackageManagerKind::Yarn,
            version,
        }),
        "bun" => Some(PackageManager {
            kind: PackageManagerKind::Bun,
            version,
        }),
        _ => None,
    }
}

fn detect_from_package_json() -> Option<PackageManager> {
    let package_json = get_package_info(&current_dir().unwrap())?;
    let package_manager = &package_json.package_manager?;

    let (name, version) = package_manager
        .split_once('@')
        .map(|(n, v)| (n, Some(v.to_string())))
        .unwrap_or_else(|| (&package_manager.as_str(), None));

    match name {
        "npm" => Some(PackageManager {
            kind: PackageManagerKind::Npm,
            version,
        }),
        "yarn" => Some(PackageManager {
            kind: PackageManagerKind::Yarn,
            version,
        }),
        "bun" => Some(PackageManager {
            kind: PackageManagerKind::Bun,
            version,
        }),
        "pnpm" => Some(PackageManager {
            kind: PackageManagerKind::Pnpm,
            version,
        }),
        _ => None,
    }
}
