#[derive(Clone)]
pub struct Image {
    src: String,
}

impl Image {
    pub fn new(src: String) -> Self {
        Self { src }
    }
    pub fn get_src(&self) -> String {
        self.src.clone()
    }
}
