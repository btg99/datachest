use crate::*;

pub struct ParseResult<'a, T, E>
{
    result: Result<T, E>,
    rest: &'a str
}

impl<'a, T, E> ParseResult<'a, T, E> {
    fn map<U, F: FnOnce(T) -> U>(self, op: F) -> ParseResult<'a, U, E> {
        ParseResult {
            result: self.result.map(op),
            rest: self.rest,
        }
    }

    fn map_error<G, F: FnOnce(E) -> G>(self, op: F) -> ParseResult<'a, T, G> {
        ParseResult {
            result: match self.result {
                Ok(t) => Ok(t),
                Err(e) => Err(op(e)),
            },
            rest: self.rest,
        }
    }
}

fn sequence<'a, T, U, E, F: FnOnce(&'a str) -> ParseResult<T, E>, G: FnOnce(&'a str) -> ParseResult<U, E>>(f: F, g: G) -> impl FnOnce(&'a str) -> ParseResult<U, E> {
    move |input: &'a str| {
        let first_result = f(input);
        match first_result.result {
            Ok(_) => g(first_result.rest),
            Err(e) => ParseResult {
                result: Err(e),
                rest: first_result.rest,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    line: u32,
    message: String,
}

pub fn parse_function(input: &str) -> Result<Vec<Command>, Vec<ParseError>> {
    Err(vec![])
}

pub fn parse_command<'a>(input: &'a str) -> ParseResult<'a, Option<Command>, String> {
    let lexeme = alphanumeric(input.trim_start());
    match lexeme.result {
        Ok(alpha) => match alpha {
            "scoreboard" => {
                sequence(|text| discard(text, ' ').map_error(|unexpected| match unexpected {
                    Some(unexpected) => expected_whitespace(text, unexpected),
                    None => format!("Expected a space but there was nothing."),
                }), scoreboard)(lexeme.rest).map(|scoreboard| Some(Command::Scoreboard(scoreboard)))
            },
            unexpected => ParseResult {
                result: Err(format!("Unknown command '{}'.", unexpected)),
                rest: lexeme.rest,
            }
        },
        Err(_) => ParseResult {
            result: Ok(None),
            rest: lexeme.rest,
        }
    }
}

fn scoreboard<'a>(input: &'a str) -> ParseResult<'a, Scoreboard, String> {
    let lexeme = alphanumeric(input);
    match lexeme.result {
        Ok(alpha) => match alpha {
            "objectives" => {
                sequence(|text| discard(text, ' ').map_error(|unexpected| match unexpected {
                    Some(unexpected) => expected_whitespace(text, unexpected),
                    None => format!("Expected a space but there was nothing."),}), objectives)(lexeme.rest).map(|objectives| Scoreboard::Objectives(objectives))
            }
            unexpected => ParseResult {
                result: Err(format!("Unknown subcommand '{}'. Expected either 'objectives' or 'players'.", unexpected)),
                rest: lexeme.rest,
            }
        },
        Err(Some(whitespace)) if whitespace.is_whitespace() => {
            let permissive_result = scoreboard(input.trim_start());
            ParseResult {
                result: match permissive_result.result {
                    Ok(command) => Err(format!("Expected exactly one space before '{}'.", scoreboard_command_name(&command))),
                    Err(_) => Err(format!("Expected exactly one space."))
                },
                rest: lexeme.rest,
            }
            
        },
        Err(Some(unexpected)) => ParseResult {
            result: Err(format!("Unexpected symbol '{}'.", unexpected)),
            rest: lexeme.rest,
        },
        Err(_) => ParseResult {
            result: Err("Expected either 'objectives' or 'players'.".to_string()),
            rest: lexeme.rest,
        }
    }   
}

fn objectives<'a>(input: &'a str) -> ParseResult<'a, Objectives, String> {
    let lexeme = alphanumeric(input);
    match lexeme.result {
        Ok(alpha) => match alpha {
            "list" => {
                ParseResult {
                    result: Ok(Objectives::List),
                    rest: lexeme.rest,
                }
            },
            _ => ParseResult {
                result: Err("Unknown command '{}'. Expected 'list'.".to_string()),
                rest: lexeme.rest,
            }
        },
        Err(Some(whitespace)) if whitespace.is_whitespace() => {
            let permissive_result = objectives(input.trim_start());
            ParseResult {
                result: match permissive_result.result {
                    Ok(command) => Err(format!("Expected exactly one space before '{}'.", objectives_command_name(&command))),
                    Err(_) => Err(format!("Expected exactly one space."))
                },
                rest: lexeme.rest,
            }
            
        },
        Err(Some(unexpected)) => ParseResult {
            result: Err(format!("Unexpected symbol '{}'", unexpected)),
            rest: lexeme.rest,
        },
        Err(_) => ParseResult {
            result: Err("Expected one of: 'add', 'list', 'modify', 'remove' or 'setdisplay'.".to_string()),
            rest: lexeme.rest,
        }
    }
}

fn discard<'a>(input: &'a str, expected: char) -> ParseResult<'a, (), Option<char>> {
    match input.chars().next() {
        Some(exp) if exp == expected => ParseResult { result: Ok(()), rest: &input[expected.len_utf8()..]},
        Some(unexpected) => ParseResult { result: Err(Some(unexpected)), rest: &input},
        None => ParseResult { result: Err(None), rest: &input}
    }
}

fn alphanumeric<'a>(input: &'a str) -> ParseResult<'a, &'a str, Option<char>> {
    let mut length = 0;
    for c in input.chars() {
        if c.is_alphanumeric() {
            length += c.len_utf8();
        }
        else {
            break;
        }
    }
    match length {
        0 => ParseResult {
            result: Err(input.chars().next()),
            rest: &input,
        },
        _ => ParseResult {
            result: Ok(&input[0..length]),
            rest: &input[length..],
        }
    }
}

fn is_space_prefixed(input: &str) -> bool {
    match input.len() {
        0 => false,
        _ => &input[0..' '.len_utf8()] == " ",
    }
}

fn scoreboard_command_name(scoreboard: &Scoreboard) -> String {
    match scoreboard {
        Scoreboard::Objectives(_) => "objectives".to_string(),
        Scoreboard::Players(_) => "players".to_string(),
    }
}

fn expected_whitespace<'a>(input: &'a str, c: char) -> String {
    match c {
        c if c.is_whitespace() => 
            match alphanumeric(input.trim_start()).result {
                Ok(after) => format!("Expected exactly one space before '{}'.", after),
                Err(_) => format!("Expected exactly one space.")
            },
        c => format!("Expected exactly one space but got '{}' instead.", c),
    }
}

fn objectives_command_name(objectives: &Objectives) -> String {
    match objectives {
        Objectives::Add(_) => "add".to_string(),
        Objectives::List => "list".to_string(),
        Objectives::Modify(_) => "modify".to_string(),
        Objectives::Remove(_) => "remove".to_string(),
        Objectives::SetDisplay(_) => "setdisplay".to_string(),
    }
}

#[test]
fn scoreboard_objectives_list_normal() {
    let text = "scoreboard objectives list";
    let parse_result = parse_command(text);
    assert_eq!(
        parse_result.result,
        Ok(Some(Command::Scoreboard(Scoreboard::Objectives(
            Objectives::List
        ))))
    );
    assert_eq!(parse_result.rest, "");
}

#[test]
fn scoreboard_objectives_list_whitespace_before() {
    let text = "   scoreboard objectives list";
    let parse_result = parse_command(text);
    assert_eq!(
        parse_result.result,
        Ok(Some(Command::Scoreboard(Scoreboard::Objectives(
            Objectives::List
        ))))
    );
    assert_eq!(parse_result.rest, "");

    let text = "\t\tscoreboard objectives list";
    let parse_result = parse_command(text);
    assert_eq!(
        parse_result.result,
        Ok(Some(Command::Scoreboard(Scoreboard::Objectives(
            Objectives::List
        ))))
    );
    assert_eq!(parse_result.rest, "");
}

#[test]
fn scoreboard_objectives_list_whitespace_middle() {
    let text = "scoreboard  objectives list";
    let parse_result = parse_command(text);
    assert_eq!(
        parse_result.result,
        Err("Expected exactly one space before 'objectives'.".to_string())
    );
    assert_eq!(parse_result.rest, " objectives list");

    let text = "scoreboard objectives   list extra";
    let parse_result = parse_command(text);
    assert_eq!(
        parse_result.result,
        Err("Expected exactly one space before 'list'.".to_string())
    );
    assert_eq!(parse_result.rest, "  list extra");

    let text = "scoreboard\tobjectives list";
    let parse_result = parse_command(text);
    assert_eq!(
        parse_result.result,
        Err("Expected exactly one space before 'objectives'.".to_string())
    );

    let text = "scoreboard objectives\t\tlist";
    let parse_result = parse_command(text);
    assert_eq!(
        parse_result.result,
        Err("Expected exactly one space before 'list'.".to_string())
    );
}