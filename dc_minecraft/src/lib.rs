#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display, Formatter};

pub mod execute;
pub mod lower;
pub mod parse;

/// See <a href="commands.html">commands</a> for more information
#[derive(Debug, PartialEq)]
pub enum Command {
    Scoreboard(Scoreboard),
    Function(FunctionIdentifier),
    Execute(Execute),
}

#[derive(Debug, PartialEq)]
pub enum Scoreboard {
    Objectives(Objectives),
    Players(Players),
}

#[derive(Debug, PartialEq)]
pub enum Objectives {
    Add(ObjectivesAdd),
    List,
    Modify(ObjectivesModify),
    Remove(ObjectivesRemove),
    SetDisplay(ObjectivesSetDisplay),
}

#[derive(Debug, PartialEq)]
pub enum Criteria {
    Dummy,
}

#[derive(Debug, PartialEq)]
pub struct ObjectivesAdd {
    pub objective: String,
    pub criteria: Criteria,
    pub display_name: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct ObjectivesModify {
    pub objective: String,
    pub modification: Modification,
}

#[derive(Debug, PartialEq)]
pub enum Modification {
    DisplayName(String),
    RenderType(RenderType),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RenderType {
    Hearts,
    Integer,
}

#[derive(Debug, PartialEq)]
pub struct ObjectivesRemove {
    pub objective: String,
}

#[derive(Debug, PartialEq)]
pub struct ObjectivesSetDisplay {
    pub slot: DisplaySlot,
    pub objective: String,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum DisplaySlot {
    BelowName,
    List,
    Sidebar,
}

impl Display for DisplaySlot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DisplaySlot::BelowName => String::from("belowName"),
                DisplaySlot::List => String::from("list"),
                DisplaySlot::Sidebar => String::from("sidebar"),
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Players {
    Add(PlayersAdd),
    Enable(PlayersEnable),
    Get(PlayersGet),
    List(PlayersList),
    Operation(PlayersOperation),
    Remove(PlayersRemove),
    Reset(PlayersReset),
    Set(PlayersSet),
}

#[derive(Debug, PartialEq)]
pub struct PlayersAdd {
    pub targets: Target,
    pub objective: String,
    pub score: i32,
}

#[derive(Debug, PartialEq)]
pub struct PlayersEnable {
    pub targets: Target,
    pub objective: String,
}

#[derive(Debug, PartialEq)]
pub struct PlayersGet {
    pub target: Target,
    pub objective: String,
}

#[derive(Debug, PartialEq)]
pub struct PlayersList {
    pub target: Option<Target>,
}

#[derive(Debug, PartialEq)]
pub struct PlayersOperation {
    targets: Target,
    target_objective: String,
    operation: OperationType,
    source: Target,
    source_objective: String,
}

#[derive(Debug, PartialEq)]
pub enum OperationType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    Assign,
    Min,
    Max,
    Swap,
}

#[derive(Debug, PartialEq)]
pub struct PlayersRemove {
    targets: Target,
    objective: String,
    score: i32,
}

#[derive(Debug, PartialEq)]
pub struct PlayersReset {
    targets: Target,
    objective: String,
}

#[derive(Debug, PartialEq)]
pub struct PlayersSet {
    targets: Target,
    objective: String,
    score: i32,
}

#[derive(Debug, PartialEq)]
pub struct FunctionIdentifier {
    namespace: Option<String>,
    name: String,
}

#[derive(Debug, PartialEq)]
pub enum Target {
    Name(String),
    Selector(Selector),
}

#[derive(Debug, PartialEq)]
pub struct Selector {
    pub variable: SelectorVariable,
}

#[derive(Debug, PartialEq)]
pub enum SelectorVariable {
    P,
    R,
    A,
    E,
    S,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    identifier: FunctionIdentifier,
    commands: Vec<Command>,
}

#[derive(Debug, PartialEq)]
pub enum Execute {
    If(If),
}

#[derive(Debug, PartialEq)]
pub enum If {
    Score(Score),
}

#[derive(Debug, PartialEq)]
pub enum Score {
    Less(SourceComparison),
    LessEqual(SourceComparison),
    Greater(SourceComparison),
    GreaterEqual(SourceComparison),
    Equal(SourceComparison),
    Matches(RangeComparison),
}

#[derive(Debug, PartialEq)]
pub struct SourceComparison {
    target: Target,
    target_objective: String,
    source: Target,
    source_objective: String,
    command: Box<Command>,
}

#[derive(Debug, PartialEq)]
pub struct RangeComparison {
    target: Target,
    target_objective: String,
    interval: Interval,
    command: Box<Command>,
}

#[derive(Debug, PartialEq)]
pub enum Interval {
    Value(i32),
    Bounded(i32, i32),
    LeftUnbounded(i32),
    RightUnbounded(i32),
}
