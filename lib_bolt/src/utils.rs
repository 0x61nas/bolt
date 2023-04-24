use crate::Method;
use crate::Request;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

pub fn extract_headers(map: &reqwest::header::HeaderMap) -> Vec<Vec<String>> {
    let mut headers: Vec<Vec<String>> = Vec::new();

    for (key, value) in map.iter() {
        let mut header: Vec<String> = Vec::new();

        header.push(key.to_string());
        header.push(value.to_str().unwrap().to_string());

        headers.push(header);
    }

    return headers;
}

pub fn get_timestamp() -> u128 {
    return SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
}

pub fn prepare_request(req: Request) -> reqwest::RequestBuilder {
    let client = reqwest::Client::new();

    let builder = match req.method {
        Method::GET => client.get(req.url).body(req.body),
        Method::POST => client.post(req.url).body(req.body),
        Method::PUT => client.put(req.url).body(req.body),
        Method::DELETE => client.delete(req.url).body(req.body),
        Method::HEAD => client.head(req.url).body(req.body),
        Method::PATCH => client.patch(req.url).body(req.body),
        Method::OPTIONS => client
            .request(reqwest::Method::OPTIONS, req.url)
            .body(req.body),
        Method::CONNECT => client
            .request(reqwest::Method::CONNECT, req.url)
            .body(req.body),
    };

    return builder;
}

// downloads the dist from github
pub fn build_dist() {
    println!("Downloading static files");

    #[cfg(debug_assertions)]
    _clone_repo_dev();

    #[cfg(not(debug_assertions))]
    _clone_repo_release();

    let src = get_home() + "bolt/tauri/" + "./dist/";
    let dst = get_home() + "bolt/tauri/" + "../../dist";
    copy_dir(&src, &dst).unwrap();

    println!("Download complete");
}

pub fn _clone_repo_dev() {
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

fn copy_dir(src: &str, dst: &str) -> std::io::Result<()> {
    let src = Path::new(&src);
    let dst = Path::new(&dst);

    if src.is_dir() {
        std::fs::create_dir_all(dst)?;

        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let new_path = dst.join(path.file_name().unwrap());

            if entry.file_type()?.is_dir() {
                copy_dir(path.to_str().unwrap(), new_path.to_str().unwrap())?;
            } else {
                std::fs::copy(&path, &new_path)?;
            }
        }
    } else {
        std::fs::copy(src, dst)?;
    }

    Ok(())
}

pub fn run_command(shell_command: String, dir: String) {
    let _output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &shell_command])
            .current_dir(dir)
            .output()
            .expect(&format!("failed to execute command: {}", &shell_command))
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&shell_command)
            .current_dir(dir)
            .output()
            .expect(&format!("failed to execute command: {}", &shell_command))
    };
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
    let new_state = r#"{"page":"Home","main_current":0,"col_current":[0,0],"main_col":{"name":"New Collection ","requests":[{"url":"","body":"","headers":[["",""]],"params":[["",""]],"method":"GET","response":{"status":0,"body":"","headers":[],"time":0,"size":0,"response_type":"TEXT","request_index":0,"failed":false},"name":"New Request 1","req_tab":1,"resp_tab":1,"loading":false}],"collapsed":false},"collections":[]}"#;

    std::fs::write(path, new_state).unwrap();
}

pub fn open_browser(link: String) {
    std::thread::sleep(std::time::Duration::from_secs(2));

    webbrowser::open(&link).unwrap();
}

pub fn reset_home() {
    println!("reseting dist");

    let home_path = get_home();

    let _reset = match std::fs::remove_dir_all(home_path) {
        Ok(_) => println!("Deleted bolt home"),
        Err(err) => println!("could not delete Bolt home: {}", err),
    };

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
