use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn check_process_status(message: &str, process: Result<Child, Error>) -> bool {
    match process {
        Err(error) => {
            println!("process error message: {}", error);

            false
        }
        Ok(mut child) => {
            let exit_status = child.wait();

            match exit_status {
                Err(error) => {
                    println!("exit status error message: {}", error);

                    false
                }
                Ok(status) => {
                    println!("process status: {}", status);
                    println!("{}", message);

                    true
                }
            }
        }
    }
}

fn install_rustc() {
    println!("\ninstalling tool: rustc");
    let mut curl_process_child = Command::new("curl")
        .arg("--proto")
        .arg("=https")
        .arg("--tlsv1.2")
        .arg("-sSf")
        .arg("https://sh.rustup.rs")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(curl_process) = curl_process_child.stdout.take() {
        let sh_process = Command::new("sh").stdin(curl_process).spawn();

        check_process_status("installed tool: rustc", sh_process);
    }
}

fn install_code() {
    println!("\ninstalling tool: code");
    let process = Command::new("sudo")
        .arg("snap")
        .arg("install")
        .arg("code")
        .arg("--classic")
        .spawn();

    check_process_status("installed tool: code", process);
}

fn install_firefox() {
    println!("\ninstalling tool: firefox");
    let process = Command::new("sudo")
        .arg("snap")
        .arg("install")
        .arg("firefox")
        .spawn();

    check_process_status("installed tool: firefox", process);
}

fn install_docker() {
    println!("\ninstalling tool: docker");
    let process = Command::new("sudo")
        .arg("snap")
        .arg("install")
        .arg("docker")
        .spawn();

    check_process_status("installed tool: docker", process);
}

fn can_find_tool(tool_name: &str) -> bool {
    println!("\nchecking for tool: {}", tool_name);
    let process = Command::new(tool_name).arg("--version").spawn();

    let message = format!("found tool: {}", tool_name);

    check_process_status(&message, process)
}

fn update_system() {
    println!("\nupdating system");
    let process = Command::new("sudo").arg("apt-get").arg("update").spawn();

    check_process_status("system updated", process);
}

fn main() {
    update_system();
    if !can_find_tool("rustc") {
        install_rustc();
    }
    if !can_find_tool("code") {
        install_code();
    }
    if !can_find_tool("firefox") {
        install_firefox();
    }
    if !can_find_tool("docker") {
        install_docker();
    }
    can_find_tool("google-chrome");
    can_find_tool("docker-compose");
}
