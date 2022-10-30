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
        self.current.clone().nth(self.index - 1)
    }

    pub fn peep_char(&mut self) -> Option<char> {
        self.current.clone().nth(self.index)
    }

    pub fn reset(&mut self, index: usize) {
        self.index = index;
    }

    pub fn now(&self) {
        println!("{}", self.current.as_str());
    }
}
