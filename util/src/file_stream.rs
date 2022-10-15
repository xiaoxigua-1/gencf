use std::str::Chars;

pub struct FileStream<'a> {
    current: Chars<'a>,
    pub index: usize,
}

impl FileStream<'_> {
    pub fn new<'a>(file_content: Chars<'a>) -> FileStream<'a> {
        FileStream {
            current: file_content,
            index: 0,
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        self.index += 1;
        self.current.next()
    }

    pub fn peep_char(&mut self) -> Option<char> {
        self.current.clone().next()
    }
}
