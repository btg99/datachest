use crate::*;
use std::collections::HashMap;

struct Objective {
    display_name: String,
    data: HashMap<String, i32>,
}

#[derive(PartialEq, Debug)]
pub enum Level {
    Info,
    Fail,
}

pub trait Log {
    fn log(&mut self, level: Level, message: &str);
}

pub struct Game {
    objectives: HashMap<String, Objective>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            objectives: HashMap::new(),
        }
    }

    pub fn execute<T: Log>(&mut self, command: &Command, logger: &mut T) {
        match command {
            Command::Scoreboard(s) => self.execute_scoreboard(s, logger),
            _ => {}
        }
    }

    fn execute_scoreboard<T: Log>(&mut self, scoreboard: &Scoreboard, logger: &mut T) {
        match scoreboard {
            Scoreboard::Objectives(o) => self.execute_objectives(o, logger),
            _ => {}
        }
    }

    fn execute_objectives<T: Log>(&mut self, objectives: &Objectives, logger: &mut T) {
        match &objectives {
            Objectives::Add(objectives_add) => self.execute_objectives_add(objectives_add, logger),
            _ => {}
        };
    }

    fn execute_objectives_add<T: Log>(&mut self, objectives_add: &ObjectivesAdd, logger: &mut T) {
        match self.objectives.get(&objectives_add.objective) {
            Some(_) => logger.log(Level::Fail, "An objective already exists by that name"),
            None => {
                let display_name = condense_display_name(
                    &objectives_add.objective,
                    objectives_add.display_name.as_ref().map(String::as_ref),
                );
                self.add_objective(&objectives_add.objective, &display_name);
                logger.log(
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
                data: HashMap::new(),
            },
        );
    }
}

fn condense_display_name(objective_name: &str, display_name: Option<&str>) -> String {
    match display_name {
        Some(name) => String::from(name),
        None => String::from(objective_name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

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
    }

    impl Log for LoggerSpy {
        fn log(&mut self, level: Level, message: &str) {
            self.messages.push_back((level, String::from(message)));
        }
    }

    #[test]
    fn scoreboard_objectives_add_no_display() {
        let command = Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
            objective: String::from("obj"),
            criteria: Criteria::Dummy,
            display_name: None,
        })));
        let mut logger = LoggerSpy::new();
        let mut game = Game::new();
        game.execute(&command, &mut logger);
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
        let mut game = Game::new();
        game.execute(&command, &mut logger);
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
        let mut game = Game::new();
        game.execute(&command, &mut logger);
        game.execute(&command, &mut logger);
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
        let mut game = Game::new();
        game.execute(&command1, &mut logger);
        game.execute(&command2, &mut logger);
        assert!(game.objectives.get("obj1").is_some());
        assert!(game.objectives.get("obj2").is_some());
        logger.assert_logged(Level::Info, "Created new objective [display name]");
        logger.assert_logged(Level::Info, "Created new objective [display name]");
    }
}
