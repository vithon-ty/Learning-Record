#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    String(String), // 文字列
    Number(f64),    // 数値
    Bool(bool),     // 真偽値
    Null,           // Null
    WhiteSpace,     // 空白
    LeftBrace,      // {  JSON object 開始文字
    RightBrace,     // }  JSON object 終了文字
    LeftBracket,    // [  JSON array  開始文字
    RightBrachet,   // ]  JSON array  終了文字
    Comma,          // ,  JSON value  区切り文字
    Colon,          // :  "key":value 区切り文字
}

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

#[derive(Debug)]
pub struct LexerError {
    // Error message
    pub msg: String,
}

impl LexerError {
    fn new(msg: &str) -> LexerError {
        LexerError {
            msg: msg.to_string(),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    /// 文字列を`Token`単位に分割する
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];
        while let Some(token) = self.next_token()? {
            match token {
                Token::WhiteSpace => {}
                _ => {
                    tokens.push(token);
                }
            }
        }
        Ok(tokens)
    }

    /// １文字分だけ読み進め、tokenを返す
    fn next_return_token(&mut self, token: Token) -> Option<Token> {
        self.chars.next();
        Some(token)
    }

    /// 文字列を読み込み、マッチしたTokenを返す
    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        match self.chars.peek() {
            Some(c) => match c {
                // 一文字分だけ読み進めTokenを返す

                // WhiteSpace' 'もしくは'\n'
                c if c.is_whitespace() || *c == '\n' => {
                    Ok(self.next_return_token(Token::WhiteSpace))
                }
                '{' => Ok(self.next_return_token(Token::LeftBrace)),
                '}' => Ok(self.next_return_token(Token::RightBrace)),
                '[' => Ok(self.next_return_token(Token::LeftBracket)),
                ']' => Ok(self.next_return_token(Token::RightBrachet)),
                ',' => Ok(self.next_return_token(Token::Comma)),
                ':' => Ok(self.next_return_token(Token::Colon)),

                // Note
                // 以下のマッチ条件は開始文字が該当するTokenの開始文字なら、Tokenの文字列分だけ読み進める

                // Stringは開始文字 '"'
                // e.g. "togatoga"
                '"' => {
                    // parse string
                    self.chars.next();
                    self.parse_string_token()
                }
                // Numberは開始文字が[0-9]もしくは('+', '-', '.')
                // e.g.
                //    -1234
                //    +10
                //    .0001
                c if c.is_numeric() || matches!(c, '+' | '-' | '.') => self.parse_number_token(),
                // Booleanの"true"の開始文字は't'
                // e.g.
                //    true
                't' => self.parse_bool_token(true),
                // Booleanの"false"の開始文字は'f'
                // e.g.
                //    false
                'f' => self.parse_bool_token(false),
                // Nullの開始文字は'n'
                // e.g.
                //    null
                'n' => self.parse_null_token(),
                // 上のルールにマッチしない文字はエラー
                _ => Err(LexerError::new(&format!("error: an unexpected char {}", c))),
            },
            None => Ok(None),
        }
    }

    /// nullの文字列をparseする
    fn parse_null_token(&mut self) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }

    /// (true|false)の文字列をparseする
    fn parse_bool_token(&mut self, b: bool) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }

    /// 数字として使用可能な文字まで読み込む。読み込んだ文字列が数字(`f64`)としてparseに成功した場合Tokenを返す。
    fn parse_number_token(&mut self) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }

    /// 終端文字'\"'まで文字列を読み込む。UTF-16(\u000~\uFFF)や特殊なエスケープ文字(e.g. '\t', '\n')も考慮する
    fn parse_string_token(&mut self) -> Result<Option<Token>, LexerError> {
        unimplemented!()
    }

    /// utf16のバッファが存在するならば連結しておく
    fn push_utf16(result: &mut String, utf16: &mut Vec<u16>) -> Result<(), LexerError> {
        unimplemented!()
    }
}