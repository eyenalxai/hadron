mod command;
mod steam;
mod vdf;

use anyhow::{Result, bail};
use clap::Parser;

use crate::command::ProtonCommand;
use crate::steam::Steam;

#[derive(Parser)]
#[command(name = "hadron")]
#[command(about = "Launch Steam games with alternative executables through Proton")]
struct Args {
    #[arg(help = "Steam application ID")]
    app_id: String,

    #[arg(help = "Relative path to executable within game directory")]
    exe_path: String,

    #[arg(short = 'n', long, help = "Print command without executing")]
    dry_run: bool,

    #[arg(short = 'u', long, help = "Steam user ID for launch options")]
    user_id: Option<String>,

    #[arg(short = 's', long, help = "Steam directory path")]
    steam_dir: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    run(args)
}

fn run(args: Args) -> Result<()> {
    let steam = Steam::new(args.steam_dir)?;

    let library_path = steam.find_library_for_app(&args.app_id)?;
    let install_dir = steam.get_install_dir(&library_path, &args.app_id)?;
    let exe_full_path = install_dir.join(&args.exe_path);

    if !exe_full_path.exists() {
        bail!(
            "Executable not found: {}\nLooking in: {}",
            args.exe_path,
            install_dir.display()
        );
    }

    let compat_tool = steam.get_compat_tool(&args.app_id)?;
    let compat_tool_name = compat_tool
        .as_ref()
        .and_then(|t| t.name.as_ref())
        .map_or("proton_experimental", String::as_str);

    let proton_path = steam.get_proton_path(&library_path, compat_tool_name)?;
    let compat_data_path = steam.get_compat_data_path(&library_path, &args.app_id);

    let launch_options = steam.get_launch_options(args.user_id.as_deref(), &args.app_id)?;

    let cmd = ProtonCommand {
        proton_path: proton_path.to_string_lossy().to_string(),
        exe_path: exe_full_path.to_string_lossy().to_string(),
        compat_data_path: compat_data_path.to_string_lossy().to_string(),
        steam_client_path: steam.root_path().to_string_lossy().to_string(),
        app_id: args.app_id,
        launch_options,
    };

    cmd.execute(args.dry_run)
}
