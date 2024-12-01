pub fn read_file(path: String) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}
