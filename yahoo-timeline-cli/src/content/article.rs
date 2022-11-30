#[derive(Clone)]
pub struct Article {
    title: String,
    url: String,
}

impl Article {
    pub fn new(title: String, url: String) -> Self {
        Self { title, url }
    }
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}
