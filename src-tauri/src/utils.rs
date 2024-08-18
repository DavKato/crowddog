pub fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}
