pub fn parse_mm(s: &str) -> Option<f32> {
    if s.ends_with("mm") {
        s.strip_suffix("mm")?.parse().ok()
    } else {
        None
    }
}
