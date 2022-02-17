use anyhow::Result;
use std::{env, fs, process::Command, time::Duration};
use tokio::{task, time};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CmdOutput {
    #[serde(rename = "CommandLine")]
    command_line: Option<String>,
}

pub struct LCU {
    auth_url: String,
}

impl LCU {
    pub fn new() -> Self {
        Self {
            auth_url: String::new(),
        }
    }

    pub async fn parse_auth(&self) -> Result<()> {
        let forever = task::spawn(async {
            let mut interval = time::interval(Duration::from_secs(5));

            loop {
                interval.tick().await;

                let tmp = env::temp_dir();
                let output_file = tmp.join("lcu_cmd_line.json");
                let cmd_str = format!(
                    r#"Start-Process powershell -Verb runAs -ArgumentList "Get-CimInstance Win32_Process -Filter \""name = 'LeagueClientUx.exe'\"" | Select-Object CommandLine | ConvertTo-Json | out-file -encoding utf8 -force {}" -WindowStyle hidden"#,
                    tmp.join("lcu_cmd_line.json").display()
                );

                Command::new("powershell")
                    .args(["/C", &cmd_str])
                    .output()
                    .expect("failed to run powershell");

                let file_content = fs::read_to_string(&output_file).unwrap();
                println!("{}", file_content);
                let cmd_line: CmdOutput = serde_json::from_str(&file_content).unwrap();
                let output = cmd_line.command_line.unwrap_or(String::new());
                if output.chars().count() > 0 {
                    let port: Vec<&str> = output.split("--app-port=").collect();
                    println!("{:?}", port);
                }
            }
        });

        forever.await?;
        Ok(())
    }
}
