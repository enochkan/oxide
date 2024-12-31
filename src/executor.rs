use tokio::process::Command;

pub async fn execute_task(command: &str) -> Result<String, String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .await;

    match output {
        Ok(output) => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
        Err(err) => Err(err.to_string()),
    }
}
