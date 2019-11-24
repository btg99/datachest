#![allow(dead_code)]

pub mod lower;

pub enum Command {
    Scoreboard(Scoreboard),
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

pub struct Selector {
    pub variable: SelectorVariable,
}

pub enum SelectorVariable {
    p,
    r,
    a,
    e,
    s,
}
