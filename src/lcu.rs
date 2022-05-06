use anyhow::Result;
use regex::Regex;
use std::{
    env,
    fs::{self, File},
    process::Command,
    sync::mpsc,
    time::Duration,
};
use tokio::{task, time};
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

    pub async fn start_parse_auth_task(&mut self) -> Result<()> {
        let (tx, rx) = mpsc::channel();

        let forever = task::spawn(async move {
            let tx = tx.clone();
            let mut interval = time::interval(Duration::from_secs(5));

            loop {
                interval.tick().await;

                // match self.parse_auth().await {
                //     Ok(s) => {
                //         println!("auth url is {}", s);
                //     }
                //     Err(why) => {
                //         println!("parse lcu auth failed, {:?}", why);
                //     }
                // };
                tx.send(()).unwrap();
            }
        });

        forever.await?;

        // drop(tx);
        while let Ok(_) = rx.recv() {
            println!("loop");
        }

        Ok(())
    }
}

pub async fn parse_auth() -> Result<String> {
    lazy_static! {
        static ref PORT_REGEXP: Regex = Regex::new(r"--app-port=\d+").unwrap();
        static ref TOKEN_REGEXP: Regex = Regex::new(r"--remoting-auth-token=\w+").unwrap();
    }

    let (tx, rx) = mpsc::channel();
    let job = task::spawn(async move {
        let tx = tx.clone();
        let output_file_path = env::temp_dir().join("champr_lcu.tmp");
        let display = output_file_path.display();
        if !output_file_path.exists() {
            match File::create(&output_file_path) {
                Ok(_) => (),
                Err(why) => {
                    println!("couldn't create {}: {}", display, why);
                }
            };
        }
        let cmd_str = format!(
            r#"Start-Process powershell -WindowStyle hidden -Verb runAs -ArgumentList "-noprofile (Get-CimInstance Win32_Process -Filter \""name = 'LeagueClientUx.exe'\"").CommandLine | out-file -encoding utf8 -force {}""#,
            display
        );

        Command::new("powershell")
            .args(["/C", &cmd_str])
            .output()
            .expect("failed to run powershell");

        let file_content = fs::read_to_string(&output_file_path).unwrap_or_default();
        match PORT_REGEXP.is_match(&file_content) {
            false => (),
            true => {
                let port_match = PORT_REGEXP.find(&file_content).unwrap();
                let port = port_match.as_str().replace(APP_PORT_KEY, "");
                let token_match = TOKEN_REGEXP.find(&file_content).unwrap();
                let token = token_match
                    .as_str()
                    .replace(AUTH_TOKEN_KEY, "")
                    .replace(CONTROL_CHAR, "");

                let auth_url = make_auth_url(port, token);
                tx.send(auth_url).unwrap();
            }
        }
    });
    job.await?;

    let auth_url = match rx.recv() {
        Ok(url) => url,
        Err(_) => String::from(""),
    };
    Ok(auth_url.clone())
}

mod tests {
    #![allow(unused_imports)]
    use crate::lcu;

    #[tokio::test]
    async fn get_auth() {
        // let mut client = LCU::new();
        match lcu::parse_auth().await {
            Ok(url) => print!("auth url: {}\n", url),
            Err(why) => panic!("{:?}", why),
        };
    }
}
