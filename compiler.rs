use crate::lexer::Lexer;
use crate::syntaxtree::Node;
use crate::token::{Token, TokenKind};
use crate::traits::{Compiler, SyntaxAnalyzer};


/// This compiler performs tokenization using the lexer and a recursive
/// descent parse over the grammar
pub struct Lolcompiler {
    tokens: Vec<Token>,
    pos: usize,
    current: String,
    ast: Option<Node>,
    node_stack: Vec<Vec<Node>>,
}

impl Lolcompiler {
    /// Create a new empty compiler with no source loaded
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            pos: 0,
            current: String::new(),
            ast: None,
            node_stack: Vec::new(),
        }
    }

    /// Return the completed syntax tree or exit if compilation failed
    pub fn ast(&self) -> &Node {
        self.ast.as_ref().unwrap_or_else(|| {
            eprintln!("No syntax tree available");
            std::process::exit(1);
        })
    }

    /// Get the current token at the parser pointer
    fn current_struct(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or_else(|| {
            self.tokens.last().unwrap_or_else(|| {
                eprintln!("Compiler error");
                std::process::exit(1);
            })
        })
    }

    fn current_kind(&self) -> TokenKind {
        self.current_struct().kind.clone()
    }

    /// Advance the parser to the next token and update the current lexeme string
    fn advance_struct(&mut self) {
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        self.current = self.current_struct().lexeme.clone();
    }

    
    /// If the current token is correct consume it, else exit with an error message
    fn expect(&mut self, kind: TokenKind) {
        if self.current_kind() == kind {
            self.advance_struct();
        } else {
            let token = self.current_struct();
            eprintln!(
                "Syntax error at {}, col {}: expected {:?}, found {:?}",
                token.line, token.col, kind, token.kind
            );
            std::process::exit(1);
        }
    }

    /// Push a parsed node into the current active node list
    fn push_node(&mut self, node: Node) {
        if let Some(nodes) = self.node_stack.last_mut() {
            nodes.push(node);
        } else {
            eprintln!("Compiler error");
            std::process::exit(1);
        }
    }

  
    /// This is used for constructs like title, bold and italics where the  parser gathers text content until a closing annotation
    fn collect_text(&mut self, end: TokenKind) -> String {
        let mut text = String::new();
        while self.current_kind() != end && self.current_kind() != TokenKind::Eof {
            text.push_str(&self.current_struct().lexeme);
            self.advance_struct();
        }
        text
    }

   
    /// This method uses a node stack to accumulate child nodes for block structures
    fn parse_nodes(&mut self, end: TokenKind) -> Vec<Node> {
        self.node_stack.push(Vec::new());
        while self.current_kind() != end && self.current_kind() != TokenKind::Eof {
            match self.current_kind() {
                TokenKind::Text => self.parse_text(),
                TokenKind::Maek => {
                    self.expect(TokenKind::Maek);
                    match self.current_kind() {
                        TokenKind::Head => self.parse_head(),
                        TokenKind::Paragraf => self.parse_paragraph(),
                        TokenKind::List => self.parse_list(),
                        _ => {
                            eprintln!("incorrect #MAEK block");
                            std::process::exit(1);
                        }
                    }
                }
                TokenKind::Gimmeh => {
                    self.expect(TokenKind::Gimmeh);
                    match self.current_kind() {
                        TokenKind::Title => self.parse_title(),
                        TokenKind::Bold => self.parse_bold(),
                        TokenKind::Italics => self.parse_italics(),
                        TokenKind::Item => self.parse_list_items(),
                        TokenKind::Linx => self.parse_link(),
                        TokenKind::Newline => self.parse_newline(),
                        _ => {
                            eprintln!("invalid #GIMMEH block");
                            std::process::exit(1);
                        }
                    }
                }
                TokenKind::Obtw => self.parse_comment(),
                TokenKind::Ihaz => self.parse_variable_define(),
                TokenKind::Lemmesee => self.parse_variable_use(),
                TokenKind::Newline => self.parse_newline(),
                _ => {
                    let token = self.current_struct();
                    eprintln!(
                        "Syntax error at line {}, col {}: unexpected token {:?}",
                        token.line, token.col, token.kind
                    );
                    std::process::exit(1);
                }
            }
        }
        self.node_stack.pop().unwrap_or_default()
    }
}

impl Compiler for Lolcompiler {
    /// Compile source text by lexing and parsing it into an AST
    fn compile(&mut self, source: &str) {
        let mut lexer = Lexer::new(source);
        self.tokens = lexer.get_all_tokens();
        self.pos = 0;
        self.current = self
            .tokens
            .first()
            .map(|token| token.lexeme.clone())
            .unwrap_or_default();
        self.ast = None;
        self.parse();
    }

    /// Start the parsing phase after tokenization
    fn parse(&mut self) {
        self.parse_lolcode();
    }
}

impl SyntaxAnalyzer for Lolcompiler {
  
    /// The program must begin with `#HAI` and end with `#KBYE`
    fn parse_lolcode(&mut self) {
        self.expect(TokenKind::Hai);
        let body = self.parse_nodes(TokenKind::Kbye);
        self.expect(TokenKind::Kbye);
        self.ast = Some(Node::Document(body));
    }

    fn parse_head(&mut self) {
        self.expect(TokenKind::Head);
        let body = self.parse_nodes(TokenKind::Mkay);
        self.expect(TokenKind::Mkay);
        self.push_node(Node::Head(body));
    }

    fn parse_title(&mut self) {
        self.expect(TokenKind::Title);
        let text = self.collect_text(TokenKind::Oic).trim().to_string();
        self.expect(TokenKind::Oic);
        self.push_node(Node::Title(text));
    }

    fn parse_comment(&mut self) {
        self.expect(TokenKind::Obtw);
        let text = self.collect_text(TokenKind::Tldr);
        self.expect(TokenKind::Tldr);
        self.push_node(Node::Comment(text));
    }

    fn parse_paragraph(&mut self) {
        self.expect(TokenKind::Paragraf);
        let body = self.parse_nodes(TokenKind::Mkay);
        self.expect(TokenKind::Mkay);
        self.push_node(Node::Paragraph(body));
    }

    fn parse_variable_define(&mut self) {
        self.expect(TokenKind::Ihaz);
        let name = self.current_struct().lexeme.trim().to_string();
        self.expect(TokenKind::Text);
        self.expect(TokenKind::Itiz);
        let value = self.collect_text(TokenKind::Mkay).trim().to_string();
        self.expect(TokenKind::Mkay);
        self.push_node(Node::VarDef { name, value });
    }

    fn parse_variable_use(&mut self) {
        self.expect(TokenKind::Lemmesee);
        let name = self.current_struct().lexeme.trim().to_string();
        self.expect(TokenKind::Text);
        self.expect(TokenKind::Oic);
        self.push_node(Node::VarUse(name));
    }

    fn parse_bold(&mut self) {
        self.expect(TokenKind::Bold);
        let text = self.collect_text(TokenKind::Oic).trim().to_string();
        self.expect(TokenKind::Oic);
        self.push_node(Node::Bold(text));
    }

    fn parse_italics(&mut self) {
        self.expect(TokenKind::Italics);
        let text = self.collect_text(TokenKind::Oic).trim().to_string();
        self.expect(TokenKind::Oic);
        self.push_node(Node::Italics(text));
    }

    fn parse_list(&mut self) {
        self.expect(TokenKind::List);
        let body = self.parse_nodes(TokenKind::Mkay);
        self.expect(TokenKind::Mkay);
        self.push_node(Node::List(body));
    }

    fn parse_list_items(&mut self) {
        self.expect(TokenKind::Item);
        let text = self.collect_text(TokenKind::Oic);
        self.expect(TokenKind::Oic);
        self.push_node(Node::Item(vec![Node::Text(text)]));
    }

    fn parse_link(&mut self) {
        self.expect(TokenKind::Linx);
        let text = self.collect_text(TokenKind::Oic).trim().to_string();
        self.expect(TokenKind::Oic);
        self.push_node(Node::Link(text));
    }

    fn parse_newline(&mut self) {
        if self.current_kind() == TokenKind::Gimmeh {
            self.expect(TokenKind::Gimmeh);
            self.expect(TokenKind::Newline);
        } else {
            self.expect(TokenKind::Newline);
        }
        self.push_node(Node::Newline);
    }

    fn parse_text(&mut self) {
        let text = self.current_struct().lexeme.clone();
        self.expect(TokenKind::Text);
        self.push_node(Node::Text(text));
    }
}
