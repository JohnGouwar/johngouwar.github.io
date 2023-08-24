use build_html::Html;

#[derive(Debug)]
pub struct Span {
    content: String,
    class: String,
    id: Option<String>,
}


impl Span {
    pub fn new(content: impl ToString, class: impl ToString, id: Option<impl ToString>) -> Self {
        Span {
            content: content.to_string(),
            class: class.to_string(),
            id: id.map(|s| s.to_string()),
        }
    }
}

impl Html for Span {
    fn to_html_string(&self) -> String {
        match &self.id {
            None => format!("<span class=\"{}\">{}</span>", self.class, self.content),
            Some(id) => format!(
                "<span class=\"{}\" id=\"{}\">{}</span>",
                self.class, id, self.content
            ),
        }
    }
}

#[derive(Debug)]
pub struct Break {}
impl Html for Break {
    fn to_html_string(&self) -> String {
        "<br/>".to_string()
    }
}

