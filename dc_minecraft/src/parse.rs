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
    EOFInstead,
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
        Ok("function") => space(input)
            .and(function_identifier(input))
            .map(Command::Function),
        Ok("execute") => space(input).and(execute(input)).map(Command::Execute),
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
        Ok("get") => space(input).and(players_get(input)).map(Players::Get),
        Ok("list") => players_list(input).map(Players::List),
        Ok("operation") => space(input)
            .and(players_operation(input))
            .map(Players::Operation),
        Ok("remove") => space(input).and(players_remove(input)).map(Players::Remove),
        Ok("reset") => space(input).and(players_reset(input)).map(Players::Reset),
        Ok("set") => space(input).and(players_set(input)).map(Players::Set),
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
        objective,
    })
}

fn players_get(input: &mut Input) -> Result<PlayersGet, Error> {
    let target = target(input)?;
    let objective = space(input).and(identifier(input))?;

    Ok(PlayersGet { target, objective })
}

fn players_list(input: &mut Input) -> Result<PlayersList, Error> {
    let maybe = end_or(input, |input| space(input).and(target(input)))?;

    Ok(PlayersList { target: maybe })
}

fn players_operation(input: &mut Input) -> Result<PlayersOperation, Error> {
    let target2 = target(input)?;
    let target_objective = space(input).and(identifier(input))?;
    let operation = space(input).and(operation_type(input))?;
    let source = space(input).and(target(input))?;
    let source_objective = space(input).and(identifier(input))?;

    Ok(PlayersOperation {
        targets: target2,
        target_objective,
        operation,
        source,
        source_objective,
    })
}

fn operation_type(input: &mut Input) -> Result<OperationType, Error> {
    match operator(input).as_ref().map(String::as_str) {
        Ok("+=") => Ok(OperationType::Addition),
        Ok("-=") => Ok(OperationType::Subtraction),
        Ok("*=") => Ok(OperationType::Multiplication),
        Ok("/=") => Ok(OperationType::Division),
        Ok("%=") => Ok(OperationType::Modulus),
        Ok("=") => Ok(OperationType::Assign),
        Ok("<") => Ok(OperationType::Min),
        Ok(">") => Ok(OperationType::Max),
        Ok("><") => Ok(OperationType::Swap),
        _ => todo!(),
    }
}

fn players_remove(input: &mut Input) -> Result<PlayersRemove, Error> {
    let target = target(input)?;
    let objective = space(input).and(identifier(input))?;
    let score = space(input).and(positive_integer(input))?;

    Ok(PlayersRemove {
        targets: target,
        objective,
        score,
    })
}

fn players_reset(input: &mut Input) -> Result<PlayersReset, Error> {
    let target = target(input)?;
    let objective = space(input).and(identifier(input))?;

    Ok(PlayersReset {
        targets: target,
        objective,
    })
}

fn players_set(input: &mut Input) -> Result<PlayersSet, Error> {
    let target = target(input)?;
    let objective = space(input).and(identifier(input))?;
    let score = space(input).and(signed_integer(input))?;

    Ok(PlayersSet {
        targets: target,
        objective,
        score,
    })
}

fn function_identifier(input: &mut Input) -> Result<FunctionIdentifier, Error> {
    let first = identifier(input)?;
    let second = end_or(input, |input| {
        expect_char(input, ':').and(identifier(input))
    })?;

    match (first, second) {
        (namespace, Some(name)) => Ok(FunctionIdentifier {
            namespace: Some(namespace),
            name,
        }),
        (name, None) => Ok(FunctionIdentifier {
            namespace: None,
            name,
        }),
    }
}

fn execute(input: &mut Input) -> Result<Execute, Error> {
    match identifier(input).as_ref().map(String::as_str) {
        Ok("if") => space(input).and(execute_if(input)).map(Execute::If),
        _ => todo!(),
    }
}

fn execute_if(input: &mut Input) -> Result<If, Error> {
    match identifier(input).as_ref().map(String::as_str) {
        Ok("score") => space(input).and(score(input)).map(If::Score),
        _ => todo!(),
    }
}

fn score(input: &mut Input) -> Result<Score, Error> {
    let target = target(input)?;
    let target_objective = space(input).and(identifier(input))?;

    space(input).and(comparison(input, target, target_objective))
}

fn comparison(input: &mut Input, target: Target, target_objective: String) -> Result<Score, Error> {
    match operator(input).as_ref().map(String::as_str) {
        Ok("<") => space(input)
            .and(source_comparison(input, target, target_objective))
            .map(Score::Less),
        Ok("<=") => space(input)
            .and(source_comparison(input, target, target_objective))
            .map(Score::LessEqual),
        Ok(">") => space(input)
            .and(source_comparison(input, target, target_objective))
            .map(Score::Greater),
        Ok(">=") => space(input)
            .and(source_comparison(input, target, target_objective))
            .map(Score::GreaterEqual),
        Ok("=") => space(input)
            .and(source_comparison(input, target, target_objective))
            .map(Score::Equal),
        _ => todo!(),
    }
}

fn source_comparison(
    input: &mut Input,
    t: Target,
    target_objective: String,
) -> Result<SourceComparison, Error> {
    let source = target(input)?;
    let source_objective = space(input).and(identifier(input))?;
    space(input).and(identifier(input))?;
    let command = space(input).and(command(input))?;

    Ok(SourceComparison {
        target: t,
        target_objective,
        source,
        source_objective,
        command: Box::new(command),
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

fn operator(input: &mut Input) -> Result<String, Error> {
    get_while(input, |c| c != Some(' ') && c != None)
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
        Some(whitespace) if whitespace.is_whitespace() => {
            Err(Error::Space(Space::WhitespaceInstead))
        }
        Some(_) => Err(Error::Space(Space::SymbolInstead)),
        None => Err(Error::Space(Space::EOFInstead)),
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

fn signed_integer(input: &mut Input) -> Result<i32, Error> {
    let integer = get_while(input, |c| {
        c.map(|c| c.is_numeric() || c == '-').unwrap_or(false)
    })?;
    integer.parse().map_err(|_| todo!())
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn scoreboard_players_get() {
        assert_eq!(
            parse_line("scoreboard players get target obj"),
            Ok(Command::Scoreboard(Scoreboard::Players(Players::Get(
                PlayersGet {
                    target: Target::Name("target".to_string()),
                    objective: "obj".to_string()
                }
            ))))
        )
    }

    #[test]
    fn scoreboard_players_list() {
        assert_eq!(
            parse_line("scoreboard players list"),
            Ok(Command::Scoreboard(Scoreboard::Players(Players::List(
                PlayersList { target: None }
            ))))
        );
        assert_eq!(
            parse_line("scoreboard players list target"),
            Ok(Command::Scoreboard(Scoreboard::Players(Players::List(
                PlayersList {
                    target: Some(Target::Name("target".to_string()))
                }
            ))))
        )
    }

    #[test]
    fn scoreboard_players_operation() {
        assert_eq!(
            parse_line("scoreboard players operation target targetObj += source sourceObj"),
            Ok(Command::Scoreboard(Scoreboard::Players(
                Players::Operation(PlayersOperation {
                    targets: Target::Name("target".to_string()),
                    target_objective: "targetObj".to_string(),
                    operation: OperationType::Addition,
                    source: Target::Name("source".to_string()),
                    source_objective: "sourceObj".to_string()
                })
            )))
        );
        assert_eq!(
            parse_line("scoreboard players operation target targetObj -= source sourceObj"),
            Ok(Command::Scoreboard(Scoreboard::Players(
                Players::Operation(PlayersOperation {
                    targets: Target::Name("target".to_string()),
                    target_objective: "targetObj".to_string(),
                    operation: OperationType::Subtraction,
                    source: Target::Name("source".to_string()),
                    source_objective: "sourceObj".to_string()
                })
            )))
        );
        assert_eq!(
            parse_line("scoreboard players operation target targetObj *= source sourceObj"),
            Ok(Command::Scoreboard(Scoreboard::Players(
                Players::Operation(PlayersOperation {
                    targets: Target::Name("target".to_string()),
                    target_objective: "targetObj".to_string(),
                    operation: OperationType::Multiplication,
                    source: Target::Name("source".to_string()),
                    source_objective: "sourceObj".to_string()
                })
            )))
        );
        assert_eq!(
            parse_line("scoreboard players operation target targetObj /= source sourceObj"),
            Ok(Command::Scoreboard(Scoreboard::Players(
                Players::Operation(PlayersOperation {
                    targets: Target::Name("target".to_string()),
                    target_objective: "targetObj".to_string(),
                    operation: OperationType::Division,
                    source: Target::Name("source".to_string()),
                    source_objective: "sourceObj".to_string()
                })
            )))
        );
        assert_eq!(
            parse_line("scoreboard players operation target targetObj %= source sourceObj"),
            Ok(Command::Scoreboard(Scoreboard::Players(
                Players::Operation(PlayersOperation {
                    targets: Target::Name("target".to_string()),
                    target_objective: "targetObj".to_string(),
                    operation: OperationType::Modulus,
                    source: Target::Name("source".to_string()),
                    source_objective: "sourceObj".to_string()
                })
            )))
        );
        assert_eq!(
            parse_line("scoreboard players operation target targetObj = source sourceObj"),
            Ok(Command::Scoreboard(Scoreboard::Players(
                Players::Operation(PlayersOperation {
                    targets: Target::Name("target".to_string()),
                    target_objective: "targetObj".to_string(),
                    operation: OperationType::Assign,
                    source: Target::Name("source".to_string()),
                    source_objective: "sourceObj".to_string()
                })
            )))
        );
        assert_eq!(
            parse_line("scoreboard players operation target targetObj < source sourceObj"),
            Ok(Command::Scoreboard(Scoreboard::Players(
                Players::Operation(PlayersOperation {
                    targets: Target::Name("target".to_string()),
                    target_objective: "targetObj".to_string(),
                    operation: OperationType::Min,
                    source: Target::Name("source".to_string()),
                    source_objective: "sourceObj".to_string()
                })
            )))
        );
        assert_eq!(
            parse_line("scoreboard players operation target targetObj > source sourceObj"),
            Ok(Command::Scoreboard(Scoreboard::Players(
                Players::Operation(PlayersOperation {
                    targets: Target::Name("target".to_string()),
                    target_objective: "targetObj".to_string(),
                    operation: OperationType::Max,
                    source: Target::Name("source".to_string()),
                    source_objective: "sourceObj".to_string()
                })
            )))
        );
        assert_eq!(
            parse_line("scoreboard players operation target targetObj >< source sourceObj"),
            Ok(Command::Scoreboard(Scoreboard::Players(
                Players::Operation(PlayersOperation {
                    targets: Target::Name("target".to_string()),
                    target_objective: "targetObj".to_string(),
                    operation: OperationType::Swap,
                    source: Target::Name("source".to_string()),
                    source_objective: "sourceObj".to_string()
                })
            )))
        );
    }

    #[test]
    fn scoreboard_players_remove() {
        assert_eq!(
            parse_line("scoreboard players remove target targetObj 0"),
            Ok(Command::Scoreboard(Scoreboard::Players(Players::Remove(
                PlayersRemove {
                    targets: Target::Name("target".to_string()),
                    objective: "targetObj".to_string(),
                    score: 0
                }
            ))))
        );
        assert_eq!(
            parse_line("scoreboard players remove target targetObj 1487"),
            Ok(Command::Scoreboard(Scoreboard::Players(Players::Remove(
                PlayersRemove {
                    targets: Target::Name("target".to_string()),
                    objective: "targetObj".to_string(),
                    score: 1487
                }
            ))))
        );
        assert_eq!(
            parse_line("scoreboard players remove target targetObj 2147483647"),
            Ok(Command::Scoreboard(Scoreboard::Players(Players::Remove(
                PlayersRemove {
                    targets: Target::Name("target".to_string()),
                    objective: "targetObj".to_string(),
                    score: 2147483647
                }
            ))))
        );
    }

    #[test]
    fn scoreboard_players_reset() {
        assert_eq!(
            parse_line("scoreboard players reset target obj"),
            Ok(Command::Scoreboard(Scoreboard::Players(Players::Reset(
                PlayersReset {
                    targets: Target::Name("target".to_string()),
                    objective: "obj".to_string()
                }
            ))))
        )
    }

    #[test]
    fn scoreboard_players_set() {
        assert_eq!(
            parse_line("scoreboard players set target obj 1487"),
            Ok(Command::Scoreboard(Scoreboard::Players(Players::Set(
                PlayersSet {
                    targets: Target::Name("target".to_string()),
                    objective: "obj".to_string(),
                    score: 1487
                }
            ))))
        );
        assert_eq!(
            parse_line("scoreboard players set target obj -8700"),
            Ok(Command::Scoreboard(Scoreboard::Players(Players::Set(
                PlayersSet {
                    targets: Target::Name("target".to_string()),
                    objective: "obj".to_string(),
                    score: -8700
                }
            ))))
        )
    }

    #[test]
    fn function() {
        assert_eq!(
            parse_line("function name_space:func_name"),
            Ok(Command::Function(FunctionIdentifier {
                namespace: Some("name_space".to_string()),
                name: "func_name".to_string()
            }))
        );
        assert_eq!(
            parse_line("function func_name"),
            Ok(Command::Function(FunctionIdentifier {
                namespace: None,
                name: "func_name".to_string()
            }))
        )
    }

    #[test]
    fn execute() {
        assert_eq!(
            parse_line("execute if score target targetObj < source sourceObj run scoreboard objectives list"),
            Ok(execute_if_score(Score::Less, "scoreboard objectives list"))
        );
        assert_eq!(
            parse_line("execute if score target targetObj <= source sourceObj run scoreboard objectives add obj dummy"),
            Ok(execute_if_score(Score::LessEqual, "scoreboard objectives add obj dummy"))
        );
        assert_eq!(
            parse_line("execute if score target targetObj > source sourceObj run scoreboard objectives list"),
            Ok(execute_if_score(Score::Greater, "scoreboard objectives list"))
        );
        assert_eq!(
            parse_line("execute if score target targetObj >= source sourceObj run scoreboard objectives list"),
            Ok(execute_if_score(Score::GreaterEqual, "scoreboard objectives list"))
        );
        assert_eq!(
            parse_line("execute if score target targetObj = source sourceObj run scoreboard objectives list"),
            Ok(execute_if_score(Score::Equal, "scoreboard objectives list"))
        );
    }

    fn execute_if_score(
        comparison_type: fn(SourceComparison) -> Score,
        conditional_command: &str,
    ) -> Command {
        let conditional_command = parse_line(conditional_command).unwrap();
        let comparison = SourceComparison {
            target: Target::Name("target".to_string()),
            target_objective: "targetObj".to_string(),
            source: Target::Name("source".to_string()),
            source_objective: "sourceObj".to_string(),
            command: Box::new(conditional_command),
        };
        Command::Execute(Execute::If(If::Score(comparison_type(comparison))))
    }
}
