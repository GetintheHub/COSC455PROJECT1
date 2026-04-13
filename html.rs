use crate::syntaxtree::Node;
use std::collections::HashMap;

pub struct HtmlGenerator {
    scopes: Vec<HashMap<String, String>>,
}

impl HtmlGenerator {
    pub fn new() -> Self {
        HtmlGenerator {
            scopes: vec![HashMap::new()],
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn define(&mut self, name: &str, value: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), value.to_string());
        }
    }

    fn lookup(&self, name: &str) -> Option<String> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val.clone());
            }
        }
        None
    }

    pub fn generate(&mut self, node: &Node) -> String {
        match node {
            Node::Document(children) => {
                self.enter_scope();
                let body = self.render_nodes(children);
                self.exit_scope();
                format!("<html>{}</html>", body)
            }
            Node::Head(children) => {
                self.enter_scope();
                let body = self.render_nodes(children);
                self.exit_scope();
                format!("<head>{}</head>", body)
            }
            Node::Title(text) => format!("<title>{}</title>", text),
            Node::Paragraph(children) => {
                self.enter_scope();
                let body = self.render_nodes(children);
                self.exit_scope();
                format!("<p>{}</p>", body)
            }
            Node::Bold(text) => format!("<b>{}</b>", text),
            Node::Italics(text) => format!("<i>{}</i>", text),
            Node::List(children) => {
                let body = self.render_nodes(children);
                format!("<ul>{}</ul>", body)
            }
            Node::Item(children) => {
                let body = self.render_nodes(children);
                format!("<li>{}</li>", body)
            }
            Node::Comment(text) => format!("<!--{}-->", text),
            Node::Link(url) => format!("<a href=\"{0}\">{0}</a>", url),
            Node::Newline => "<br>".to_string(),
            Node::Text(text) => text.clone(),
            Node::VarDef(name, value) => {
                self.define(name, value);
                String::new()
            }
            Node::VarUse(name) => {
                self.lookup(name).unwrap_or_else(|| {
                    eprintln!("Undefined variable {}", name);
                    std::process::exit(1);
                })
            }
        }
    }

    fn render_nodes(&mut self, nodes: &[Node]) -> String {
        let mut result = String::new();
        for node in nodes {
            result.push_str(&self.generate(node));
        }
        result
    }
}