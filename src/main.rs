use std::env;
use std::path::PathBuf;
use std::sync::OnceLock;


slint::include_modules!();
static mut WINDOW: OnceLock<AppWindow> = OnceLock::new();
static GAME_PATH: fn() -> String = move || {
    "\\AlbinoClient".to_string()
};


fn main() {
    unsafe {
        WINDOW = OnceLock::from(AppWindow::new().unwrap());
    }
    let win = unsafe { extract_win() };

    win.on_play_click(on_play);
    win.run().unwrap()
}

fn launch_game() {}

unsafe fn extract_win<'a>() -> &'a AppWindow {
    WINDOW.get().unwrap()
}

fn on_play() {
    unsafe {
        let win = extract_win();
        let path = game_path().join(win.get_version().as_str());
        if path.exists() {
            for path in path.read_dir().unwrap() {
                println!("{:?}", path.unwrap().file_name());
            }
            println!("Path exists");
        } else {
            println!("Path not found")
        }
    }
}


fn game_path() -> PathBuf {
    let os = env::consts::OS;
    let mut parent = String::from("");
    match os {
        "linux" => {
            parent = env::var("ROOT").unwrap()
        }
        "windows" => parent = env::var("APPDATA").unwrap(),
        _ => {
            println!("os: {}", os)
        }
    }
    parent.push_str(&GAME_PATH());
    PathBuf::from(parent)
}
