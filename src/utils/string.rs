pub fn substring_from_index(s: &str, start_index: usize) -> &str {
    let sub = s.get(start_index..);
    sub.unwrap_or("")
}

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}