use std::{env, fs, process::Command, time::Duration};

use anyhow::Result;
use tokio::{task, time};

pub async fn get_lcu_auth() -> Result<()> {
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

            let output = Command::new("powershell")
                .args(["/C", &cmd_str])
                .output()
                .expect("failed to run powershell");

            let file_content = fs::read_to_string(&output_file).unwrap();
            println!("{}", file_content);
        }
    });

    forever.await?;
    Ok(())
}
