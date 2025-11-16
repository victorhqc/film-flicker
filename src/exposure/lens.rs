#[derive(Debug)]
pub struct Lens {
    pub focal_length: f32,
    pub name: Option<String>,
    pub maker: Option<String>,
    pub kind: Option<LensKind>,
}

#[derive(Debug)]
pub enum LensKind {
    Prime,
    Zoom,
}

impl Lens {
    pub fn new(
        focal_length: f32,
        name: Option<&str>,
        maker: Option<&str>,
        kind: Option<LensKind>,
    ) -> Self {
        Self {
            focal_length,
            kind,
            name: name.map(|name| name.to_string()),
            maker: maker.map(|maker| maker.to_string()),
        }
    }
}
