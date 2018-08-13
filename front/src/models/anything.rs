#[derive(Clone)]
pub struct Anything {
    pub uuid: String,
    pub content: String,
}

impl Anything {
    pub fn new(content: String) -> Self {
        Anything {
            uuid: String::new(),
            content: content,
        }
    }
}
