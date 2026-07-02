pub fn make(text: &str) -> String {
    text.to_lowercase().replace(' ', "-")
}
