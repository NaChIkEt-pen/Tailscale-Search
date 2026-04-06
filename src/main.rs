use dialoguer::FuzzySelect;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
fn main() {
    let output = Command::new("tailscale")
        .arg("status")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut machine: Vec<&str> = Vec::new();

    for value in stdout.lines() {
        let temp_vec: Vec<&str> = value.split_whitespace().collect();
        machine.push(temp_vec[1]);
    }

    let selection = FuzzySelect::new()
        .with_prompt("Select Machine")
        .items(&machine)
        .interact()
        .unwrap();

    let selected = machine[selection];

    let usernames = ["root", "ubuntu", ""]; // ADD usernames here
    for user in usernames {
        let ssh_target = format!("{}@{}", user, selected);

        let status = Command::new("ssh")
            .args([
                "-o",
                "BatchMode=yes",
                "-o",
                "ConnectTimeout=5",
                "-o",
                "StrictHostKeyChecking=no",
                &ssh_target,
                "exit",
            ])
            .status()
            .expect("Failed to start ssh");

        if status.success() {
            let _res = Command::new("ssh").arg(&ssh_target).exec();
        }
    }
    println!("Can't connect to the machine with any username");
}
