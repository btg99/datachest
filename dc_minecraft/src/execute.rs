use crate::*;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct Objective {
    display_name: String,
    render_type: RenderType,
    data: HashMap<String, i32>,
}

#[derive(PartialEq, Debug)]
struct Player {
    name: String,
}

#[derive(PartialEq, Debug)]
pub enum Level {
    Info,
    Fail,
}

pub trait Log {
    fn log(&mut self, level: Level, message: &str);
}

pub struct Game<'a, T: Log> {
    objectives: HashMap<String, Objective>,
    displays: HashMap<DisplaySlot, Option<String>>,
    players: HashMap<String, Player>,
    logger: &'a mut T,
}

impl<'a, T: Log> Game<'a, T> {
    pub fn new(logger: &'a mut T) -> Game<'a, T> {
        Game {
            objectives: HashMap::new(),
            displays: HashMap::new(),
            players: HashMap::new(),
            logger,
        }
    }

    pub fn add_player(&mut self, name: &str) {
        self.players.insert(
            String::from(name),
            Player {
                name: String::from(name),
            },
        );
    }

    pub fn execute(&mut self, command: &Command) {
        match command {
            Command::Scoreboard(s) => self.execute_scoreboard(s),
            _ => {}
        }
    }

    fn execute_scoreboard(&mut self, scoreboard: &Scoreboard) {
        match scoreboard {
            Scoreboard::Objectives(o) => self.execute_objectives(o),
            Scoreboard::Players(p) => self.execute_players(p),
        }
    }

    fn execute_objectives(&mut self, objectives: &Objectives) {
        match &objectives {
            Objectives::Add(objectives_add) => self.execute_objectives_add(objectives_add),
            Objectives::List => self.execute_objectives_list(),
            Objectives::Modify(objectives_modify) => {
                self.execute_objectives_modify(objectives_modify)
            }
            Objectives::Remove(objectives_remove) => {
                self.execute_objectives_remove(objectives_remove)
            }
            Objectives::SetDisplay(objectives_set_display) => {
                self.execute_objectives_set_display(objectives_set_display)
            }
        };
    }

    fn execute_objectives_add(&mut self, objectives_add: &ObjectivesAdd) {
        match self.objectives.get(&objectives_add.objective) {
            Some(_) => self
                .logger
                .log(Level::Fail, "An objective already exists by that name"),
            None => {
                let display_name = condense_display_name(
                    &objectives_add.objective,
                    objectives_add.display_name.as_ref().map(String::as_ref),
                );
                self.add_objective(&objectives_add.objective, &display_name);
                self.logger.log(
                    Level::Info,
                    &format!("Created new objective [{}]", display_name),
                )
            }
        }
    }

    fn add_objective(&mut self, objective_name: &str, display_name: &str) {
        self.objectives.insert(
            String::from(objective_name),
            Objective {
                display_name: String::from(display_name),
                render_type: RenderType::Integers,
                data: HashMap::new(),
            },
        );
    }

    fn execute_objectives_list(&mut self) {
        match self.objectives.len() {
            0 => self.logger.log(Level::Info, "There are no objectives"),
            n => self.logger.log(
                Level::Info,
                &format!(
                    "There are {} objectives:{}",
                    n,
                    space_separate(self.objectives.values().map(|o| &o.display_name))
                ),
            ),
        }
    }

    fn execute_objectives_modify(&mut self, objective_modify: &ObjectivesModify) {
        match &objective_modify.modification {
            Modification::DisplayName(new_display_name) => self
                .execute_objectives_modify_display_name(
                    &objective_modify.objective,
                    &new_display_name,
                ),
            Modification::RenderType(new_render_type) => self
                .execute_objectives_modify_render_type(
                    &objective_modify.objective,
                    *new_render_type,
                ),
        }
    }

    fn execute_objectives_modify_display_name(
        &mut self,
        objective_name: &str,
        new_display_name: &str,
    ) {
        match &mut self.objectives.get_mut(objective_name) {
            Some(objective) => {
                if objective.display_name != new_display_name {
                    objective.display_name = String::from(new_display_name);
                    self.logger.log(
                        Level::Info,
                        &format!(
                            "Changed objective {} display name to [{}]",
                            objective_name, new_display_name
                        ),
                    );
                }
            }
            None => self.logger.log(
                Level::Fail,
                &format!("Unknown scoreboard objective '{}'", objective_name),
            ),
        }
    }

    fn execute_objectives_modify_render_type(
        &mut self,
        objective_name: &str,
        new_render_type: RenderType,
    ) {
        match &mut self.objectives.get_mut(objective_name) {
            Some(objective) => {
                if objective.render_type != new_render_type {
                    objective.render_type = new_render_type;
                    self.logger.log(
                        Level::Info,
                        &format!(
                            "Changed objective [{}] render type",
                            &objective.display_name
                        ),
                    );
                }
            }
            None => self.logger.log(
                Level::Fail,
                &format!("Unknown scoreboard objective '{}'", &objective_name),
            ),
        }
    }

    fn execute_objectives_remove(&mut self, objectives_remove: &ObjectivesRemove) {
        match self.objectives.remove(&objectives_remove.objective) {
            Some(objective) => self.logger.log(
                Level::Info,
                &format!("Removed objective [{}]", &objective.display_name),
            ),
            None => self.logger.log(
                Level::Fail,
                &format!(
                    "Unknown scoreboard objective '{}'",
                    &objectives_remove.objective
                ),
            ),
        }
    }

    fn execute_objectives_set_display(&mut self, objectives_set_display: &ObjectivesSetDisplay) {
        match &mut self.objectives.get_mut(&objectives_set_display.objective) {
            Some(objective) => {
                if slot_contains(
                    self.displays.get(&objectives_set_display.slot),
                    &objectives_set_display.objective,
                ) {
                    self.logger.log(
                        Level::Fail,
                        "Nothing changed. That display slot is already showing that objective",
                    );
                } else {
                    self.displays.insert(
                        objectives_set_display.slot,
                        Some(objectives_set_display.objective.clone()),
                    );
                    self.logger.log(
                        Level::Info,
                        &format!(
                            "Set display slot {} to show objective {}",
                            objectives_set_display.slot, &objective.display_name
                        ),
                    )
                }
            }
            None => self.logger.log(
                Level::Fail,
                &format!(
                    "Unknown scoreboard objective '{}'",
                    &objectives_set_display.objective
                ),
            ),
        }
    }

    fn execute_players(&mut self, players: &Players) {
        match players {
            Players::Add(a) => self.execute_players_add(a),
            _ => {}
        }
    }

    fn execute_players_add(&mut self, players_add: &PlayersAdd) {
        match &players_add.targets {
            Target::Name(name) => {
                self.execute_players_add_from_name(&name, &players_add.objective, players_add.score)
            }
            _ => {}
        }
    }

    fn execute_players_add_from_name(
        &mut self,
        player_name: &str,
        objective_name: &str,
        score: u32,
    ) {
        match &mut self.objectives.get_mut(objective_name) {
            Some(objective) => {
                objective
                    .data
                    .entry(String::from(player_name))
                    .and_modify(|e| *e = (*e).overflowing_add(score as i32).0)
                    .or_insert(score as i32);
                self.logger.log(
                    Level::Info,
                    &format!(
                        "Added {} to [{}] for {} (now {})",
                        score,
                        objective.display_name,
                        player_name,
                        objective.data.get(player_name).unwrap()
                    ),
                );
            }
            None => {}
        }
    }
}

fn condense_display_name(objective_name: &str, display_name: Option<&str>) -> String {
    match display_name {
        Some(name) => String::from(name),
        None => String::from(objective_name),
    }
}

fn space_separate<'a, Iter: Iterator<Item = &'a String>>(strings: Iter) -> String {
    let mut output = String::new();
    strings.for_each(|s| output.push_str(&format!(" [{}]", s)));
    output
}

fn slot_contains(slot: Option<&Option<String>>, value: &str) -> bool {
    match slot {
        Some(wrapper) => match wrapper {
            Some(x) => x == value,
            None => false,
        },
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::collections::VecDeque;
    use std::hash::Hash;

    struct LoggerSpy {
        messages: VecDeque<(Level, String)>,
    }

    impl LoggerSpy {
        fn new() -> LoggerSpy {
            LoggerSpy {
                messages: VecDeque::new(),
            }
        }

        fn assert_logged(&mut self, level: Level, message: &str) {
            match self.messages.pop_front() {
                Some(msg) => {
                    assert_eq!(level, msg.0);
                    assert_eq!(message, &msg.1);
                }
                None => assert!(false),
            }
        }

        fn assert_matches<F>(&mut self, condition: F)
        where
            F: Fn(Level, &str) -> bool,
        {
            match self.messages.pop_front() {
                Some(msg) => assert!(condition(msg.0, &msg.1)),
                None => assert!(false),
            }
        }

        fn assert_no_logs(&mut self) {
            assert!(self.messages.is_empty());
        }

        fn skip(&mut self) {
            self.messages.pop_front();
        }
    }

    impl Log for LoggerSpy {
        fn log(&mut self, level: Level, message: &str) {
            self.messages.push_back((level, String::from(message)));
        }
    }

    fn is_anagram<T>(a: Vec<T>, b: Vec<T>) -> bool
    where
        T: Hash + Eq,
    {
        let mut counts: HashMap<&T, i32> = HashMap::new();
        for item in &a {
            counts.entry(item).and_modify(|e| *e += 1).or_insert(1);
        }
        for item in &b {
            counts.entry(item).and_modify(|e| *e -= 1).or_insert(-1);
        }
        counts.values().all(|v| *v == 0)
    }

    #[test]
    fn scoreboard_objectives_add_no_display() {
        let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
            objective: String::from("obj"),
            criteria: Criteria::Dummy,
            display_name: None,
        })));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&command);
        assert!(game.objectives.get("obj").is_some());
        logger.assert_logged(Level::Info, "Created new objective [obj]");
    }

    #[test]
    fn scoreboard_objectives_add_with_display() {
        let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
            objective: String::from("obj"),
            criteria: Criteria::Dummy,
            display_name: Some(String::from("obj name")),
        })));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&command);
        assert!(game.objectives.get("obj").is_some());
        logger.assert_logged(Level::Info, "Created new objective [obj name]");
    }

    #[test]
    fn scoreboard_objectives_add_twice() {
        let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
            objective: String::from("obj"),
            criteria: Criteria::Dummy,
            display_name: Some(String::from("obj name")),
        })));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&command);
        game.execute(&command);
        assert!(game.objectives.get("obj").is_some());
        logger.assert_logged(Level::Info, "Created new objective [obj name]");
        logger.assert_logged(Level::Fail, "An objective already exists by that name");
    }

    #[test]
    fn scoreboard_objectives_add_same_display() {
        let command1 =
            Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
                objective: String::from("obj1"),
                criteria: Criteria::Dummy,
                display_name: Some(String::from("display name")),
            })));
        let command2 =
            Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
                objective: String::from("obj2"),
                criteria: Criteria::Dummy,
                display_name: Some(String::from("display name")),
            })));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&command1);
        game.execute(&command2);
        assert!(game.objectives.get("obj1").is_some());
        assert!(game.objectives.get("obj2").is_some());
        logger.assert_logged(Level::Info, "Created new objective [display name]");
        logger.assert_logged(Level::Info, "Created new objective [display name]");
    }

    #[test]
    fn scoreboard_objectives_list_0_objectives() {
        let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::List));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&command);
        logger.assert_logged(Level::Info, "There are no objectives");
    }

    #[test]
    fn scoreboard_objectives_list_1_objective() {
        let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::List));
        let add_command =
            Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
                objective: String::from("obj"),
                criteria: Criteria::Dummy,
                display_name: Some(String::from("obj name")),
            })));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&add_command);
        game.execute(&command);
        logger.skip();
        logger.assert_logged(Level::Info, "There are 1 objectives: [obj name]");
    }

    #[test]
    fn scoreboard_objectives_list_many_objectives() {
        let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::List));
        let add_first =
            Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
                objective: String::from("obj1"),
                criteria: Criteria::Dummy,
                display_name: None,
            })));
        let add_second =
            Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
                objective: String::from("obj2"),
                criteria: Criteria::Dummy,
                display_name: None,
            })));
        let add_third =
            Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
                objective: String::from("obj3"),
                criteria: Criteria::Dummy,
                display_name: None,
            })));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&add_first);
        game.execute(&add_second);
        game.execute(&add_third);
        game.execute(&command);
        logger.skip();
        logger.skip();
        logger.skip();
        logger.assert_matches(|level, message: &str| {
            level == Level::Info
                && message.starts_with("There are 3 objectives: ")
                && is_anagram(
                    message[(message.find(":").unwrap() + 2)..]
                        .split(" ")
                        .collect(),
                    vec!["[obj1]", "[obj2]", "[obj3]"],
                )
        });
    }

    #[test]
    fn scoreboard_objectives_modify_display_name() {
        let add = Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
            objective: String::from("obj"),
            criteria: Criteria::Dummy,
            display_name: Some(String::from("prev display name")),
        })));
        let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Modify(
            ObjectivesModify {
                objective: String::from("obj"),
                modification: Modification::DisplayName(String::from("new display name")),
            },
        )));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&add);
        game.execute(&command);
        assert_eq!(
            game.objectives.get("obj").unwrap().display_name,
            "new display name".to_owned()
        );
        logger.skip();
        logger.assert_logged(
            Level::Info,
            "Changed objective obj display name to [new display name]",
        );
    }

    #[test]
    fn scoreboard_objectives_modify_display_name_no_objective() {
        let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Modify(
            ObjectivesModify {
                objective: String::from("obj"),
                modification: Modification::DisplayName(String::from("new display name")),
            },
        )));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&command);
        logger.assert_logged(Level::Fail, "Unknown scoreboard objective 'obj'");
    }

    #[test]
    fn scoreboard_objectives_modify_display_name_no_change() {
        let add = Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
            objective: String::from("obj"),
            criteria: Criteria::Dummy,
            display_name: Some(String::from("display name")),
        })));
        let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Modify(
            ObjectivesModify {
                objective: String::from("obj"),
                modification: Modification::DisplayName(String::from("display name")),
            },
        )));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&add);
        game.execute(&command);
        logger.skip();
        logger.assert_no_logs();
    }

    #[test]
    fn scoreboard_objectives_remove_existing() {
        let add = Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
            objective: String::from("obj"),
            criteria: Criteria::Dummy,
            display_name: Some(String::from("display name")),
        })));
        let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Remove(
            ObjectivesRemove {
                objective: String::from("obj"),
            },
        )));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&add);
        game.execute(&command);
        assert!(game.objectives.get("obj").is_none());
        logger.skip();
        logger.assert_logged(Level::Info, "Removed objective [display name]");
    }

    #[test]
    fn scoreboard_objectives_remove_nothing() {
        let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Remove(
            ObjectivesRemove {
                objective: String::from("obj"),
            },
        )));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&command);
        logger.assert_logged(Level::Fail, "Unknown scoreboard objective 'obj'");
    }

    #[test]
    fn initial_render_type() {
        let add = Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
            objective: String::from("obj"),
            criteria: Criteria::Dummy,
            display_name: None,
        })));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&add);
        assert_eq!(
            game.objectives.get("obj").unwrap().render_type,
            RenderType::Integers
        );
    }

    #[test]
    fn scoreboard_objectives_modify_render_type() {
        let add = Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
            objective: String::from("obj"),
            criteria: Criteria::Dummy,
            display_name: Some(String::from("display name")),
        })));
        let modify = Command::Scoreboard(Scoreboard::Objectives(Objectives::Modify(
            ObjectivesModify {
                objective: String::from("obj"),
                modification: Modification::RenderType(RenderType::Hearts),
            },
        )));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&add);
        game.execute(&modify);
        assert_eq!(
            game.objectives.get("obj").unwrap().render_type,
            RenderType::Hearts
        );
        logger.skip();
        logger.assert_logged(Level::Info, "Changed objective [display name] render type");
    }

    #[test]
    fn scoreboard_objectives_modify_render_type_no_objective() {
        let modify = Command::Scoreboard(Scoreboard::Objectives(Objectives::Modify(
            ObjectivesModify {
                objective: String::from("obj"),
                modification: Modification::RenderType(RenderType::Integers),
            },
        )));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&modify);
        logger.assert_logged(Level::Fail, "Unknown scoreboard objective 'obj'");
    }

    #[test]
    fn scoreboard_objectives_modify_render_type_no_change() {
        let add = Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
            objective: String::from("obj"),
            criteria: Criteria::Dummy,
            display_name: Some(String::from("display name")),
        })));
        let modify = Command::Scoreboard(Scoreboard::Objectives(Objectives::Modify(
            ObjectivesModify {
                objective: String::from("obj"),
                modification: Modification::RenderType(RenderType::Integers),
            },
        )));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&add);
        game.execute(&modify);
        logger.skip();
        logger.assert_no_logs();
    }

    #[test]
    fn scoreboard_objectives_set_display() {
        let add = Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
            objective: String::from("obj"),
            criteria: Criteria::Dummy,
            display_name: Some(String::from("display name")),
        })));
        let set_display = Command::Scoreboard(Scoreboard::Objectives(Objectives::SetDisplay(
            ObjectivesSetDisplay {
                slot: DisplaySlot::Sidebar,
                objective: String::from("obj"),
            },
        )));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&add);
        game.execute(&set_display);
        assert_eq!(
            game.displays.get(&DisplaySlot::Sidebar).unwrap(),
            &Some(String::from("obj"))
        );
        logger.skip();
        logger.assert_logged(
            Level::Info,
            "Set display slot sidebar to show objective display name",
        );
    }

    #[test]
    fn scoreboard_objectives_set_display_no_objective() {
        let set_display = Command::Scoreboard(Scoreboard::Objectives(Objectives::SetDisplay(
            ObjectivesSetDisplay {
                slot: DisplaySlot::BelowName,
                objective: String::from("obj"),
            },
        )));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&set_display);
        logger.assert_logged(Level::Fail, "Unknown scoreboard objective 'obj'");
    }

    #[test]
    fn scoreboard_objectives_set_display_twice() {
        let add = Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
            objective: String::from("obj"),
            criteria: Criteria::Dummy,
            display_name: Some(String::from("display name")),
        })));
        let set_display = Command::Scoreboard(Scoreboard::Objectives(Objectives::SetDisplay(
            ObjectivesSetDisplay {
                slot: DisplaySlot::Sidebar,
                objective: String::from("obj"),
            },
        )));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&add);
        game.execute(&set_display);
        game.execute(&set_display);
        logger.skip();
        logger.skip();
        logger.assert_logged(
            Level::Fail,
            "Nothing changed. That display slot is already showing that objective",
        );
    }

    #[test]
    fn add_player_to_game() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.add_player("player1");
        assert_eq!(
            game.players.get("player1").unwrap(),
            &Player {
                name: String::from("player1")
            }
        );
    }

    #[test]
    fn scoreboard_players_add_player_not_ingame() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.add_player("player1");
        game.execute(&Command::Scoreboard(Scoreboard::Objectives(
            Objectives::Add(ObjectivesAdd {
                objective: String::from("obj"),
                criteria: Criteria::Dummy,
                display_name: Some(String::from("display name")),
            }),
        )));
        game.execute(&Command::Scoreboard(Scoreboard::Players(Players::Add(
            PlayersAdd {
                targets: Target::Name(String::from("player1")),
                objective: String::from("obj"),
                score: 11,
            },
        ))));
        assert_eq!(
            game.objectives
                .get("obj")
                .unwrap()
                .data
                .get("player1")
                .unwrap(),
            &11
        );
        game.execute(&Command::Scoreboard(Scoreboard::Players(Players::Add(
            PlayersAdd {
                targets: Target::Name(String::from("player1")),
                objective: String::from("obj"),
                score: 4,
            },
        ))));
        assert_eq!(
            game.objectives
                .get("obj")
                .unwrap()
                .data
                .get("player1")
                .unwrap(),
            &15
        );
        logger.skip();
        logger.assert_logged(
            Level::Info,
            "Added 11 to [display name] for player1 (now 11)",
        );
        logger.assert_logged(
            Level::Info,
            "Added 4 to [display name] for player1 (now 15)",
        );
    }
}
