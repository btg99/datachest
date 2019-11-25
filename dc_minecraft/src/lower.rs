use crate::*;

pub fn lower(command: Command) -> String {
    match command {
        Command::Scoreboard(s) => scoreboard(s),
    }
}

fn scoreboard(scoreboard: Scoreboard) -> String {
    match scoreboard {
        Scoreboard::Objectives(o) => objectives(o),
        Scoreboard::Players(p) => players(p),
    }
}

fn objectives(objectives: Objectives) -> String {
    match objectives {
        Objectives::Add(a) => match &a.display_name {
            Some(display) => format!(
                "scoreboard objectives add {} {} \"{}\"",
                a.objective,
                criteria(a.criteria),
                display,
            ),
            None => format!(
                "scoreboard objectives add {} {}",
                a.objective,
                criteria(a.criteria),
            ),
        },
        Objectives::List => String::from("scoreboard objectives list"),
        Objectives::Modify(m) => match &m.modification {
            Modification::DisplayName(display) => format!(
                "scoreboard objectives modify {} displayname \"{}\"",
                m.objective, display
            ),
            Modification::RenderType(render) => match render {
                RenderType::Hearts => format!(
                    "scoreboard objectives modify {} rendertype hearts",
                    m.objective
                ),
                RenderType::Integers => format!(
                    "scoreboard objectives modify {} rendertype integer",
                    m.objective
                ),
            },
        },
        Objectives::Remove(r) => format!("scoreboard objectives remove {}", r.objective),
        Objectives::SetDisplay(sd) => match &sd.slot {
            DisplaySlot::BelowName => format!(
                "scoreboard objectives setdisplay belowName {}",
                sd.objective
            ),
            DisplaySlot::List => format!("scoreboard objectives setdisplay list {}", sd.objective),
            DisplaySlot::Sidebar => {
                format!("scoreboard objectives setdisplay sidebar {}", sd.objective)
            }
        },
    }
}

fn criteria(criteria: Criteria) -> String {
    match criteria {
        Criteria::Dummy => String::from("dummy"),
    }
}

fn players(players: Players) -> String {
    match players {
        Players::Add(a) => format!(
            "scoreboard players add {} {} {}",
            selector(a.targets),
            a.objective,
            a.score
        ),
        Players::Enable(e) => format!(
            "scoreboard players enable {} {}",
            selector(e.targets),
            e.objective,
        ),
        Players::Get(g) => format!(
            "scoreboard players get {} {}",
            selector(g.target),
            g.objective
        ),
        Players::List(l) => match l.target {
            Some(target) => format!("scoreboard players list {}", selector(target)),
            None => format!("scoreboard players list"),
        },
        Players::Operation(o) => format!(
            "scoreboard players operation {} {} {} {} {}",
            selector(o.targets),
            o.target_objective,
            operation(o.operation),
            selector(o.source),
            o.source_objective
        ),
        Players::Remove(r) => format!(
            "scoreboard players remove {} {} {}",
            selector(r.targets),
            r.objective,
            r.score
        ),
        Players::Reset(r) => format!(
            "scoreboard players reset {} {}",
            selector(r.targets),
            r.objective
        ),
        Players::Set(s) => format!(
            "scoreboard players set {} {} {}",
            selector(s.targets),
            s.objective,
            s.score
        ),
    }
}

fn operation(operation_type: OperationType) -> String {
    match operation_type {
        OperationType::Addition => String::from("+="),
        OperationType::Subtraction => String::from("-="),
        OperationType::Multiplication => String::from("*="),
        OperationType::Division => String::from("/="),
        OperationType::Modulus => String::from("%="),
        OperationType::Assign => String::from("="),
        OperationType::Min => String::from("<"),
        OperationType::Max => String::from(">"),
        OperationType::Swap => String::from("><"),
    }
}

fn selector(selector: Selector) -> String {
    match selector.variable {
        SelectorVariable::P => String::from("@p"),
        SelectorVariable::R => String::from("@r"),
        SelectorVariable::A => String::from("@a"),
        SelectorVariable::E => String::from("@e"),
        SelectorVariable::S => String::from("@s"),
    }
}

#[test]
fn scoreboard_objectives_add_no_display() {
    let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
        objective: String::from("objective"),
        criteria: Criteria::Dummy,
        display_name: None,
    })));

    assert_eq!(
        lower(command),
        String::from("scoreboard objectives add objective dummy")
    );
}

#[test]
fn scoreboard_objectives_add_with_display() {
    let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
        objective: String::from("objective"),
        criteria: Criteria::Dummy,
        display_name: Some(String::from("display name")),
    })));

    assert_eq!(
        lower(command),
        String::from("scoreboard objectives add objective dummy \"display name\"")
    );
}

#[test]
fn scoreboard_objectives_list() {
    let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::List));

    assert_eq!(lower(command), String::from("scoreboard objectives list"))
}

#[test]
fn scoreboard_objectives_modify_displayname() {
    let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Modify(
        ObjectivesModify {
            objective: String::from("objective"),
            modification: Modification::DisplayName(String::from("new display name")),
        },
    )));

    assert_eq!(
        lower(command),
        String::from("scoreboard objectives modify objective displayname \"new display name\"")
    );
}

#[test]
fn scoreboard_objectives_modify_rendertype_hearts() {
    let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Modify(
        ObjectivesModify {
            objective: String::from("objective"),
            modification: Modification::RenderType(RenderType::Hearts),
        },
    )));

    assert_eq!(
        lower(command),
        String::from("scoreboard objectives modify objective rendertype hearts")
    );
}

#[test]
fn scoreboard_objectives_modify_rendertype_integer() {
    let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Modify(
        ObjectivesModify {
            objective: String::from("objective"),
            modification: Modification::RenderType(RenderType::Integers),
        },
    )));

    assert_eq!(
        lower(command),
        String::from("scoreboard objectives modify objective rendertype integer")
    );
}

#[test]
fn scoreboard_objectives_remove() {
    let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Remove(
        ObjectivesRemove {
            objective: String::from("objective"),
        },
    )));

    assert_eq!(
        lower(command),
        String::from("scoreboard objectives remove objective")
    );
}

#[test]
fn scoreboard_objectives_setdisplay_belowname() {
    let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::SetDisplay(
        ObjectivesSetDisplay {
            slot: DisplaySlot::BelowName,
            objective: String::from("objective"),
        },
    )));

    assert_eq!(
        lower(command),
        String::from("scoreboard objectives setdisplay belowName objective")
    );
}

#[test]
fn scoreboard_objectives_setdisplay_list() {
    let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::SetDisplay(
        ObjectivesSetDisplay {
            slot: DisplaySlot::List,
            objective: String::from("objective"),
        },
    )));

    assert_eq!(
        lower(command),
        String::from("scoreboard objectives setdisplay list objective")
    );
}

#[test]
fn scoreboard_objectives_setdisplay_sidebar() {
    let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::SetDisplay(
        ObjectivesSetDisplay {
            slot: DisplaySlot::Sidebar,
            objective: String::from("objective"),
        },
    )));

    assert_eq!(
        lower(command),
        String::from("scoreboard objectives setdisplay sidebar objective")
    );
}

#[test]
fn scoreboard_players_add() {
    let command = Command::Scoreboard(Scoreboard::Players(Players::Add(PlayersAdd {
        targets: Selector {
            variable: SelectorVariable::A,
        },
        objective: String::from("obj"),
        score: 17,
    })));

    assert_eq!(
        lower(command),
        String::from("scoreboard players add @a obj 17")
    )
}

#[test]
fn scoreboard_players_enable() {
    let command = Command::Scoreboard(Scoreboard::Players(Players::Enable(PlayersEnable {
        targets: Selector {
            variable: SelectorVariable::E,
        },
        objective: String::from("obj"),
    })));

    assert_eq!(
        lower(command),
        String::from("scoreboard players enable @e obj")
    );
}

#[test]
fn scoreboard_players_get() {
    let command = Command::Scoreboard(Scoreboard::Players(Players::Get(PlayersGet {
        target: Selector {
            variable: SelectorVariable::P,
        },
        objective: String::from("obj"),
    })));

    assert_eq!(
        lower(command),
        String::from("scoreboard players get @p obj")
    )
}

#[test]
fn selector_simple() {
    assert_eq!(
        selector(Selector {
            variable: SelectorVariable::P
        }),
        String::from("@p")
    );
    assert_eq!(
        selector(Selector {
            variable: SelectorVariable::R
        }),
        String::from("@r")
    );
    assert_eq!(
        selector(Selector {
            variable: SelectorVariable::A
        }),
        String::from("@a")
    );
    assert_eq!(
        selector(Selector {
            variable: SelectorVariable::E
        }),
        String::from("@e")
    );
    assert_eq!(
        selector(Selector {
            variable: SelectorVariable::S
        }),
        String::from("@s")
    );
}

#[test]
fn scoreboard_players_list_no_target() {
    let command = Command::Scoreboard(Scoreboard::Players(Players::List(PlayersList {
        target: None,
    })));

    assert_eq!(lower(command), String::from("scoreboard players list"));
}

#[test]
fn scoreboard_players_list_with_target() {
    let command = Command::Scoreboard(Scoreboard::Players(Players::List(PlayersList {
        target: Some(Selector {
            variable: SelectorVariable::R,
        }),
    })));

    assert_eq!(lower(command), String::from("scoreboard players list @r"));
}

#[test]
fn scoreboard_players_operation() {
    assert_eq!(
        lower(generic_player_operation(OperationType::Addition)),
        String::from("scoreboard players operation @a targetObj += @p sourceObj")
    );
    assert_eq!(
        lower(generic_player_operation(OperationType::Subtraction)),
        String::from("scoreboard players operation @a targetObj -= @p sourceObj")
    );
    assert_eq!(
        lower(generic_player_operation(OperationType::Multiplication)),
        String::from("scoreboard players operation @a targetObj *= @p sourceObj"),
    );
    assert_eq!(
        lower(generic_player_operation(OperationType::Division)),
        String::from("scoreboard players operation @a targetObj /= @p sourceObj")
    );
    assert_eq!(
        lower(generic_player_operation(OperationType::Modulus)),
        String::from("scoreboard players operation @a targetObj %= @p sourceObj")
    );
    assert_eq!(
        lower(generic_player_operation(OperationType::Assign)),
        String::from("scoreboard players operation @a targetObj = @p sourceObj")
    );
    assert_eq!(
        lower(generic_player_operation(OperationType::Min)),
        String::from("scoreboard players operation @a targetObj < @p sourceObj")
    );
    assert_eq!(
        lower(generic_player_operation(OperationType::Max)),
        String::from("scoreboard players operation @a targetObj > @p sourceObj")
    );
    assert_eq!(
        lower(generic_player_operation(OperationType::Swap)),
        String::from("scoreboard players operation @a targetObj >< @p sourceObj")
    );
}

#[test]
fn scoreboard_players_remove() {
    let command = Command::Scoreboard(Scoreboard::Players(Players::Remove(PlayersRemove {
        targets: Selector {
            variable: SelectorVariable::E,
        },
        objective: String::from("obj"),
        score: 19,
    })));

    assert_eq!(
        lower(command),
        String::from("scoreboard players remove @e obj 19")
    )
}

#[test]
fn scoreboard_players_reset() {
    let command = Command::Scoreboard(Scoreboard::Players(Players::Reset(PlayersReset {
        targets: Selector {
            variable: SelectorVariable::R,
        },
        objective: String::from("obj"),
    })));

    assert_eq!(
        lower(command),
        String::from("scoreboard players reset @r obj")
    );
}

#[test]
fn scoreboard_players_set() {
    let command = Command::Scoreboard(Scoreboard::Players(Players::Set(PlayersSet {
        targets: Selector {
            variable: SelectorVariable::P,
        },
        objective: String::from("obj"),
        score: -27,
    })));

    assert_eq!(
        lower(command),
        String::from("scoreboard players set @p obj -27")
    );
}

fn generic_player_operation(operation_type: OperationType) -> Command {
    Command::Scoreboard(Scoreboard::Players(Players::Operation(PlayersOperation {
        targets: Selector {
            variable: SelectorVariable::A,
        },
        target_objective: String::from("targetObj"),
        operation: operation_type,
        source: Selector {
            variable: SelectorVariable::P,
        },
        source_objective: String::from("sourceObj"),
    })))
}
