use std::process::Command;

fn check_for_tool(name: &str) {
    let spawn = Command::new(name)
        .arg("--version")
        .spawn();

    match spawn {
        Err(error) => {
            println!("Error message: {}", error)
        },
        Ok(mut child) => {
            let exit_status = child.wait();

            match exit_status {
                Err(error) => {
                    println!("Error message: {}", error)
                },
                Ok(status) => {
                    println!("{}", status);
                }
            }
        },
    }    
}

fn main() {
    check_for_tool("git");
    check_for_tool("code");
    check_for_tool("google-chrome");
    check_for_tool("firefox");
    check_for_tool("docker");
    check_for_tool("docker-compose")
}
