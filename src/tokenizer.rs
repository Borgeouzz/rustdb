type Token = String;

/* Tokenizer is an object that can be used to transform a string given in input
 * into a list of tokens. The tokens are returned by an iterator object.
 */
pub struct Tokenizer {
    tokens: Vec<Token>,
    position: usize,
}

impl Iterator for Tokenizer {
    type Item = Token;

    /* next returns the next token in the iterator.
     * If there are no more tokens, it returns None.
     */
    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }
}

impl Tokenizer {
    /* new creates a new tokenizer object.
     * It is used to initialize the tokenizer object.
     */
    pub fn new() -> Tokenizer {
        Tokenizer {
            tokens: Vec::new(),
            position: 0,
        }
    }

    // tokenize transforms an input string into a list of tokens.
    pub fn tokenize(&mut self, input: &str) {
        self.tokens = input.split_whitespace().map(|s| s.to_string()).collect();
    }
}