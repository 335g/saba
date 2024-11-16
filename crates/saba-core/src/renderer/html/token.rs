use crate::renderer::html::attribute::Attribute;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HtmlTokenizer {
    state: State,
    reconsume: bool,
    latest_token: Option<HtmlToken>,
    input: Vec<char>,
    buf: String,
}

impl HtmlTokenizer {
    pub fn new(html: String) -> Self {
        Self {
            state: State::Data,
            reconsume: false,
            latest_token: None,
            input: html.chars().collect(),
            buf: String::new(),
        }
    }

    fn is_eof(&self) -> bool {
        self.input.is_empty()
    }

    fn consume_next_input(&mut self) -> char {
        self.input.remove(0)
    }
}

impl Iterator for HtmlTokenizer {
    type Item = HtmlToken;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.consume_next_input();

            match self.state {
                State::Data => {
                    if c == '<' {
                        self.state = State::TagOpen;
                        continue;
                    }

                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    } else {
                        return Some(HtmlToken::Char(c));
                    }
                }
                State::TagOpen => todo!(),
                State::EndTagOpen => todo!(),
                State::TagName => todo!(),
            }
        }
    }
}

/// cf. https://html.spec.whatwg.org/multipage/parsing.html
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum State {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HtmlToken {
    StartTag {
        tag: String,
        self_closing: bool,
        attributes: Vec<Attribute>,
    },
    EndTag {
        tag: String,
    },
    Char(char),
    Eof,
}
