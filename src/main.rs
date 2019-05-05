use rnix::types::Wrapper;
use syntex_syntax::print::pp;

pub const DEFAULT_COLUMNS: usize = 78;

fn main() -> Result<(), Error> {
    let contents = std::fs::read_to_string("./tests/golden/set.nix")?;

    let ast = rnix::parse(&contents);

    if !ast.errors().is_empty() {
        return Err(Error::ParseErrors(ast.errors()));
    }

    let mut writer = Vec::new();
    let mut node_stack = Vec::new();

    {
        // NOTE: Need a block here to contain the borrow of `writer` below.
        let mut printer = pp::mk_printer(Box::new(&mut writer), DEFAULT_COLUMNS);

        for event in ast.root().inner().preorder_with_tokens() {
            match event {
                rowan::WalkEvent::Enter(node) => {
                    match node {
                        rowan::SyntaxElement::Node(node) => node_stack.push(node),
                        rowan::SyntaxElement::Token(token) => {
                            use rnix::tokenizer::tokens::*;
                            //match (token.kind(), nodes.last()) {
                            match token.kind() {
                                TOKEN_COMMENT => (),
                                TOKEN_WHITESPACE => (),

                                _ => pp::word(&mut printer, token.text())?,
                            }
                        }
                    }
                }
                rowan::WalkEvent::Leave(_) => {
                    node_stack.pop();
                    ()
                }
            }
        }
    }

    let result = String::from_utf8(writer)?;
    println!("{}", result);
    Ok(())
}

#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    ParseErrors(Vec<rnix::parser::ParseError>),
    EncodingError(std::string::FromUtf8Error),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}

impl std::convert::From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::EncodingError(err)
    }
}
