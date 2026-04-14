#[derive(Debug, Clone)]
pub enum Node {
    Document(Vec<Node>),
    Head(Vec<Node>),
    Title(String),
    Paragraph(Vec<Node>),
    Bold(String),
    Italics(String),
    List(Vec<Node>),
    Item(Vec<Node>),
    Comment(String),
    Link(String),
    Newline,
    Text(String),
    VarDef { name: String, value: String },
    VarUse(String),
}
