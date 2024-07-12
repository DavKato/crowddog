use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, io};

fn get_file_path(app: &tauri::AppHandle) -> PathBuf {
    let dir = app.path_resolver().app_config_dir().unwrap();
    dir.join("settings.json")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    pub email: String,
    pub passwd: String,
}
impl Auth {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            passwd: String::new(),
        }
    }

    pub fn init(app: &tauri::AppHandle) -> Auth {
        let file_path = get_file_path(app);

        match restore_from_file(&file_path) {
            Ok(auth) => auth,
            Err(e) => {
                println!("{}, path: {:#?}", e, file_path);
                Self::new()
            }
        }
    }

    pub fn replace(&mut self, new_settings: Auth) {
        self.email = new_settings.email;
        self.passwd = new_settings.passwd;
    }

    pub fn save(&self, app: &tauri::AppHandle) {
        let file_path = get_file_path(app);
        write_file(&file_path, self);
    }

    pub fn is_valid(&self) -> bool {
        !self.email.is_empty() && !self.passwd.is_empty()
    }
}

fn write_file(path: &PathBuf, data: &Auth) {
    let dir = Path::new(&path).parent().unwrap();
    if !dir.exists() {
        fs::create_dir_all(dir).unwrap();
    }
    let data = serde_json::to_string(data).expect("Unable to serialize JSON");
    fs::write(path, data).expect("Unable to write file");
}

fn restore_from_file(path: &PathBuf) -> Result<Auth, io::Error> {
    let json = fs::read_to_string(path)?;
    let auth: Auth = serde_json::from_str(&json)?;
    Ok(auth)
}
