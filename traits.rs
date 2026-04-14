pub trait Compiler {
    fn compile(&mut self, source: &str);
    fn parse(&mut self);
}

pub trait LexicalAnalyzer {
    fn get_char(&mut self) -> char;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
}

pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self);
    fn parse_head(&mut self);
    fn parse_title(&mut self);
    fn parse_comment(&mut self);
    fn parse_paragraph(&mut self);
    fn parse_variable_define(&mut self);
    fn parse_variable_use(&mut self);
    fn parse_bold(&mut self);
    fn parse_italics(&mut self);
    fn parse_list(&mut self);
    fn parse_list_items(&mut self);
    fn parse_link(&mut self);
    fn parse_newline(&mut self);
    fn parse_text(&mut self);
}
