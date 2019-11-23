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
        _ => String::new(),
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
