#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Hai,
    Kbye,
    Obtw,
    Tldr,
    Maek,
    Gimmeh,
    Mkay,
    Oic,
    Ihaz,
    Itiz,
    Lemmesee,
    Head,
    Title,
    Paragraf,
    Bold,
    Italics,
    List,
    Item,
    Linx,
    Newline,
    Text,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize, col: usize) -> Self {
        Self {
            kind,
            lexeme,
            line,
            col,
        }
    }
}
