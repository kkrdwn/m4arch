use clap::{Parser, Subcommand};
use m4arch_service::{
    brightness_info, decrease_brightness, get_max_brightness, increase_brightness, set_brightness,
    set_rgb_color,
};

use serde_json::json;
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "m4arch-cli",
    version,
    about = "CLI tool to control M4Arch keyboard backlight and RGB",
    long_about = r#"
        m4arch-cli allows you to control keyboard brightness and RGB lighting.
        
        ⚠️  Some commands require write access to sysfs (may need sudo or udev rules).
    "#,
    after_help = r#"
        EXAMPLES:
        m4arch-cli get-brightness
        m4arch-cli get-max-brightness
        m4arch-cli set-brightness 128
        m4arch-cli set-brightness 0 --off
        m4arch-cli increase-brightness 10
        m4arch-cli decrease-brightness 10
        m4arch-cli set-rgb 255 0 0

        NOTES:
        • Commands that modify hardware may require root privileges
        • Use with caution to avoid hardware misuse
    "#
)]
struct Cli {
    /// Output result in JSON format
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Set the keyboard backlight brightness (0 to turn off)
    SetBrightness {
        /// Brightness level (0–255)
        #[arg(value_parser = clap::value_parser!(u8).range(0..=255))]
        level: u8,
    },

    /// Get the current keyboard backlight brightness
    GetBrightness,

    /// Get the maximum keyboard backlight brightness
    GetMaxBrightness,

    /// Increase keyboard backlight brightness
    IncreaseBrightness {
        /// Step value
        step: u8,
    },

    /// Decrease keyboard backlight brightness
    DecreaseBrightness {
        /// Step value
        step: u8,
    },

    /// Set the RGB color of the keyboard backlight
    SetRgb { r: u8, g: u8, b: u8 },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::SetBrightness { level } => {
            if !cli.json {
                println!(
                    "Setting brightness to {} (backlight: {})",
                    level,
                    if level > 0 { "ON" } else { "OFF" }
                );
            }

            set_brightness(level).map(|_| {
                if cli.json {
                    println!(
                        "{}",
                        json!({ "status": "ok", "level": level, "enabled": level > 0 })
                    );
                }
            })
        }

        Commands::GetBrightness => match brightness_info() {
            Ok(info) => {
                if cli.json {
                    println!("{}", json!(info));
                } else {
                    println!("Brightness information:");
                    println!("  Current : {}", info.current);
                    println!("  Max     : {}", info.max);
                    println!("  Percent : {}%", info.percent);
                    println!("  Status  : {}", if info.is_on { "ON" } else { "OFF" });
                }
                Ok(())
            }
            Err(e) => Err(e),
        },

        Commands::GetMaxBrightness => match get_max_brightness() {
            Ok(v) => {
                if cli.json {
                    println!("{}", json!({ "max_brightness": v }));
                } else {
                    println!("Max brightness: {}", v);
                }
                Ok(())
            }
            Err(e) => Err(e),
        },

        Commands::IncreaseBrightness { step } => increase_brightness(step).map(|_| {
            if cli.json {
                println!(
                    "{}",
                    json!({ "status": "ok", "action": "increase", "step": step })
                );
            }
        }),

        Commands::DecreaseBrightness { step } => decrease_brightness(step).map(|_| {
            if cli.json {
                println!(
                    "{}",
                    json!({ "status": "ok", "action": "decrease", "step": step })
                );
            }
        }),

        Commands::SetRgb { r, g, b } => set_rgb_color(r, g, b).map(|_| {
            if cli.json {
                println!(
                    "{}",
                    json!({ "status": "ok", "rgb": { "r": r, "g": g, "b": b } })
                );
            }
        }),
    };

    if let Err(e) = result {
        if cli.json {
            eprintln!("{}", json!({ "status": "error", "message": e.to_string() }));
        } else {
            eprintln!("Error: {}", e);
            eprintln!("Hint: run as root or fix udev permission");
        }
        process::exit(1);
    }
}
