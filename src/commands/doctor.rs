use crate::error::Result;
use crate::ui::{self, WRENCH};
use console::style;
use std::process::Command;

pub fn run_doctor() -> Result<()> {
    ui::print_banner();
    ui::print_section_title(WRENCH, "Environment Check");

    check_rust_toolchain();
    check_android_ndk();
    check_cargo_ndk();
    check_rust_targets();

    println!();
    Ok(())
}

fn check_rust_toolchain() {
    print!("  Rust toolchain: ");
    match Command::new("rustc").arg("--version").output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim().replace("rustc ", "");
            println!("{} {}", style("✓").green().bold(), style(version).cyan());
        }
        _ => {
            println!("{} {}", style("✗").red().bold(), style("not found").red());
            ui::print_hint("Install from https://rustup.rs");
        }
    }
}

fn check_android_ndk() {
    print!("  Android NDK:    ");
    if let Ok(ndk_home) = std::env::var("ANDROID_NDK_HOME") {
        println!("{} {}", style("✓").green().bold(), style(ndk_home).cyan());
    } else if let Ok(ndk_home) = std::env::var("NDK_HOME") {
        println!("{} {}", style("✓").green().bold(), style(ndk_home).cyan());
    } else {
        println!(
            "{} {}",
            style("✗").red().bold(),
            style("ANDROID_NDK_HOME not set").red()
        );
        ui::print_hint("Set ANDROID_NDK_HOME environment variable");
    }
}

fn check_cargo_ndk() {
    print!("  cargo-ndk:      ");
    match Command::new("cargo").args(["ndk", "--version"]).output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!(
                "{} {}",
                style("✓").green().bold(),
                style(version.trim()).cyan()
            );
        }
        _ => {
            println!(
                "{} {}",
                style("✗").red().bold(),
                style("not installed").red()
            );
            ui::print_hint("cargo install cargo-ndk");
        }
    }
}

fn check_rust_targets() {
    println!();
    println!("  {}", style("Android Targets").bold());

    let targets = [
        ("aarch64-linux-android", "ARM64"),
        ("armv7-linux-androideabi", "ARMv7"),
        ("x86_64-linux-android", "x86_64"),
        ("i686-linux-android", "x86"),
    ];

    let output = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output();

    let installed: Vec<String> = match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect(),
        _ => Vec::new(),
    };

    for (target, label) in targets {
        if installed.iter().any(|t| t == target) {
            println!(
                "    {} {} {}",
                style("✓").green().bold(),
                style(format!("{:6}", label)).dim(),
                style(target).cyan()
            );
        } else {
            println!(
                "    {} {} {}",
                style("✗").red().bold(),
                style(format!("{:6}", label)).dim(),
                style(target).red()
            );
            println!("        {} rustup target add {}", style("→").dim(), target);
        }
    }
}
