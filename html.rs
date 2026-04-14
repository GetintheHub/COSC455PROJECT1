use crate::syntaxtree::Node;
use std::collections::HashMap;


/// This struct walks the compiled syntax tree and emits HTML markup
/// It also tracks scoped variable definitions so that variable uses  can be resolved while generating the output
pub struct Htmlgenerator {
    scopes: Vec<HashMap<String, String>>,
}

impl Htmlgenerator {
    /// Create a new HTML generator with an initial global scope
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

 
    /// This is primarily used to isolate variable definitions inside  document/head/paragraph blocks if needed
    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Exit the current scope and restore the previous one
    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    /// Define a named variable in the current scope
    fn define(&mut self, name: &str, value: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), value.to_string());
        }
    }

    /// Lookup a variable value by name, searching from the innermost scope outward
    fn lookup(&self, name: &str) -> Option<String> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    /// The generator matches each node kind and emits the corresponding HTML tag with special handling for scoped structures and variable definitions
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
            Node::List(children) => format!("<ul>{}</ul>", self.render_nodes(children)),
            Node::Item(children) => format!("<li>{}</li>", self.render_nodes(children)),
            Node::Comment(text) => format!("<!--{}-->", text),
            Node::Link(url) => format!("<a href=\"{0}\">{0}</a>", url),
            Node::Newline => "<br>".to_string(),
            Node::Text(text) => text.clone(),
            Node::VarDef { name, value } => {
                // Variables are not rendered directly they are stored in scope
                
                self.define(name, value);
                String::new()
            }
            Node::VarUse(name) => self.lookup(name).unwrap_or_else(|| {
                eprintln!("Undefined variable {}", name);
                std::process::exit(1);
            }),
        }
    }

    /// Render a list of nodes sequentially into a single HTML string
    fn render_nodes(&mut self, nodes: &[Node]) -> String {
        let mut result = String::new();
        for node in nodes {
            result.push_str(&self.generate(node));
        }
        result
    }
}
