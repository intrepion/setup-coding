use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::io::Error;
use std::process;
use std::process::{Child, Command, Output, Stdio};
use std::string::FromUtf8Error;

#[derive(Debug, Deserialize)]
struct DockerCompose {
    version: String,
}

#[derive(Debug, Deserialize)]
struct TargetEnvironment {
    keys: Option<Keys>,
    tools: Option<Tools>,
    updates: Option<Updates>,
}

#[derive(Debug, Deserialize)]
struct Keys {
    ssh: Option<Ssh>,
}

#[derive(Debug, Deserialize)]
struct Node {
    version: String,
}

#[derive(Debug, Deserialize)]
struct Ssh {
    algorithm: String,
    email: String,
    title: String,
}

#[derive(Debug, Deserialize)]
struct Solana {
    version: String,
}

#[derive(Debug, Deserialize)]
struct Tools {
    brave_browser: Option<String>,
    code: Option<String>,
    docker: Option<String>,
    docker_compose: Option<DockerCompose>,
    gh: Option<String>,
    git: Option<String>,
    google_chrome: Option<String>,
    node: Option<Node>,
    rustc: Option<String>,
    solana: Option<Solana>,
}

#[derive(Debug, Deserialize)]
struct Updates {
    cleanup: Option<bool>,
    dependencies: Option<bool>,
    system: Option<bool>,
}

fn can_find_tool(tool_name: &str) -> bool {
    println!("\nchecking for tool: {}", tool_name);
    let process = Command::new(tool_name).arg("--version").spawn();

    let message = format!("found tool: {}", tool_name);

    check_process_status(&message, process)
}

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

fn convert_output_to_string(process_child_stdout: Output) -> Result<String, FromUtf8Error> {
    let architecture_name_result = String::from_utf8(process_child_stdout.stdout);

    match architecture_name_result {
        Err(error) => {
            println!(
                "error when trying to convert string from dpkg output: {}",
                error
            );

            Err(error)
        }
        Ok(architecture_name) => Ok(architecture_name.trim().to_string()),
    }
}

fn generate_new_ssh_key(algorithm: &str, email: &str, title: &str) {
    println!("\ngenerating new ssh key");

    // ssh-keygen -t ed25519 -C "your_email@example.com"
    let ssh_keygen_process = Command::new("ssh-keygen")
        .arg("-t")
        .arg(&algorithm)
        .arg("-C")
        .arg(email)
        .spawn();

    check_process_status("generated ssh key", ssh_keygen_process);

    // eval "$(ssh-agent -s)"
    let eval_process = Command::new("eval").arg("$(ssh-agent -s)").spawn();

    check_process_status("started the ssh agent", eval_process);

    let ssh_directory = format!("~/.ssh/{}", algorithm);

    // sh-add ~/.ssh/id_ed25519
    let ssh_add_process = Command::new("ssh-add").arg(&ssh_directory).spawn();

    check_process_status("added to the ssh agent", ssh_add_process);

    // gh ssh-key add ~/.ssh/id_ed25519.pub --title "personal laptop"
    let gh_process = Command::new("gh")
        .arg("ssh-key")
        .arg("add")
        .arg(ssh_directory)
        .arg("--title")
        .arg(title)
        .spawn();

    check_process_status("added ssh key to github", gh_process);
}

fn get_architecture_name_output() -> Result<Output, Error> {
    println!("\ngetting architecture name");

    // dpkg --print-architecture
    let dpkg_process_child_result = Command::new("dpkg")
        .arg("--print-architecture")
        .stdout(Stdio::piped())
        .spawn();

    match dpkg_process_child_result {
        Err(error) => {
            println!("error when trying to dpkg: {}", error);

            Err(error)
        }
        Ok(dpkg_process_child) => {
            let dpkg_process_child_stdout_result = dpkg_process_child.wait_with_output();

            match dpkg_process_child_stdout_result {
                Err(error) => {
                    println!("error when trying to dpkg: {}", error);

                    Err(error)
                }
                Ok(dpkg_process_child_stdout) => Ok(dpkg_process_child_stdout),
            }
        }
    }
}

fn get_kernel_name_output() -> Result<Output, Error> {
    println!("\ngetting kernel name");

    // uname -s
    let uname_process_child_result = Command::new("uname")
        .arg("-s")
        .stdout(Stdio::piped())
        .spawn();

    match uname_process_child_result {
        Err(error) => {
            println!("error when trying to uname: {}", error);

            Err(error)
        }
        Ok(uname_process_child) => {
            let uname_process_child_stdout_result = uname_process_child.wait_with_output();

            match uname_process_child_stdout_result {
                Err(error) => {
                    println!("error when trying to get output of uname: {}", error);

                    Err(error)
                }
                Ok(uname_process_child_stdout_result) => Ok(uname_process_child_stdout_result),
            }
        }
    }
}

fn get_machine_hardware_name_output() -> Result<Output, Error> {
    println!("\ngetting machine hardware name");

    // uname -s
    let uname_process_child_result = Command::new("uname")
        .arg("-m")
        .stdout(Stdio::piped())
        .spawn();

    match uname_process_child_result {
        Err(error) => {
            println!("error when trying to uname: {}", error);

            Err(error)
        }
        Ok(uname_process_child) => {
            let uname_process_child_stdout_result = uname_process_child.wait_with_output();

            match uname_process_child_stdout_result {
                Err(error) => {
                    println!("error when trying to get output of uname: {}", error);

                    Err(error)
                }
                Ok(uname_process_child_stdout_result) => Ok(uname_process_child_stdout_result),
            }
        }
    }
}

fn get_release_name_output() -> Result<Output, Error> {
    println!("\ngetting release name");

    // lsb_release -cs
    let lsb_release_process_child_result = Command::new("lsb_release")
        .arg("-cs")
        .stdout(Stdio::piped())
        .spawn();

    match lsb_release_process_child_result {
        Err(error) => {
            println!("error when trying to lsb_release: {}", error);

            Err(error)
        }
        Ok(lsb_release_process_child) => {
            let lsb_release_process_child_stdout_result =
                lsb_release_process_child.wait_with_output();

            match lsb_release_process_child_stdout_result {
                Err(error) => {
                    println!("error when trying to get output of lsb_release: {}", error);

                    Err(error)
                }
                Ok(lsb_release_process_child_stdout) => Ok(lsb_release_process_child_stdout),
            }
        }
    }
}

fn install_brave_browser() {
    println!("\ninstalling tool: brave-browser");

    // sudo curl -fsSLo /usr/share/keyrings/brave-browser-archive-keyring.gpg https://brave-browser-apt-release.s3.brave.com/brave-browser-archive-keyring.gpg
    let apt_curl_process = Command::new("sudo")
        .arg("curl")
        .arg("-fsSLo")
        .arg("/usr/share/keyrings/brave-browser-archive-keyring.gpg")
        .arg("https://brave-browser-apt-release.s3.brave.com/brave-browser-archive-keyring.gpg")
        .spawn();

    check_process_status("downloaded gpg file", apt_curl_process);

    // echo "deb [signed-by=/usr/share/keyrings/brave-browser-archive-keyring.gpg arch=amd64] https://brave-browser-apt-release.s3.brave.com/ stable main"
    let echo_process_child_result = Command::new("echo")
        .arg("deb [signed-by=/usr/share/keyrings/brave-browser-archive-keyring.gpg arch=amd64] https://brave-browser-apt-release.s3.brave.com/ stable main")
        .stdout(Stdio::piped())
        .spawn();

    match echo_process_child_result {
        Err(error) => {
            println!("could not install brave-browser: {}", error);
        }
        Ok(mut echo_process_child) => {
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
    }
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

fn install_docker(architecture_name: &str, release_name: &str) {
    println!("\ninstalling tool: docker");

    // curl -fsSL https://download.docker.com/linux/ubuntu/gpg
    let curl_process_child_result = Command::new("curl")
        .arg("-fsSL")
        .arg("https://download.docker.com/linux/ubuntu/gpg")
        .stdout(Stdio::piped())
        .spawn();

    match curl_process_child_result {
        Err(error) => {
            println!("error when trying to download: {}", error);
        }
        Ok(mut curl_process_child) => {
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

                let echo_argument = format!("deb [arch={} signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu {} stable", architecture_name, release_name);

                // echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
                let echo_process_child_result = Command::new("echo")
                    .arg(echo_argument)
                    .stdout(Stdio::piped())
                    .spawn();

                match echo_process_child_result {
                    Err(error) => {
                        println!("error when processing echo: {}", error);
                    }
                    Ok(mut echo_process_child) => {
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
            }
        }
    }
}

fn install_docker_compose(_version: &str, kernel_name: &str, machine_hardware_name: &str) {
    println!("\ninstalling tool: docker-compose");

    let github_url = format!(
        "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-{}-{}",
        kernel_name, machine_hardware_name
    );

    // sudo curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
    let curl_process = Command::new("sudo")
        .arg("curl")
        .arg("-L")
        .arg(github_url)
        .arg("-o")
        .arg("/usr/local/bin/docker-compose")
        .spawn();

    check_process_status("downloaded docker-compose from github", curl_process);

    // sudo chmod +x /usr/local/bin/docker-compose
    let chmod_process = Command::new("sudo")
        .arg("chmod")
        .arg("+x")
        .arg("/usr/local/bin/docker-compose")
        .spawn();
    check_process_status("chmoded downloaded file", chmod_process);

    // sudo ln -s /usr/local/bin/docker-compose /usr/bin/docker-compose
    let ln_process = Command::new("sudo")
        .arg("ln")
        .arg("-s")
        .arg("/usr/local/bin/docker-compose")
        .arg("/usr/bin/docker-compose")
        .spawn();

    check_process_status("installed tool: docker-compose", ln_process);
}

fn install_gh(architecture_name: &str) {
    println!("\ninstalling tool: gh");

    // curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg
    let curl_process_child_result = Command::new("curl")
        .arg("-fsSL")
        .arg("https://cli.github.com/packages/githubcli-archive-keyring.gpg")
        .stdout(Stdio::piped())
        .spawn();

    match curl_process_child_result {
        Err(error) => {
            println!("error when trying to curl: {}", error);
        }
        Ok(mut curl_process_child) => {
            if let Some(curl_process) = curl_process_child.stdout.take() {
                // | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
                let dd_process = Command::new("sudo")
                    .arg("dd")
                    .arg("of=/usr/share/keyrings/githubcli-archive-keyring.gpg")
                    .stdin(curl_process)
                    .spawn();

                check_process_status("downloading gpg file", dd_process);

                let echo_argument = format!("deb [arch={} signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main", architecture_name);

                // echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main"
                let echo_process_child_result = Command::new("echo")
                    .arg(echo_argument)
                    .stdout(Stdio::piped())
                    .spawn();

                match echo_process_child_result {
                    Err(error) => {
                        println!("error when trying to echo: {}", error);
                    }
                    Ok(mut echo_process_child) => {
                        if let Some(echo_process) = echo_process_child.stdout.take() {
                            // | sudo tee /etc/apt/sources.list.d/github-cli.list
                            let tee_process = Command::new("sudo")
                                .arg("tee")
                                .arg("/etc/apt/sources.list.d/github-cli.list")
                                .stdin(echo_process)
                                .spawn();

                            check_process_status("creating repository source file", tee_process);

                            // sudo apt update
                            let apt_update_process =
                                Command::new("sudo").arg("apt").arg("update").spawn();

                            check_process_status("updating system", apt_update_process);

                            // sudo apt install gh
                            let apt_install_process = Command::new("sudo")
                                .arg("apt")
                                .arg("install")
                                .arg("gh")
                                .spawn();

                            check_process_status("installed tool: gh", apt_install_process);

                            // gh auth login
                            let gh_process = Command::new("gh").arg("auth").arg("login").spawn();

                            check_process_status("logged into gh", gh_process);
                        }
                    }
                }
            }
        }
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

fn install_google_chrome() {
    println!("\ninstalling tool: google-chrome");

    // wget -q -O - https://dl.google.com/linux/linux_signing_key.pub
    let wget_process_child_result = Command::new("wget")
        .arg("-q")
        .arg("-O")
        .arg("-")
        .arg("https://dl.google.com/linux/linux_signing_key.pub")
        .stdout(Stdio::piped())
        .spawn();

    match wget_process_child_result {
        Err(error) => {
            println!("error when trying to download: {}", error);
        }
        Ok(mut wget_process_child) => {
            if let Some(wget_process) = wget_process_child.stdout.take() {
                // | sudo apt-key add -
                let apt_key_process = Command::new("sudo")
                    .arg("apt-key")
                    .arg("add")
                    .arg("-")
                    .stdin(wget_process)
                    .spawn();

                check_process_status("downloaded linux signing key", apt_key_process);

                // echo "deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main"
                let echo_process_child_result = Command::new("echo")
                    .arg("deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main")
                    .stdout(Stdio::piped())
                    .spawn();

                match echo_process_child_result {
                    Err(error) => {
                        println!("error when trying to echo: {}", error);
                    }
                    Ok(mut echo_process_child) => {
                        if let Some(echo_process) = echo_process_child.stdout.take() {
                            // | sudo tee /etc/apt/sources.list.d/google-chrome.list
                            let tee_process = Command::new("sudo")
                                .arg("tee")
                                .arg("/etc/apt/sources.list.d/google-chrome.list")
                                .stdin(echo_process)
                                .spawn();

                            check_process_status("creating repository source file", tee_process);

                            // sudo apt update
                            let apt_install_process =
                                Command::new("sudo").arg("apt").arg("update").spawn();

                            check_process_status("updated apt", apt_install_process);

                            // sudo apt install google-chrome-stable
                            let apt_install_process = Command::new("sudo")
                                .arg("apt")
                                .arg("install")
                                .arg("google-chrome-stable")
                                .spawn();

                            check_process_status(
                                "installed tool: google-chrome",
                                apt_install_process,
                            );
                        }
                    }
                }
            }
        }
    }
}

fn install_node(version: &str) {
    println!("\ninstalling tool: node");

    let package_url = format!("https://deb.nodesource.com/setup_{}.x", version);

    // curl -fsSL https://deb.nodesource.com/setup_17.x
    let curl_process_child_result = Command::new("curl")
        .arg("-fsSL")
        .arg(&package_url)
        .stdout(Stdio::piped())
        .spawn();

    match curl_process_child_result {
        Err(error) => {
            println!("error trying to curl: {}", error);
        }
        Ok(mut curl_process_child) => {
            if let Some(curl_process) = curl_process_child.stdout.take() {
                // | sudo -E bash -
                let bash_process = Command::new("sudo")
                    .arg("-E")
                    .arg("bash")
                    .arg("-")
                    .stdin(curl_process)
                    .spawn();

                check_process_status("setup node in repository", bash_process);

                // sudo apt-get install -y nodejs
                let apt_get_process = Command::new("sudo")
                    .arg("apt-get")
                    .arg("install")
                    .arg("-y")
                    .arg("nodejs")
                    .spawn();

                check_process_status("installed tool: node", apt_get_process);
            }
        }
    }
}

fn install_rustc() {
    println!("\ninstalling tool: rustc");

    // curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs
    let curl_process_child_result = Command::new("curl")
        .arg("--proto")
        .arg("=https")
        .arg("--tlsv1.2")
        .arg("-sSf")
        .arg("https://sh.rustup.rs")
        .stdout(Stdio::piped())
        .spawn();

    match curl_process_child_result {
        Err(error) => {
            println!("error trying to curl: {}", error);
        }
        Ok(mut curl_process_child) => {
            if let Some(curl_process) = curl_process_child.stdout.take() {
                // | sh
                let sh_process = Command::new("sh").stdin(curl_process).spawn();

                check_process_status("installed tool: rustc", sh_process);
            }
        }
    }
}

fn install_solana(version: &str) {
    println!("\ninstalling tool: solana");

    let curl_url = format!("$(curl -sSfL https://release.solana.com/v{version}/install)");

    // sh -c "$(curl -sSfL https://release.solana.com/v1.9.7/install)"
    let sh_process = Command::new("sh")
        .arg("-c")
        .arg(&curl_url)
        .spawn();
    
    check_process_status("installed tool: solana", sh_process);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_has_two_values = args.len() == 2;

    if !(args_has_two_values) {
        println!("\nusage: ./setup-coding <filename>");

        process::exit(1);
    }

    let filename = &args[1];

    println!("\nreading file: {}", filename);

    let contents = fs::read_to_string(filename).expect("couldn't read file");

    let target_environment: TargetEnvironment =
        toml::from_str(&contents).expect("error trying to convert toml file to string");

    target_updates(target_environment.updates);

    let architecture_name_output =
        get_architecture_name_output().expect("architecture name process output error");

    let architecture_name = convert_output_to_string(architecture_name_output)
        .expect("architecture name conversion error");

    let release_name_output = get_release_name_output().expect("release name process output error");

    let release_name =
        convert_output_to_string(release_name_output).expect("release name conversion error");

    let kernel_name_output = get_kernel_name_output().expect("kernel name process output error");

    let kernel_name =
        convert_output_to_string(kernel_name_output).expect("kernel name conversion error");

    let machine_hardware_name_output =
        get_machine_hardware_name_output().expect("machine hardware name process output error");

    let machine_hardware_name = convert_output_to_string(machine_hardware_name_output)
        .expect("machine hardware name conversion error");

    target_tools(
        target_environment.tools,
        &architecture_name,
        &release_name,
        &kernel_name,
        &machine_hardware_name,
    );
    target_keys(target_environment.keys);
}

fn target_keys(keys: Option<Keys>) {
    println!("\nchecking targets for keys");
    match keys {
        None => {
            println!("can't find keys in targets");
        }
        Some(keys) => match keys.ssh {
            None => {}
            Some(ssh) => {
                generate_new_ssh_key(&ssh.algorithm, &ssh.email, &ssh.title);
            }
        },
    }
}

fn target_tools(
    tools: Option<Tools>,
    architecture_name: &str,
    release_name: &str,
    kernel_name: &str,
    machine_hardware_name: &str,
) {
    println!("\nchecking targets for tools");
    match tools {
        None => {
            println!("can't find tools in targets");
        }
        Some(tools) => {
            match tools.brave_browser {
                None => {}
                Some(_brave_browser) => {
                    if !can_find_tool("brave-browser") {
                        install_brave_browser();
                    }
                }
            }
            match tools.code {
                None => {}
                Some(_code) => {
                    if !can_find_tool("code") {
                        install_code();
                    }
                }
            }
            match tools.docker {
                None => {}
                Some(_docker) => {
                    if !can_find_tool("docker") {
                        install_docker(&architecture_name, &release_name);
                    }
                }
            }
            match tools.docker_compose {
                None => {}
                Some(docker_compose) => {
                    if !can_find_tool("docker-compose") {
                        install_docker_compose(
                            &docker_compose.version,
                            &kernel_name,
                            &machine_hardware_name,
                        );
                    }
                }
            }
            match tools.gh {
                None => {}
                Some(_gh) => {
                    if !can_find_tool("gh") {
                        install_gh(&architecture_name);
                    }
                }
            }
            match tools.git {
                None => {}
                Some(_git) => {
                    if !can_find_tool("git") {
                        install_git();
                    }
                }
            }
            match tools.google_chrome {
                None => {}
                Some(_google_chrome) => {
                    if !can_find_tool("google-chrome") {
                        install_google_chrome();
                    }
                }
            }
            match tools.node {
                None => {}
                Some(node) => {
                    if !can_find_tool("node") {
                        install_node(&node.version);
                    }
                }
            }
            match tools.rustc {
                None => {}
                Some(_rustc) => {
                    if !can_find_tool("rustc") {
                        install_rustc();
                    }
                }
            }
            match tools.solana {
                None => {}
                Some(solana) => {
                    if !can_find_tool("solana") {
                        install_solana(&solana.version);
                    }
                }
            }
        }
    }
}

fn target_updates(updates: Option<Updates>) {
    println!("\nchecking targets for updates");
    match updates {
        None => {
            println!("can't find updates in targets");
        }
        Some(updates) => {
            match updates.system {
                None => {}
                Some(system) => {
                    if system {
                        update_system();
                    }
                }
            }
            match updates.dependencies {
                None => {}
                Some(dependencies) => {
                    if dependencies {
                        update_dependencies();
                    }
                }
            }
            match updates.cleanup {
                None => {}
                Some(cleanup) => {
                    if cleanup {
                        update_cleanup();
                    }
                }
            }
        }
    }
}

fn update_system() {
    println!("\nupdating system");
    let process = Command::new("sudo").arg("apt-get").arg("update").spawn();

    check_process_status("system updated", process);
}

fn update_dependencies() {
    // sudo apt install apt-transport-https build-essential ca-certificates curl gnupg lsb-release
    let apt_install_dependencies_process = Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg("apt-transport-https")
        .arg("build-essential")
        .arg("ca-certificates")
        .arg("clang")
        .arg("cmake")
        .arg("curl")
        .arg("gnupg")
        .arg("libappindicator3-dev")
        .arg("libasound2-dev")
        .arg("libgl1-mesa-dev")
        .arg("libgtk-3-dev")
        .arg("libpq-dev")
        .arg("libssl-dev")
        .arg("libudev-dev")
        .arg("libvulkan-dev")
        .arg("libwayland-dev")
        .arg("libwebkit2gtk-4.0-dev")
        .arg("libxi-dev")
        .arg("libx11-dev")
        .arg("libxkbcommon-dev")
        .arg("lld")
        .arg("lsb-release")
        .arg("musl-tools")
        .arg("pkg-config")
        .arg("postgresql-client")
        .arg("python")
        .spawn();

    check_process_status("installed dependencies", apt_install_dependencies_process);
}

fn update_cleanup() {
    // sudo apt install apt-transport-https build-essential ca-certificates curl gnupg lsb-release
    let apt_autoremove_process = Command::new("sudo").arg("apt").arg("autoremove").spawn();

    check_process_status("system cleaned up", apt_autoremove_process);
}
