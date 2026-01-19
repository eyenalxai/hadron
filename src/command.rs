use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

pub struct ProtonCommand {
    pub proton_path: String,
    pub exe_path: String,
    pub compat_data_path: String,
    pub app_id: String,
    pub launch_options: Option<String>,
}

impl ProtonCommand {
    pub fn build_command(&self) -> String {
        let proton_cmd = format!("{} waitforexitandrun {}", self.proton_path, self.exe_path);

        match &self.launch_options {
            Some(opts) if opts.contains("%command%") => opts.replace("%command%", &proton_cmd),
            Some(opts) => format!("{} {}", opts, proton_cmd),
            None => proton_cmd,
        }
    }

    pub fn build_env(&self) -> Vec<(&str, &str)> {
        vec![
            ("STEAM_COMPAT_DATA_PATH", self.compat_data_path.as_str()),
            ("SteamAppId", self.app_id.as_str()),
            ("SteamGameId", self.app_id.as_str()),
        ]
    }

    pub fn execute(&self, dry_run: bool) -> Result<()> {
        let command_str = self.build_command();
        let env_vars = self.build_env();

        if dry_run {
            println!("Environment:");
            for (key, value) in &env_vars {
                println!("  {}={}", key, value);
            }
            println!("\nCommand:");
            println!("  {}", command_str);
            return Ok(());
        }

        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(&command_str);

        for (key, value) in env_vars {
            cmd.env(key, value);
        }

        if let Some(parent) = Path::new(&self.exe_path).parent() {
            cmd.current_dir(parent);
        }

        let status = cmd.status().context("Failed to execute command")?;

        if !status.success() {
            let code = status
                .code()
                .map_or_else(|| "unknown".to_string(), |c| c.to_string());
            anyhow::bail!("Command exited with status: {}", code);
        }

        Ok(())
    }
}
