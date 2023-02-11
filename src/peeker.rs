#[derive(Debug)]
struct Peeker {
    vec: Vec<Token>,
    at: usize,
}

impl Peeker {
    fn next(&mut self) -> Option<&Token> {
        if self.at >= self.vec.len() {
            return None;
        }
        self.at += 1;
        return self.vec.get(self.at);
    }

    fn peek(&self, num: usize) -> Option<&Token> {
        return self.vec.get(self.at + num);
    }
}