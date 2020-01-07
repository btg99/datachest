use crate::SelectorVariable::P;
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
    match identifier(input).as_ref().map(String::as_str) {
        Ok("objectives") => space(input)
            .and(objectives(input))
            .map(Scoreboard::Objectives),
        Ok("players") => space(input).and(players(input)).map(Scoreboard::Players),
        _ => todo!(),
    }
}

fn objectives(input: &mut Input) -> Result<Objectives, Error> {
    match identifier(input).as_ref().map(String::as_str) {
        Ok("add") => space(input).and(objectives_add(input)).map(Objectives::Add),
        Ok("list") => Ok(Objectives::List),
        Ok("modify") => space(input)
            .and(objectives_modify(input))
            .map(Objectives::Modify),
        Ok("remove") => space(input)
            .and(objectives_remove(input))
            .map(Objectives::Remove),
        Ok("setdisplay") => space(input)
            .and(objectives_set_display(input))
            .map(Objectives::SetDisplay),
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

fn objectives_modify(input: &mut Input) -> Result<ObjectivesModify, Error> {
    let objective = identifier(input)?;
    let modification = space(input).and(modification(input))?;

    Ok(ObjectivesModify {
        objective,
        modification,
    })
}

fn modification(input: &mut Input) -> Result<Modification, Error> {
    match identifier(input).as_ref().map(String::as_str) {
        Ok("displayname") => space(input)
            .and(string(input))
            .map(Modification::DisplayName),
        Ok("rendertype") => space(input)
            .and(rendertype(input))
            .map(Modification::RenderType),
        _ => todo!(),
    }
}

fn rendertype(input: &mut Input) -> Result<RenderType, Error> {
    match identifier(input).as_ref().map(String::as_ref) {
        Ok("hearts") => Ok(RenderType::Hearts),
        Ok("integer") => Ok(RenderType::Integer),
        _ => todo!(),
    }
}

fn objectives_remove(input: &mut Input) -> Result<ObjectivesRemove, Error> {
    let objective = identifier(input)?;

    Ok(ObjectivesRemove { objective })
}

fn objectives_set_display(input: &mut Input) -> Result<ObjectivesSetDisplay, Error> {
    let slot = display_slot(input)?;
    let objective = space(input).and(identifier(input))?;

    Ok(ObjectivesSetDisplay { slot, objective })
}

fn display_slot(input: &mut Input) -> Result<DisplaySlot, Error> {
    match identifier(input).as_ref().map(String::as_ref) {
        Ok("belowName") => Ok(DisplaySlot::BelowName),
        Ok("list") => Ok(DisplaySlot::List),
        Ok("sidebar") => Ok(DisplaySlot::Sidebar),
        _ => todo!(),
    }
}

fn players(input: &mut Input) -> Result<Players, Error> {
    match identifier(input).as_ref().map(String::as_ref) {
        Ok("add") => space(input).and(players_add(input)).map(Players::Add),
        Ok("enable") => space(input).and(players_enable(input)).map(Players::Enable),
        _ => todo!(),
    }
}

fn players_add(input: &mut Input) -> Result<PlayersAdd, Error> {
    let target = target(input)?;
    let objective = space(input).and(identifier(input))?;
    let score = space(input).and(positive_integer(input))?;

    Ok(PlayersAdd {
        targets: target,
        objective,
        score,
    })
}

fn players_enable(input: &mut Input) -> Result<PlayersEnable, Error> {
    let target = target(input)?;
    let objective = space(input).and(identifier(input))?;

    Ok(PlayersEnable {
        targets: target,
        objective
    })
}

fn target(input: &mut Input) -> Result<Target, Error> {
    let name = identifier(input)?;
    Ok(Target::Name(name))
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

fn get_while<F: Fn(Option<char>) -> bool>(
    input: &mut Input,
    condition: F,
) -> Result<String, Error> {
    let mut lexeme = String::new();
    while condition(input.peek()) {
        if let Some(c) = input.advance() {
            lexeme.push(c)
        }
    }
    Ok(lexeme)
}

fn space(input: &mut Input) -> Result<(), Error> {
    expect_char(input, ' ')
}

fn expect_char(input: &mut Input, expected: char) -> Result<(), Error> {
    match input.peek() {
        Some(actual) if actual == expected => {
            input.advance();
            Ok(())
        }
        _ => todo!(),
    }
}

fn end_or<T, F>(input: &mut Input, f: F) -> Result<Option<T>, Error>
where
    F: Fn(&mut Input) -> Result<T, Error>,
{
    match input.peek() {
        Some(_) => f(input).map(Some),
        None => Ok(None),
    }
}

fn positive_integer(input: &mut Input) -> Result<i32, Error> {
    let integer = get_while(input, |c| c.map(char::is_numeric).unwrap_or(false))?;
    integer.parse().map_err(|_| todo!())
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
        parse_line("scoreboard objectives add obj_2 dummy \"display name 2\""),
        Ok(Command::Scoreboard(Scoreboard::Objectives(
            Objectives::Add(ObjectivesAdd {
                objective: "obj_2".to_string(),
                criteria: Criteria::Dummy,
                display_name: Some("display name 2".to_string()),
            })
        )))
    );
}

#[test]
fn scoreboard_objectives_list() {
    assert_eq!(
        parse_line("scoreboard objectives list"),
        Ok(Command::Scoreboard(Scoreboard::Objectives(
            Objectives::List
        )))
    )
}

#[test]
fn scoreboard_objectives_modify() {
    assert_eq!(
        parse_line("scoreboard objectives modify obj displayname \"new name\""),
        Ok(Command::Scoreboard(Scoreboard::Objectives(
            Objectives::Modify(ObjectivesModify {
                objective: "obj".to_string(),
                modification: Modification::DisplayName("new name".to_string())
            })
        )))
    );
    assert_eq!(
        parse_line("scoreboard objectives modify obj rendertype hearts"),
        Ok(Command::Scoreboard(Scoreboard::Objectives(
            Objectives::Modify(ObjectivesModify {
                objective: "obj".to_string(),
                modification: Modification::RenderType(RenderType::Hearts)
            })
        )))
    );
    assert_eq!(
        parse_line("scoreboard objectives modify obj rendertype integer"),
        Ok(Command::Scoreboard(Scoreboard::Objectives(
            Objectives::Modify(ObjectivesModify {
                objective: "obj".to_string(),
                modification: Modification::RenderType(RenderType::Integer)
            })
        )))
    )
}

#[test]
fn scoreboard_objectives_remove() {
    assert_eq!(
        parse_line("scoreboard objectives remove obj"),
        Ok(Command::Scoreboard(Scoreboard::Objectives(
            Objectives::Remove(ObjectivesRemove {
                objective: "obj".to_string(),
            })
        )))
    )
}

#[test]
fn scoreboard_objectives_setdisplay() {
    assert_eq!(
        parse_line("scoreboard objectives setdisplay belowName obj"),
        Ok(Command::Scoreboard(Scoreboard::Objectives(
            Objectives::SetDisplay(ObjectivesSetDisplay {
                slot: DisplaySlot::BelowName,
                objective: "obj".to_string()
            })
        )))
    );
    assert_eq!(
        parse_line("scoreboard objectives setdisplay list obj"),
        Ok(Command::Scoreboard(Scoreboard::Objectives(
            Objectives::SetDisplay(ObjectivesSetDisplay {
                slot: DisplaySlot::List,
                objective: "obj".to_string()
            })
        )))
    );
    assert_eq!(
        parse_line("scoreboard objectives setdisplay sidebar obj"),
        Ok(Command::Scoreboard(Scoreboard::Objectives(
            Objectives::SetDisplay(ObjectivesSetDisplay {
                slot: DisplaySlot::Sidebar,
                objective: "obj".to_string()
            })
        )))
    );
}

#[test]
fn scoreboard_players_add() {
    assert_eq!(
        parse_line("scoreboard players add target obj 10958"),
        Ok(Command::Scoreboard(Scoreboard::Players(Players::Add(
            PlayersAdd {
                targets: Target::Name("target".to_string()),
                objective: "obj".to_string(),
                score: 10958
            }
        ))))
    );
    assert_eq!(
        parse_line("scoreboard players add target obj 0"),
        Ok(Command::Scoreboard(Scoreboard::Players(Players::Add(
            PlayersAdd {
                targets: Target::Name("target".to_string()),
                objective: "obj".to_string(),
                score: 0
            }
        ))))
    );
    assert_eq!(
        parse_line("scoreboard players add target obj 2147483647"),
        Ok(Command::Scoreboard(Scoreboard::Players(Players::Add(
            PlayersAdd {
                targets: Target::Name("target".to_string()),
                objective: "obj".to_string(),
                score: 2147483647
            }
        ))))
    );
}

#[test]
fn scoreboard_players_enable() {
    assert_eq!(
        parse_line("scoreboard players enable target obj"),
        Ok(Command::Scoreboard(Scoreboard::Players(Players::Enable(
            PlayersEnable {
                targets: Target::Name("target".to_string()),
                objective: "obj".to_string()
            }
        ))))
    );
}
