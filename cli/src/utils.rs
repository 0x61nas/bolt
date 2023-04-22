// use crate::*;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

// downloads the dist's source files from the official github repository and builds it
pub fn build_dist() {
    println!("Downloading static files");

    #[cfg(debug_assertions)]
    _clone_repo_debug();

    #[cfg(not(debug_assertions))]
    _clone_repo_release();

    let shell_command = format!("cp -r ./dist/ ../../dist");
    run_command(shell_command, get_home() + "bolt/cli/");

    let shell_command = format!("cp ./bolt/icon/* ./dist/");
    run_command(shell_command, get_home());

    println!("Download complete");
}

pub fn _clone_repo_debug() {
    let shell_command = format!(
        "rsync -a --exclude-from=.gitignore --exclude='.git' ./ {}",
        get_home() + "bolt/"
    );
    run_command(shell_command, "../".to_string());
}

pub fn _clone_repo_release() {
    let url = "https://github.com/hiro-codes/bolt";

    let shell_command = format!("git clone {url} --depth 1");

    run_command(shell_command, get_home());
}

pub fn run_command(shell_command: String, dir: String) {
    let mut child = Command::new("sh")
        .current_dir(dir)
        .arg("-c")
        .arg(shell_command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let stdout = BufReader::new(child.stdout.take().unwrap());
    let stderr = BufReader::new(child.stderr.take().unwrap());

    let stdout_lines = stdout.lines();
    let stderr_lines = stderr.lines();

    let mut output = io::LineWriter::new(io::stdout());

    for line in stdout_lines.chain(stderr_lines) {
        if let Ok(line) = line {
            writeln!(output, "{}", line).expect("Failed to write to stdout");
            io::stdout().flush().expect("Failed to flush stdout");
        } else {
            break;
        }
    }

    let status = child.wait().expect("Failed to wait for command");
    if !status.success() {
        eprintln!("Command failed with status: {}", status);
    }
}

pub fn verify_state() {
    let path = get_home() + "state.json";
    if !file_exists(&path) {
        println!("Creating state file");

        create_state(&path);
    }
}

pub fn file_exists(path: &String) -> bool {
    if Path::new(&path).exists() {
        true
    } else {
        println!("File {} does not exist", path);
        false
    }
}

pub fn create_state(path: &String) {
    let new_state = r#"
{"page":"Home","main_current":0,"col_current":[0,0],"main_col":{"name":"New Collection ","requests":[{"url":"","body":"","headers":[["",""]],"params":[["",""]],"method":"GET","response":{"status":0,"body":"","headers":[],"time":0,"size":0,"response_type":"TEXT","request_index":0,"failed":false},"name":"New Request ","req_tab":1,"resp_tab":1}],"collapsed":false},"collections":[]}
"#;

    std::fs::write(path, new_state).unwrap();
}

pub fn open_browser(link: String) {
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("opening browser");

    webbrowser::open(&link).unwrap();
}

pub fn reset_home() {
    println!("reseting dist");

    let home_path = get_home();

    std::fs::remove_dir_all(home_path).unwrap(); // deletes the local copy

    verify_home();
    verify_dist();

    println!("Dist has been reset");
}

pub fn get_dist() -> String {
    get_home() + "dist/"
}

pub fn get_home() -> String {
    let path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/bolt/";
    path
}

pub fn verify_home() {
    let path = get_home();
    if !dir_exists(&path) {
        println!("Creating directory {}", path);
        create_home(&path);
    }
}

pub fn verify_dist() {
    let path = get_dist();
    if !dir_exists(&path) {
        println!("Creating dist");
        build_dist();
    }
}

pub fn dir_exists(path: &String) -> bool {
    if Path::new(&path).exists() {
        true
    } else {
        println!("Directory {} does not exist", path);
        false
    }
}

pub fn create_home(path: &String) {
    std::fs::create_dir(path).unwrap();
}
