use crate::*;

pub fn lower(command: Command) -> String {
    match command {
        Command::Scoreboard(s) => scoreboard(s),
    }
}

fn scoreboard(scoreboard: Scoreboard) -> String {
    match scoreboard {
        Scoreboard::Objectives(o) => objectives(o),
        _ => String::new(),
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
