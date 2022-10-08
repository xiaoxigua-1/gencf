use std::{path::Path, str::Chars, fs};

struct FileStream<'a> {
    path: &'a Path,
    current: Chars<'a>,
}

pub struct Lexer<'a> {
    file_stream: FileStream<'a>
}

impl FileStream<'_> {
    fn new<'a>(path: &'a Path, file_content: Chars<'a>) -> FileStream<'a> {
        FileStream {
            path,
            current: file_content,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.current.next()
    }

    fn peep_char(&mut self) -> Option<char> {
        self.current.clone().next()
    }
}

impl Lexer<'_> {
    pub fn new<'a>(path: &'a Path, file_content: &'a String) -> Lexer<'a> {
        Lexer {
            file_stream: FileStream::new(path, file_content.chars())
        }
    }

    pub fn lex(&self) {
        // match self.file_stream.next_char() {
        //     None => EOF,
        // }
    }

    // pub fn next_token(&mut self) -> Option<Token> {

    // }
}
