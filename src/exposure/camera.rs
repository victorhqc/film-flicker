#[derive(Debug)]
pub struct Camera {
    pub name: String,
    pub maker: String,
    pub format: Option<String>,
}

impl Camera {
    pub fn new(name: &str, maker: &str, format: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            maker: maker.to_string(),
            format: format.map(|f| f.to_string()),
        }
    }
}
