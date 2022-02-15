use std::{process::Command, time::Duration};

use anyhow::Result;
use tokio::{task, time};

pub async fn get_lcu_auth() -> Result<()> {
    let forever = task::spawn(async {
        let mut interval = time::interval(Duration::from_secs(10));

        loop {
            interval.tick().await;

            let cmd_str = r#"
            $tmp = new-item -path $env:temp -name "lcu_cmd_line.json" -force
            Start-Process powershell -Verb runAs -ArgumentList "Get-CimInstance Win32_Process -Filter \""name = 'LeagueClientUx.exe'\"" | Select-Object CommandLine | ConvertTo-Json | out-file $tmp" -WindowStyle hidden
            get-content -path $tmp
            "#;

            println!("{}", cmd_str);
            let output = Command::new("powershell")
                .args(["/C", &cmd_str])
                .output()
                .expect("failed to run powershell");

            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
    });

    forever.await?;
    Ok(())
}
