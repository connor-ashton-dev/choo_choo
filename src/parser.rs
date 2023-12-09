pub mod tools {
    use std::collections::HashMap;

    use crate::dom;
    use dom::vdom::Node;

    pub struct Parser {
        pos: usize,
        input: String,
    }

    impl Parser {
        /// read next char without consuming it
        fn next_char(&self) -> char {
            self.input[self.pos..].chars().next().unwrap()
        }

        /// do next characters start with the given string?
        fn starts_with(&self, s: &str) -> bool {
            self.input[self.pos..].starts_with(s)
        }

        /// return true if all input is consumed
        fn eof(&self) -> bool {
            self.pos >= self.input.len()
        }

        /// return cur char, and advance self.pos to next char
        fn consume_char(&mut self) -> char {
            let mut iter = self.input[self.pos..].char_indices();
            let (_, cur_char) = iter.next().unwrap();
            let (next_pos, _) = iter.next().unwrap_or((1, ' '));
            self.pos += next_pos;
            cur_char
        }

        /// consume characers until `test` returns false.
        fn consume_while<F>(&mut self, test: F) -> String
        where
            F: Fn(char) -> bool,
        {
            let mut result = String::new();
            while !self.eof() && test(self.next_char()) {
                result.push(self.consume_char());
            }
            result
        }

        /// Consume and discard zero or more whitespace characers
        fn consume_whitespace(&mut self) {
            self.consume_while(char::is_whitespace);
        }

        /// parse a tag or attribute name
        fn parse_tag_name(&mut self) -> String {
            self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
        }

        /// parse a single node
        /// either uses `fn parse_text`
        /// or `fn parse_element`
        fn parse_node(&mut self) -> Node {
            match self.next_char() {
                '<' => self.parse_element(),
                _ => self.parse_text(),
            }
        }

        /// parse a text node
        fn parse_text(&mut self) -> Node {
            dom::vdom::text(self.consume_while(|c| c != '<'))
        }

        /// parse a single element, including it's opening tag, contents, and closing tag.
        fn parse_element(&mut self) -> Node {
            // Opening tag
            assert!(self.consume_char() == '<');
            let tag_name = self.parse_tag_name();
            let attrs = self.parse_attributes();
            assert!(self.consume_char() == '>');

            // Contents
            let children = self.parse_nodes();

            // Closing tag
            assert!(self.consume_char() == '<');
            assert!(self.consume_char() == '/');
            assert!(self.parse_tag_name() == tag_name);
            assert!(self.consume_char() == '>');

            dom::vdom::elem(tag_name, attrs, children)
        }

        /// parse a single `name="value"` pair
        fn parse_attr(&mut self) -> (String, String) {
            let name = self.parse_tag_name();
            assert!(self.consume_char() == '=');
            let value = self.parse_attr_value();
            (name, value)
        }

        /// parse a quoted value
        fn parse_attr_value(&mut self) -> String {
            let open_quote = self.consume_char();
            assert!(open_quote == '"' || open_quote == '\'');
            let value = self.consume_while(|c| c != open_quote);
            assert!(self.consume_char() == open_quote);
            value
        }

        /// parse a list of `name="value"` pairs, separated by whitespace
        fn parse_attributes(&mut self) -> dom::vdom::AttrMap {
            let mut attributes = HashMap::new();
            loop {
                self.consume_whitespace();
                if self.next_char() == '>' {
                    break;
                }
                let (name, value) = self.parse_attr();
                attributes.insert(name, value);
            }
            attributes
        }

        /// parse a sequence of sibling nodes
        fn parse_nodes(&mut self) -> Vec<dom::vdom::Node> {
            let mut nodes = Vec::new();
            loop {
                self.consume_whitespace();
                if self.eof() || self.starts_with("</") {
                    break;
                }
                nodes.push(self.parse_node());
            }
            nodes
        }

        /// Parse an HTML document and return the root element.
        pub fn parse(source: String) -> dom::vdom::Node {
            let mut nodes = Parser {
                pos: 0,
                input: source,
            }
            .parse_nodes();

            if nodes.len() == 1 {
                nodes.swap_remove(0)
            } else {
                dom::vdom::elem("html".to_string(), HashMap::new(), nodes)
            }
        }
    }
}
