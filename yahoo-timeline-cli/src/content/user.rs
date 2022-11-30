#[derive(Clone)]
pub struct User {
    id: String,
}

impl User {
    pub fn new(id: String) -> Self {
        Self { id }
    }
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}
