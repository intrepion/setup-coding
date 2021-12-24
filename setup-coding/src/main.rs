use std::io::Error;
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
    // sudo apt install curl
    let apt_install_dependencies_process = Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg("curl")
        .spawn();

    check_process_status("installed dependencies", apt_install_dependencies_process);

    // curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs
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
        // | sh
        let sh_process = Command::new("sh").stdin(curl_process).spawn();

        check_process_status("installed tool: rustc", sh_process);
    }
}

fn install_git() {
    println!("\ninstalling tool: git");
    // sudo apt install git-all
    let process = Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg("git-all")
        .spawn();

    check_process_status("installed tool: git", process);
}

fn install_code() {
    println!("\ninstalling tool: code");
    // sudo snap install code --classic
    let process = Command::new("sudo")
        .arg("snap")
        .arg("install")
        .arg("code")
        .arg("--classic")
        .spawn();

    check_process_status("installed tool: code", process);
}

fn install_brave_browser() {
    println!("\ninstalling tool: brave-browser");
    // sudo apt install apt-transport-https curl
    let apt_install_dependencies_process = Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg("apt-transport-https")
        .arg("curl")
        .spawn();

    check_process_status("installed dependencies", apt_install_dependencies_process);

    // sudo curl -fsSLo /usr/share/keyrings/brave-browser-archive-keyring.gpg https://brave-browser-apt-release.s3.brave.com/brave-browser-archive-keyring.gpg
    let apt_curl_process = Command::new("sudo")
        .arg("curl")
        .arg("-fsSLo")
        .arg("/usr/share/keyrings/brave-browser-archive-keyring.gpg")
        .arg("https://brave-browser-apt-release.s3.brave.com/brave-browser-archive-keyring.gpg")
        .spawn();

    check_process_status("downloaded gpg file", apt_curl_process);

    // echo "deb [signed-by=/usr/share/keyrings/brave-browser-archive-keyring.gpg arch=amd64] https://brave-browser-apt-release.s3.brave.com/ stable main"
    let mut echo_process_child = Command::new("echo")
        .arg("deb [signed-by=/usr/share/keyrings/brave-browser-archive-keyring.gpg arch=amd64] https://brave-browser-apt-release.s3.brave.com/ stable main")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(echo_process) = echo_process_child.stdout.take() {
        // |sudo tee /etc/apt/sources.list.d/brave-browser-release.list
        let tee_process = Command::new("sudo")
            .arg("tee")
            .arg("/etc/apt/sources.list.d/brave-browser-release.list")
            .stdin(echo_process)
            .spawn();

        check_process_status("writing to sources file", tee_process);

        // sudo apt update
        let apt_update_process = Command::new("sudo").arg("apt").arg("update").spawn();

        check_process_status("updated apt", apt_update_process);

        // sudo apt install brave-browser
        let apt_install_process = Command::new("sudo")
            .arg("apt")
            .arg("install")
            .arg("brave-browser")
            .spawn();

        check_process_status("installed tool: brave-browser", apt_install_process);
    }
}

fn install_docker() {
    println!("\ninstalling tool: docker");
    // sudo apt-get install ca-certificates curl gnupg lsb-release
    let apt_install_dependencies_process = Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg("ca-certificates")
        .arg("curl")
        .arg("gnupg")
        .arg("lsb-release")
        .spawn();

    check_process_status("installed dependencies", apt_install_dependencies_process);

    // curl -fsSL https://download.docker.com/linux/ubuntu/gpg
    let mut curl_process_child = Command::new("curl")
        .arg("-fsSL")
        .arg("https://download.docker.com/linux/ubuntu/gpg")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(curl_process) = curl_process_child.stdout.take() {
        // | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
        let gpg_process = Command::new("sudo")
            .arg("gpg")
            .arg("--dearmor")
            .arg("-o")
            .arg("/usr/share/keyrings/docker-archive-keyring.gpg")
            .stdin(curl_process)
            .spawn();

        check_process_status("downloaded gpg file", gpg_process);

        // dpkg --print-architecture
        let dpkg_process_child = Command::new("dpkg")
            .arg("--print-architecture")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let dpkg_process_child_stdout = dpkg_process_child.wait_with_output().unwrap();

        let architecture_name = String::from_utf8(dpkg_process_child_stdout.stdout).unwrap();
        
        // lsb_release -cs
        let lsb_release_process_child = Command::new("lsb_release")
            .arg("-cs")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let lsb_release_process_child_stdout = lsb_release_process_child.wait_with_output().unwrap();

        let release_name = String::from_utf8(lsb_release_process_child_stdout.stdout).unwrap();
    
        let echo_argument = format!("deb [arch={} signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu {} stable", architecture_name, release_name);

        // echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
        let mut echo_process_child = Command::new("echo")
            .arg(echo_argument)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(echo_process) = echo_process_child.stdout.take() {
            // | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
            let tee_process = Command::new("sudo")
                .arg("tee")
                .arg("/etc/apt/sources.list.d/docker.list")
                .stdin(echo_process)
                .spawn();

            check_process_status("writing to sources file", tee_process);

            // sudo apt-get install docker-ce docker-ce-cli containerd.io
            let apt_install_process = Command::new("sudo")
                .arg("apt-get")
                .arg("install")
                .arg("docker-ce")
                .arg("docker-ce-cli")
                .arg("containerd.io")
                .spawn();

            check_process_status("installed tool: docker", apt_install_process);
        }
    }
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
    if !can_find_tool("git") {
        install_git();
    }
    if !can_find_tool("code") {
        install_code();
    }
    if !can_find_tool("brave-browser") {
        install_brave_browser();
    }
    if !can_find_tool("docker") {
        install_docker();
    }
}
