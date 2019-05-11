use rnix::types::Wrapper;
use syntex_syntax::print::pp;

pub const DEFAULT_COLUMNS: usize = 78;
pub const INDENT_UNIT: usize = 2;

fn main() -> Result<(), Error> {
    let contents = std::fs::read_to_string("./tests/golden/set.nix")?;

    let ast = rnix::parse(&contents);

    if !ast.errors().is_empty() {
        return Err(Error::ParseErrors(ast.errors()));
    }

    let mut writer = Vec::new();

    // NOTE: Need a block here to contain the borrow of `writer`
    {
        let mut printer = pp::mk_printer(Box::new(&mut writer), DEFAULT_COLUMNS);

        for event in ast.root().inner().preorder_with_tokens() {
            match event {
                rowan::WalkEvent::Enter(node) => match node {
                    rowan::SyntaxElement::Node(_) => (),

                    rowan::SyntaxElement::Token(token) => {
                        print_token(&mut printer, node, token)?;
                    }
                },
                rowan::WalkEvent::Leave(_) => (),
            }
        }
        printer.pretty_print(pp::Token::Eof)?;
    }

    let result = String::from_utf8(writer)?;
    println!("{}", result);
    Ok(())
}

/// The Glorious Match.
fn print_token(
    pr: &mut pp::Printer,
    node: rowan::SyntaxElement,
    token: rowan::SyntaxToken,
) -> Result<(), Error> {
    use rnix::parser::nodes::*;

    // TODO(?)
    match node.kind() {
        _ => (),
    };

    match token.kind() {
        TOKEN_WHITESPACE => (),

        TOKEN_COMMENT => (), // TODO

        TOKEN_SEMICOLON => {
            pp::word(pr, ";")?;
            pp::space(pr)?;
        }

        TOKEN_ASSIGN => {
            pp::word(pr, " = ")?;
        }

        TOKEN_CURLY_B_OPEN => {
            // Containing cbox, will be closed after "}" in
            // the `TOKEN_CURLY_B_CLOSE` case (below)
            pp::cbox(pr, INDENT_UNIT)?;

            pp::ibox(pr, 0)?;
            pp::word(pr, "{")?;
            pp::end(pr)?;

            pp::space(pr)?;
        }

        TOKEN_CURLY_B_CLOSE => {
            // So we need to unindent here, and the best solution I've been able
            // to come up with is unsafe...
            //
            // (can't do this because we don't always want a hard break)
            // pr.replace_last_token(pp::hardbreak_tok_offset(-(INDENT_UNIT as isize)));
            //
            // (can't do this because we might end up with an empty line)
            // pp::break_offset(pr, 1, -(INDENT_UNIT as isize))?;
            //
            // So yolo let's just change the offset:
            match pr.last_token() {
                pp::Token::Break(bt) => unsafe {
                    let bt_unindented =
                        pp::Token::Break(replace_offset(bt, -(INDENT_UNIT as isize)));
                    pr.replace_last_token(bt_unindented);
                },
                _ => (),
            };

            pp::word(pr, "}")?;
            pp::end(pr)?;
        }

        _ => {
            pp::word(pr, token.text())?;
        }
    }
    Ok(())
}

/// Things what might go wrong.
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

/// Lol this is probably a terrible idea but let's got with it...
unsafe fn replace_offset(bt: pp::BreakToken, offset: isize) -> pp::BreakToken {
    #[allow(dead_code)]
    pub struct BreakToken {
        offset: isize,
        blank_space: isize,
    }
    let bt_exposed: BreakToken = std::mem::transmute(bt);
    std::mem::transmute(BreakToken {
        offset,
        blank_space: bt_exposed.blank_space,
    })
}
