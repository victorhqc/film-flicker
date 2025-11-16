pub fn parse_aperture(a: &str) -> Option<f32> {
    if a.starts_with("f/") {
        a.strip_prefix("f/")?.parse().ok()
    } else {
        None
    }
}
