use crate::token::{Token, TokenKind};

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            input: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn is_done(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn peek(&self) -> Option<char> {
        if self.is_done() {
            None
        } else {
            Some(self.input[self.pos])
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_done() {
            return None;
        }

        let ch = self.input[self.pos];
        self.pos += 1;

        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }

        Some(ch)
    }

    fn starts_with_ignore_case(&self, s: &str) -> bool {
        let mut temp = String::new();

        for i in 0..s.len() {
            if self.pos + i >= self.input.len() {
                return false;
            }
            temp.push(self.input[self.pos + i]);
        }

        temp.eq_ignore_ascii_case(s)
    }

    fn take_n(&mut self, n: usize) -> String {
        let mut out = String::new();

        for _ in 0..n {
            if let Some(ch) = self.advance() {
                out.push(ch);
            }
        }

        out
    }

    pub fn next_token(&mut self) -> Token {
        if self.is_done() {
            return Token::new(TokenKind::Eof, "".to_string(), self.line, self.col);
        }

        let start_line = self.line;
        let start_col = self.col;

        if self.peek() == Some('#') {
            let checks = vec![
                ("#LEMMESEE", TokenKind::Lemmesee),
                ("#PARAGRAF", TokenKind::Paragraf),
                ("#ITALICS", TokenKind::Italics),
                ("#NEWLINE", TokenKind::Newline),
                ("#GIMMEH", TokenKind::Gimmeh),
                ("#TITLE", TokenKind::Title),
                ("#OBTW", TokenKind::Obtw),
                ("#TLDR", TokenKind::Tldr),
                ("#MAEK", TokenKind::Maek),
                ("#HEAD", TokenKind::Head),
                ("#BOLD", TokenKind::Bold),
                ("#LIST", TokenKind::List),
                ("#ITEM", TokenKind::Item),
                ("#LINX", TokenKind::Linx),
                ("#IHAZ", TokenKind::Ihaz),
                ("#ITIZ", TokenKind::Itiz),
                ("#MKAY", TokenKind::Mkay),
                ("#KBYE", TokenKind::Kbye),
                ("#HAI", TokenKind::Hai),
                ("#OIC", TokenKind::Oic),
            ];

            for (word, kind) in checks {
                if self.starts_with_ignore_case(word) {
                    let lex = self.take_n(word.len());
                    return Token::new(kind, lex, start_line, start_col);
                }
            }

            eprintln!(
                "Lexical error at line {}, col {}. Bad annotation.",
                start_line, start_col
            );
            std::process::exit(1);
        }

        let mut text = String::new();

        while let Some(ch) = self.peek() {
            if ch == '#' {
                break;
            }
            text.push(self.advance().unwrap());
        }

        Token::new(TokenKind::Text, text, start_line, start_col)
    }

    pub fn get_all_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let t = self.next_token();
            let end = t.kind == TokenKind::Eof;
            tokens.push(t);

            if end {
                break;
            }
        }

        tokens
    }
}