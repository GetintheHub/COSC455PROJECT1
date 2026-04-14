use crate::token::{Token, TokenKind};
use crate::traits::LexicalAnalyzer;


///  language annotations like `#HAI`, `#MAEK`, `#GIMMEH`,
///  text segments outside annotations,
///  newline tokens,
///  and end-of-file.
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    current_lexeme: String,
}

impl Lexer {
    /// Create a lexer from source text and initialize position tracking
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            current_lexeme: String::new(),
        }
    }

    /// Check whether the lexer has consumed all characters
    fn is_done(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// Peek at the current character without advancing
    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    /// Consume the current character and update line/column position
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


    /// An annotation is consumed until the next whitespace boundary then validated against the known token set.
    fn read_annotation(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.col;
        self.current_lexeme.clear();

        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                break;
            }
            let c = self.get_char();
            self.add_char(c);
        }

        let lexeme = self.current_lexeme.clone();

        if !self.lookup(&lexeme) {
            eprintln!(
                "Lexical error at line {}, col {}: invalid annotation '{}'.",
                start_line, start_col, lexeme
            );
            std::process::exit(1);
        }

        let kind = match lexeme.to_ascii_uppercase().as_str() {
            "#LEMMESEE" => TokenKind::Lemmesee,
            "#PARAGRAF" => TokenKind::Paragraf,
            "#ITALICS" => TokenKind::Italics,
            "#NEWLINE" => TokenKind::Newline,
            "#GIMMEH" => TokenKind::Gimmeh,
            "#TITLE" => TokenKind::Title,
            "#OBTW" => TokenKind::Obtw,
            "#TLDR" => TokenKind::Tldr,
            "#MAEK" => TokenKind::Maek,
            "#HEAD" => TokenKind::Head,
            "#BOLD" => TokenKind::Bold,
            "#LIST" => TokenKind::List,
            "#ITEM" => TokenKind::Item,
            "#LINX" => TokenKind::Linx,
            "#IHAZ" => TokenKind::Ihaz,
            "#ITIZ" => TokenKind::Itiz,
            "#MKAY" => TokenKind::Mkay,
            "#KBYE" => TokenKind::Kbye,
            "#HAI" => TokenKind::Hai,
            "#OIC" => TokenKind::Oic,
            _ => unreachable!(" invalid tokens."),
        };

        Token::new(kind, lexeme, start_line, start_col)
    }

    /// Read plain text until the next annotation marker
    fn read_text(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.col;
        self.current_lexeme.clear();

        while let Some(ch) = self.peek() {
            if ch == '#' {
                break;
            }
            let next = self.get_char();
            self.add_char(next);
        }

        Token::new(
            TokenKind::Text,
            self.current_lexeme.clone(),
            start_line,
            start_col,
        )
    }

 
    /// Whitespace is ignored except for newlines, which are significant because they map to the `Newline` token.
    pub fn next_token_struct(&mut self) -> Token {
        while let Some(ch) = self.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.get_char();
            } else {
                break;
            }
        }

        if self.is_done() {
            return Token::new(TokenKind::Eof, String::new(), self.line, self.col);
        }

        if self.peek() == Some('\n') {
            let start_line = self.line;
            let start_col = self.col;
            self.get_char();
            return Token::new(
                TokenKind::Newline,
                "\n".to_string(),
                start_line,
                start_col,
            );
        }

        if self.peek() == Some('#') {
            self.read_annotation()
        } else {
            self.read_text()
        }
    }

    /// Retrieve every token from the input until EOF.
    pub fn get_all_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token_struct();
            let done = token.kind == TokenKind::Eof;
            tokens.push(token);
            if done {
                break;
            }
        }
        tokens
    }
}

impl LexicalAnalyzer for Lexer {
    /// Safely consume a character or exit if input terminates unexpectedly
    fn get_char(&mut self) -> char {
        self.advance().unwrap_or_else(|| {
            eprintln!("Lexical error: unexpected end of input.");
            std::process::exit(1);
        })
    }

    /// Append a character to the current lexeme being built
    fn add_char(&mut self, c: char) {
        self.current_lexeme.push(c);
    }

    /// Validate whether the given string is a recognized lol annotation
    fn lookup(&self, s: &str) -> bool {
        matches!(
            s.to_ascii_uppercase().as_str(),
            "#LEMMESEE"
                | "#PARAGRAF"
                | "#ITALICS"
                | "#NEWLINE"
                | "#GIMMEH"
                | "#TITLE"
                | "#OBTW"
                | "#TLDR"
                | "#MAEK"
                | "#HEAD"
                | "#BOLD"
                | "#LIST"
                | "#ITEM"
                | "#LINX"
                | "#IHAZ"
                | "#ITIZ"
                | "#MKAY"
                | "#KBYE"
                | "#HAI"
                | "#OIC"
        )
    }
}
