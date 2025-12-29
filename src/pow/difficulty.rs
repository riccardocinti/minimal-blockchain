pub fn is_valid(hash: &str, difficulty: usize) -> bool {
    hash.starts_with(&"0".repeat(difficulty))
}
