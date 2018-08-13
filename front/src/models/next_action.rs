pub struct NextAction {
    pub uuid: String,
    pub content: String,
    pub project: Option<String>,
    pub context: Option<String>
}

impl NextAction {
    pub fn new(content: String, project: Option<String>, context: Option<String>) -> Self {
        // TODO: Obtain UUID via server
        NextAction {
            uuid: String::new(),
            content: content,
            project: project,
            context: context,
        }
    }
}
