use std::io::Error;
use std::process::{Child, Command};

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

fn install_git() {
    println!("\ninstalling tool: git");
    let process = Command::new("apt")
        .arg("install")
        .arg("git-all")
        .spawn();
    
    check_process_status("installed tool: git", process);
}

fn can_find_tool(tool_name: &str) -> bool {
    println!("\nchecking for tool: {}", tool_name);
    let process = Command::new(tool_name).arg("--version").spawn();

    let message = format!("found tool: {}", tool_name);

    check_process_status(&message, process)
}

fn update_system() {
    println!("\nupdating system");
    let process = Command::new("sudo")
        .arg("apt-get")
        .arg("update")
        .spawn();

    check_process_status("system updated", process);
}

fn main() {
    update_system();
    if !can_find_tool("git") {
        install_git();
    }
    can_find_tool("code");
    can_find_tool("google-chrome");
    can_find_tool("firefox");
    can_find_tool("docker");
    can_find_tool("docker-compose");
}
