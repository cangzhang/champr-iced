use anyhow::Result;
use regex::Regex;
use std::{env, fs, process::Command, time::Duration};
use tokio::{task, time, sync::mpsc};
// use serde::{Deserialize, Serialize};

pub struct LCU {
    auth_url: String,
}

const APP_PORT_KEY: &str = "--app-port=";
const AUTH_TOKEN_KEY: &str = "--remoting-auth-token=";
const CONTROL_CHAR: &str = "\\";

pub fn make_auth_url(token: String, port: String) -> String {
    format!("riot:{token}@127.0.0.1:{port}")
}

impl LCU {
    pub fn new() -> Self {
        Self {
            auth_url: String::new(),
        }
    }

    pub async fn parse_auth(&mut self) -> Result<()> {
        lazy_static! {
            static ref PORT_REGEXP: Regex = Regex::new(r"--app-port=\d+").unwrap();
            static ref TOKEN_REGEXP: Regex = Regex::new(r"--remoting-auth-token=\S+\\").unwrap();
        }

        let (tx, mut rx) = mpsc::channel(100);
        let forever = task::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(5));

            loop {
                interval.tick().await;
                
                let tx = tx.clone();
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

                let port_match = PORT_REGEXP.find(&file_content).unwrap();
                let port = port_match.as_str().replace(APP_PORT_KEY, "");
                let token_match = TOKEN_REGEXP.find(&file_content).unwrap();
                let token = token_match
                    .as_str()
                    .replace(AUTH_TOKEN_KEY, "")
                    .replace(CONTROL_CHAR, "");

                let auth_url = make_auth_url(port, token);
                print!("auth url: {}", auth_url);
                if let Err(_) = tx.send(auth_url).await {
                    println!("tx failed");
                };
            }
        });

        forever.await?;
        
        while let Some(r) = rx.recv().await {
            println!("lcu auth url: {}", r);
            self.auth_url = r.to_owned();
        }

        Ok(())
    }
}
