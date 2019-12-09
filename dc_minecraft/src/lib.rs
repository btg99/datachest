#![allow(dead_code)]

pub mod execute;
pub mod lower;

/// See <a href="commands.html">commands</a> for more information
pub enum Command {
    Scoreboard(Scoreboard),
    Function(Function),
}

pub enum Scoreboard {
    Objectives(Objectives),
    Players(Players),
}

pub enum Objectives {
    Add(ObjectivesAdd),
    List,
    Modify(ObjectivesModify),
    Remove(ObjectivesRemove),
    SetDisplay(ObjectivesSetDisplay),
}

pub enum Criteria {
    Dummy,
}

pub struct ObjectivesAdd {
    pub objective: String,
    pub criteria: Criteria,
    pub display_name: Option<String>,
}

pub struct ObjectivesModify {
    pub objective: String,
    pub modification: Modification,
}

pub enum Modification {
    DisplayName(String),
    RenderType(RenderType),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RenderType {
    Hearts,
    Integers,
}

pub struct ObjectivesRemove {
    pub objective: String,
}

pub struct ObjectivesSetDisplay {
    pub slot: DisplaySlot,
    pub objective: String,
}

pub enum DisplaySlot {
    BelowName,
    List,
    Sidebar,
}

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

pub struct PlayersAdd {
    pub targets: Selector,
    pub objective: String,
    pub score: u32,
}

pub struct PlayersEnable {
    pub targets: Selector,
    pub objective: String,
}

pub struct PlayersGet {
    pub target: Selector,
    pub objective: String,
}

pub struct PlayersList {
    pub target: Option<Selector>,
}

pub struct PlayersOperation {
    targets: Selector,
    target_objective: String,
    operation: OperationType,
    source: Selector,
    source_objective: String,
}

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

pub struct PlayersRemove {
    targets: Selector,
    objective: String,
    score: u32,
}

pub struct PlayersReset {
    targets: Selector,
    objective: String,
}

pub struct PlayersSet {
    targets: Selector,
    objective: String,
    score: i32,
}

pub struct Function {
    namespace: Option<String>,
    name: String,
}

pub struct Selector {
    pub variable: SelectorVariable,
}

pub enum SelectorVariable {
    P,
    R,
    A,
    E,
    S,
}
