pub struct Scanner {
    source: Vec<char>,
    current: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            current: 0,
        }
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    pub fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
           return None;
        }

        let ch = self.source[self.current];
        self.current += 1;
        Some(ch)
    }
    pub fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
           Some(self.source[self.current])
        }
    }
    pub fn peek_next(&self) -> Option<char> {
       if self.current + 1 >= self.source.len() {
           None
       } else {
           Some(self.source[self.current + 1])
       }
    }
    pub fn match_char(&mut self, expected: char) -> bool {
       if self.is_at_end() {
            return false;
       }

       if self.source[self.current] != expected {
           return false;
       }

       self.current += 1;
       true
    }
    
}
