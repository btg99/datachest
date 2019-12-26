use crate::*;

pub fn lower(command: Command) -> String {
    match command {
        Command::Scoreboard(s) => scoreboard(s),
        Command::Function(f) => function(f),
        Command::Execute(e) => execute(e),
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
            target(a.targets),
            a.objective,
            a.score
        ),
        Players::Enable(e) => format!(
            "scoreboard players enable {} {}",
            target(e.targets),
            e.objective,
        ),
        Players::Get(g) => format!(
            "scoreboard players get {} {}",
            target(g.target),
            g.objective
        ),
        Players::List(l) => match l.target {
            Some(t) => format!("scoreboard players list {}", target(t)),
            None => format!("scoreboard players list"),
        },
        Players::Operation(o) => format!(
            "scoreboard players operation {} {} {} {} {}",
            target(o.targets),
            o.target_objective,
            operation(o.operation),
            target(o.source),
            o.source_objective
        ),
        Players::Remove(r) => format!(
            "scoreboard players remove {} {} {}",
            target(r.targets),
            r.objective,
            r.score
        ),
        Players::Reset(r) => format!(
            "scoreboard players reset {} {}",
            target(r.targets),
            r.objective
        ),
        Players::Set(s) => format!(
            "scoreboard players set {} {} {}",
            target(s.targets),
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

fn function(function: FunctionIdentifier) -> String {
    match function.namespace {
        Some(ns) => format!("function {}:{}", ns, function.name),
        None => format!("function {}", function.name),
    }
}

fn target(target: Target) -> String {
    match target {
        Target::Name(n) => n,
        Target::Selector(s) => selector(s),
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

fn execute(execute: Execute) -> String {
    match execute {
        Execute::If(i) => execute_if(i),
    }
}

fn execute_if(i: If) -> String {
    match i {
        If::Score(s) => score(s),
    }
}

fn score(s: Score) -> String {
    match s {
        Score::Less(src_cmp) => source_comparison(src_cmp, "<"),
        Score::LessEqual(src_cmp) => source_comparison(src_cmp, "<="),
        Score::Greater(src_cmp) => source_comparison(src_cmp, ">"),
        Score::GreaterEqual(src_cmp) => source_comparison(src_cmp, ">="),
        Score::Equal(src_cmp) => source_comparison(src_cmp, "="),
        Score::Matches(rng_cmp) => range_comparison(rng_cmp),
    }
}

fn source_comparison(source_comparison: SourceComparison, operation: &str) -> String {
    format!(
        "execute if score {} {} {} {} {} run {}",
        target(source_comparison.target),
        source_comparison.target_objective,
        operation,
        target(source_comparison.source),
        source_comparison.source_objective,
        lower(*source_comparison.command),
    )
}

fn range_comparison(range_comparison: RangeComparison) -> String {
    format!(
        "execute if score {} {} matches {} run {}",
        target(range_comparison.target),
        range_comparison.target_objective,
        interval(range_comparison.interval),
        lower(*range_comparison.command),
    )
}

fn interval(interval: Interval) -> String {
    match interval {
        Interval::Value(v) => format!("{}", v),
        Interval::Bounded(a, b) => format!("{}..{}", a, b),
        Interval::LeftUnbounded(v) => format!("..{}", v),
        Interval::RightUnbounded(v) => format!("{}..", v),
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
        targets: Target::Selector(Selector {
            variable: SelectorVariable::A,
        }),
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
        targets: Target::Selector(Selector {
            variable: SelectorVariable::E,
        }),
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
        target: Target::Selector(Selector {
            variable: SelectorVariable::P,
        }),
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
fn target_name_simple() {
    assert_eq!(
        &target(Target::Name(String::from("playername"))),
        "playername"
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
        target: Some(Target::Selector(Selector {
            variable: SelectorVariable::R,
        })),
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
        targets: Target::Selector(Selector {
            variable: SelectorVariable::E,
        }),
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
        targets: Target::Selector(Selector {
            variable: SelectorVariable::R,
        }),
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
        targets: Target::Selector(Selector {
            variable: SelectorVariable::P,
        }),
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
        targets: Target::Selector(Selector {
            variable: SelectorVariable::A,
        }),
        target_objective: String::from("targetObj"),
        operation: operation_type,
        source: Target::Selector(Selector {
            variable: SelectorVariable::P,
        }),
        source_objective: String::from("sourceObj"),
    })))
}

#[test]
fn function_no_namespace() {
    let command = Command::Function(FunctionIdentifier {
        namespace: None,
        name: String::from("funky"),
    });

    assert_eq!(lower(command), String::from("function funky"));
}

#[test]
fn function_with_namespace() {
    let command = Command::Function(FunctionIdentifier {
        namespace: Some(String::from("namespace")),
        name: String::from("function"),
    });

    assert_eq!(lower(command), String::from("function namespace:function"));
}

#[test]
fn execute_if_score_less() {
    let command = Command::Execute(Execute::If(If::Score(Score::Less(SourceComparison {
        target: Target::Name(String::from("target")),
        target_objective: String::from("target_obj"),
        source: Target::Name(String::from("source")),
        source_objective: String::from("source_obj"),
        command: Box::new(Command::Function(FunctionIdentifier {
            namespace: None,
            name: String::from("conditional_function"),
        })),
    }))));

    assert_eq!(lower(command), String::from("execute if score target target_obj < source source_obj run function conditional_function"));
}

#[test]
fn execute_if_score_less_equal() {
    let command = Command::Execute(Execute::If(If::Score(Score::LessEqual(SourceComparison {
        target: Target::Name(String::from("target")),
        target_objective: String::from("target_obj"),
        source: Target::Name(String::from("source")),
        source_objective: String::from("source_obj"),
        command: Box::new(Command::Function(FunctionIdentifier {
            namespace: None,
            name: String::from("conditional_function"),
        })),
    }))));

    assert_eq!(lower(command), String::from("execute if score target target_obj <= source source_obj run function conditional_function"));
}

#[test]
fn execute_if_score_greater() {
    let command = Command::Execute(Execute::If(If::Score(Score::Greater(SourceComparison {
        target: Target::Name(String::from("target")),
        target_objective: String::from("target_obj"),
        source: Target::Name(String::from("source")),
        source_objective: String::from("source_obj"),
        command: Box::new(Command::Function(FunctionIdentifier {
            namespace: None,
            name: String::from("conditional_function"),
        })),
    }))));

    assert_eq!(lower(command), String::from("execute if score target target_obj > source source_obj run function conditional_function"));
}

#[test]
fn execute_if_score_greater_equal() {
    let command = Command::Execute(Execute::If(If::Score(Score::GreaterEqual(
        SourceComparison {
            target: Target::Name(String::from("target")),
            target_objective: String::from("target_obj"),
            source: Target::Name(String::from("source")),
            source_objective: String::from("source_obj"),
            command: Box::new(Command::Function(FunctionIdentifier {
                namespace: None,
                name: String::from("conditional_function"),
            })),
        },
    ))));

    assert_eq!(lower(command), String::from("execute if score target target_obj >= source source_obj run function conditional_function"));
}

#[test]
fn execute_if_score_equal() {
    let command = Command::Execute(Execute::If(If::Score(Score::Equal(SourceComparison {
        target: Target::Name(String::from("target")),
        target_objective: String::from("target_obj"),
        source: Target::Name(String::from("source")),
        source_objective: String::from("source_obj"),
        command: Box::new(Command::Function(FunctionIdentifier {
            namespace: None,
            name: String::from("conditional_function"),
        })),
    }))));

    assert_eq!(lower(command), String::from("execute if score target target_obj = source source_obj run function conditional_function"));
}

#[test]
fn execute_if_score_matches_value() {
    let command = Command::Execute(Execute::If(If::Score(Score::Matches(RangeComparison {
        target: Target::Name(String::from("target")),
        target_objective: String::from("target_obj"),
        interval: Interval::Value(-23),
        command: Box::new(Command::Function(FunctionIdentifier {
            namespace: None,
            name: String::from("conditional_function"),
        })),
    }))));

    assert_eq!(
        lower(command),
        String::from(
            "execute if score target target_obj matches -23 run function conditional_function"
        )
    );
}

#[test]
fn execute_if_score_matches_bounded_range() {
    let command = Command::Execute(Execute::If(If::Score(Score::Matches(RangeComparison {
        target: Target::Name(String::from("target")),
        target_objective: String::from("target_obj"),
        interval: Interval::Bounded(-23, 52),
        command: Box::new(Command::Function(FunctionIdentifier {
            namespace: None,
            name: String::from("conditional_function"),
        })),
    }))));

    assert_eq!(
        lower(command),
        String::from(
            "execute if score target target_obj matches -23..52 run function conditional_function"
        )
    );
}

#[test]
fn execute_if_score_matches_left_unbounded_range() {
    let command = Command::Execute(Execute::If(If::Score(Score::Matches(RangeComparison {
        target: Target::Name(String::from("target")),
        target_objective: String::from("target_obj"),
        interval: Interval::LeftUnbounded(-7),
        command: Box::new(Command::Function(FunctionIdentifier {
            namespace: None,
            name: String::from("conditional_function"),
        })),
    }))));

    assert_eq!(
        lower(command),
        String::from(
            "execute if score target target_obj matches ..-7 run function conditional_function"
        )
    );
}

#[test]
fn execute_if_score_matches_right_unbounded_range() {
    let command = Command::Execute(Execute::If(If::Score(Score::Matches(RangeComparison {
        target: Target::Name(String::from("target")),
        target_objective: String::from("target_obj"),
        interval: Interval::RightUnbounded(3),
        command: Box::new(Command::Function(FunctionIdentifier {
            namespace: None,
            name: String::from("conditional_function"),
        })),
    }))));

    assert_eq!(
        lower(command),
        String::from(
            "execute if score target target_obj matches 3.. run function conditional_function"
        )
    );
}
