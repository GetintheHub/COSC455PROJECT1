use crate::syntaxtree::Node;
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

pub struct Parser {
    lexer: Lexer,
    current: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        Parser { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    fn expect(&mut self, kind: TokenKind) {
        if self.current.kind == kind {
            self.advance();
        } else {
            eprintln!(
                "Syntax error {}: expected {:?}, found {:?}",
                self.current.line, kind, self.current.kind
            );
            std::process::exit(1);
        }
    }

    pub fn parse(&mut self) -> Node {
        self.expect(TokenKind::Hai);
        let body = self.parse_body();
        self.expect(TokenKind::Kbye);
        Node::Document(body)
    }

    fn parse_body(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while self.current.kind != TokenKind::Kbye
            && self.current.kind != TokenKind::Eof
        {
            match self.current.kind {
                TokenKind::Text => {
                    nodes.push(Node::Text(self.current.lexeme.clone()));
                    self.advance();
                }
                TokenKind::Maek => nodes.push(self.parse_maek()),
                TokenKind::Gimmeh => nodes.push(self.parse_gimmeh()),
                TokenKind::Obtw => nodes.push(self.parse_comment()),
                TokenKind::Ihaz => nodes.push(self.parse_var_def()),
                TokenKind::Lemmesee => nodes.push(self.parse_var_use()),
                TokenKind::Newline => {
                    nodes.push(Node::Newline);
                    self.advance();
                }
                _ => {
                    eprintln!("unknown token: {:?}", self.current.kind);
                    std::process::exit(1);
                }
            }
        }

        nodes
    }

    fn parse_maek(&mut self) -> Node {
        self.expect(TokenKind::Maek);

        match self.current.kind {
            TokenKind::Head => {
                self.advance();
                let body = self.parse_body();
                self.expect(TokenKind::Mkay);
                Node::Head(body)
            }
            TokenKind::Paragraf => {
                self.advance();
                let body = self.parse_body();
                self.expect(TokenKind::Mkay);
                Node::Paragraph(body)
            }
            TokenKind::List => {
                self.advance();
                let body = self.parse_body();
                self.expect(TokenKind::Mkay);
                Node::List(body)
            }
            _ => {
                eprintln!("incorrect #MAEK block");
                std::process::exit(1);
            }
        }
    }

    fn parse_gimmeh(&mut self) -> Node {
        self.expect(TokenKind::Gimmeh);

        match self.current.kind {
            TokenKind::Title => {
                self.advance();
                let text = self.collect_text_until(TokenKind::Oic);
                self.expect(TokenKind::Oic);
                Node::Title(text.trim().to_string())
            }
            TokenKind::Bold => {
                self.advance();
                let text = self.collect_text_until(TokenKind::Oic);
                self.expect(TokenKind::Oic);
                Node::Bold(text.trim().to_string())
            }
            TokenKind::Italics => {
                self.advance();
                let text = self.collect_text_until(TokenKind::Oic);
                self.expect(TokenKind::Oic);
                Node::Italics(text.trim().to_string())
            }
            TokenKind::Item => {
                self.advance();
                let text = self.collect_text_until(TokenKind::Oic);
                self.expect(TokenKind::Oic);
                Node::Item(vec![Node::Text(text)])
            }
            TokenKind::Linx => {
                self.advance();
                let text = self.collect_text_until(TokenKind::Oic);
                self.expect(TokenKind::Oic);
                Node::Link(text.trim().to_string())
            }
            TokenKind::Newline => {
                self.advance();
                Node::Newline
            }
            _ => {
                eprintln!("invalid #GIMMEH block");
                std::process::exit(1);
            }
        }
    }

    fn parse_comment(&mut self) -> Node {
        self.expect(TokenKind::Obtw);
        let text = self.collect_text_until(TokenKind::Tldr);
        self.expect(TokenKind::Tldr);
        Node::Comment(text)
    }

    fn parse_var_def(&mut self) -> Node {
        self.expect(TokenKind::Ihaz);
        let name = self.current.lexeme.trim().to_string();
        self.expect(TokenKind::Text);
        self.expect(TokenKind::Itiz);
        let value = self.collect_text_until(TokenKind::Mkay);
        self.expect(TokenKind::Mkay);
        Node::VarDef(name, value.trim().to_string())
    }

    fn parse_var_use(&mut self) -> Node {
        self.expect(TokenKind::Lemmesee);
        let name = self.current.lexeme.trim().to_string();
        self.expect(TokenKind::Text);
        self.expect(TokenKind::Oic);
        Node::VarUse(name)
    }

    fn collect_text_until(&mut self, end: TokenKind) -> String {
        let mut text = String::new();

        while self.current.kind != end && self.current.kind != TokenKind::Eof {
            text.push_str(&self.current.lexeme);
            self.advance();
        }

        text
    }
}