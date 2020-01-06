use crate::*;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Error {
    Space(Space),
}

#[derive(Debug, PartialEq)]
pub enum Space {
    WhitespaceInstead,
    SymbolInstead,
}

struct Input<'a> {
    current: i32,
    chars: Peekable<std::str::Chars<'a>>,
}

impl<'a> Input<'a> {
    fn new(text: &'a str) -> Input {
        Input {
            current: 0,
            chars: text.chars().peekable(),
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next();
        self.increment_current();
        c
    }

    fn increment_current(&mut self) {
        self.current += 1;
    }
}

pub fn parse_line(line: &str) -> Result<Command, Error> {
    command(&mut Input::new(line))
}

fn command(input: &mut Input) -> Result<Command, Error> {
    match identifier(input).as_ref().map(String::as_str) {
        Ok("scoreboard") => space(input).and(scoreboard(input)).map(Command::Scoreboard),
        _ => todo!(),
    }
}

fn scoreboard(input: &mut Input) -> Result<Scoreboard, Error> {
    let identifier = identifier(input);
    match &identifier {
        Ok(s) if s == "objectives" => space(input)
            .and(objectives(input))
            .map(Scoreboard::Objectives),
        Ok(other) => panic!(other.clone()),
        _ => todo!()
    }
}

fn objectives(input: &mut Input) -> Result<Objectives, Error> {
    match identifier(input).as_ref().map(String::as_str) {
        Ok("add") => space(input).and(objectives_add(input)).map(Objectives::Add),
        _ => todo!(),
    }
}

fn objectives_add(input: &mut Input) -> Result<ObjectivesAdd, Error> {
    let objective = identifier(input)?;
    let criteria = space(input).and(criteria(input))?;
    let display_name = end_or(input, |input| space(input).and(string(input)))?;

    Ok(ObjectivesAdd {
        objective,
        criteria,
        display_name,
    })
}

fn criteria(input: &mut Input) -> Result<Criteria, Error> {
    match identifier(input).as_ref().map(String::as_str) {
        Ok("dummy") => Ok(Criteria::Dummy),
        _ => todo!(),
    }
}

fn identifier(input: &mut Input) -> Result<String, Error> {
    get_while(input, is_identifier_element)
}

fn is_identifier_element(c: Option<char>) -> bool {
    c.map(|c| c.is_alphanumeric() || c == '_').unwrap_or(false)
}

fn string(input: &mut Input) -> Result<String, Error> {
    expect_char(input, '"')?;
    let content = get_while(input, |c| c != Some('"'));
    expect_char(input, '"')?;
    content
}

fn get_while<F: Fn(Option<char>) -> bool>(input: &mut Input, condition: F) -> Result<String, Error> {
    let mut lexeme = String::new();
    while condition(input.peek()) {
        input.advance().map(|c| lexeme.push(c));
    }
    Ok(lexeme)
}

fn space(input: &mut Input) -> Result<(), Error> {
    expect_char(input, ' ')
}

fn expect_char(input: &mut Input, c: char) -> Result<(), Error> {
    match input.peek() {
        Some(c) => { input.advance(); Ok(()) },
        _ => todo!(),
    }
}

fn end_or<T, F>(input: &mut Input, f: F) -> Result<Option<T>, Error> where F: Fn(&mut Input) -> Result<T, Error> {
    match input.peek() {
        Some(_) => f(input).map(Some),
        None => Ok(None)
    }
}

#[test]
fn scoreboard_objectives_add() {
    assert_eq!(
        parse_line("scoreboard objectives add obj dummy"),
        Ok(Command::Scoreboard(Scoreboard::Objectives(
            Objectives::Add(ObjectivesAdd {
                objective: "obj".to_string(),
                criteria: Criteria::Dummy,
                display_name: None,
            })
        )))
    );
    assert_eq!(
        parse_line("scoreboard objectives add obj_2 dummy \"display_name_2\""),
        Ok(Command::Scoreboard(Scoreboard::Objectives(
            Objectives::Add(ObjectivesAdd {
                objective: "obj_2".to_string(),
                criteria: Criteria::Dummy,
                display_name: Some("display_name_2".to_string()),
            })
        )))
    );
}