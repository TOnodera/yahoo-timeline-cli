#[derive(Clone)]
pub struct Comment {
    comment: String,
}

impl Comment {
    pub fn new(comment: String) -> Self {
        Self { comment }
    }
    pub fn get_comment(&self) -> String {
        self.comment.clone()
    }
}
