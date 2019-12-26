use crate::*;
use std::cmp;
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

struct Datapack {
    name: String,
    functions: Vec<Function>,
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
    datapack: &'a Option<Datapack>,
    logger: &'a mut T,
}

impl<'a, T: Log> Game<'a, T> {
    pub fn new(logger: &'a mut T) -> Game<'a, T> {
        Game {
            objectives: HashMap::new(),
            displays: HashMap::new(),
            players: HashMap::new(),
            datapack: &None,
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
            Command::Function(f) => self.execute_function(f),
            Command::Execute(e) => self.execute_execute(e),
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
            Players::Remove(r) => self.execute_players_remove(r),
            Players::Set(s) => self.execute_players_set(s),
            Players::Operation(o) => self.execute_players_operation(o),
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
        score: u16,
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
            None => self.logger.log(
                Level::Fail,
                &format!("Unknown scoreboard objective '{}'", objective_name),
            ),
        }
    }

    fn execute_players_remove(&mut self, players_remove: &PlayersRemove) {
        match &players_remove.targets {
            Target::Name(name) => self.execute_players_remove_from_name(
                &name,
                &players_remove.objective,
                players_remove.score,
            ),
            _ => {}
        }
    }

    fn execute_players_remove_from_name(
        &mut self,
        player_name: &str,
        objective_name: &str,
        score: u16,
    ) {
        match &mut self.objectives.get_mut(objective_name) {
            Some(objective) => {
                objective
                    .data
                    .entry(String::from(player_name))
                    .and_modify(|e| *e = (*e).overflowing_sub(score as i32).0)
                    .or_insert(-(score as i32));
                self.logger.log(
                    Level::Info,
                    &format!(
                        "Removed {} from [{}] for {} (now {})",
                        score,
                        objective.display_name,
                        player_name,
                        objective.data.get(player_name).unwrap()
                    ),
                )
            }
            None => self.logger.log(
                Level::Fail,
                &format!("Unknown scoreboard objective '{}'", objective_name),
            ),
        }
    }

    fn execute_players_set(&mut self, players_set: &PlayersSet) {
        match &players_set.targets {
            Target::Name(name) => {
                self.set_player_score(&name, &players_set.objective, players_set.score)
            }
            _ => {}
        }
    }

    fn set_player_score(&mut self, player_name: &str, objective_name: &str, score: i32) {
        match &mut self.objectives.get_mut(objective_name) {
            Some(objective) => {
                objective
                    .data
                    .entry(String::from(player_name))
                    .and_modify(|e| *e = score)
                    .or_insert(score);
                self.logger.log(
                    Level::Info,
                    &format!(
                        "Set [{}] for {} to {}",
                        objective.display_name, player_name, score
                    ),
                )
            }
            None => {}
        }
    }

    fn execute_players_operation(&mut self, players_operation: &PlayersOperation) {
        let source = &self.get_player_names(&players_operation.source)[0];
        let source_score = self.objectives[&players_operation.source_objective].data[source];
        let operation = get_operation(&players_operation.operation);
        for target in self.get_player_names(&players_operation.targets) {
            let target_score = self.objectives[&players_operation.target_objective].data[&target];
            let (a, b) = operation(target_score, source_score);
            self.objectives
                .get_mut(&players_operation.target_objective)
                .unwrap()
                .data
                .entry(target.clone())
                .and_modify(|e| *e = a);
            self.objectives
                .get_mut(&players_operation.source_objective)
                .unwrap()
                .data
                .entry(source.clone())
                .and_modify(|e| *e = b);
            let display_name = &self.objectives[&players_operation.target_objective].display_name;
            self.logger.log(
                Level::Info,
                &format!(
                    "Set [{}] for {} to {}",
                    display_name,
                    &target,
                    self.objectives[&players_operation.target_objective].data[&target]
                ),
            );
        }
    }

    fn get_player_names(&self, target: &Target) -> Vec<String> {
        match target {
            Target::Name(name) => vec![String::from(name)],
            Target::Selector(_) => unimplemented!(),
        }
    }

    fn execute_function(&mut self, function: &FunctionIdentifier) {
        let datapack = self.datapack.as_ref().unwrap();
        let function = datapack
            .functions
            .iter()
            .find(|f| {
                let fi = &f.identifier;
                fi.name == function.name && fi.namespace == function.namespace
            })
            .unwrap()
            .clone();
        for command in &function.commands {
            self.execute(command);
        }
    }

    fn execute_execute(&mut self, execute: &Execute) {
        match execute {
            Execute::If(i) => self.execute_execute_if(i),
        }
    }

    fn execute_execute_if(&mut self, i: &If) {
        match i {
            If::Score(s) => self.execute_execute_if_score(s),
        }
    }

    fn execute_execute_if_score(&mut self, score: &Score) {
        match score {
            Score::Matches(rng_cmp) => self.execute_execute_if_matches(rng_cmp),
            _ => {}
        }
    }

    fn execute_execute_if_matches(&mut self, rng_cmp: &RangeComparison) {
        if self.does_match(
            *self
                .objectives
                .get(&rng_cmp.target_objective)
                .unwrap()
                .data
                .get(&self.get_player_names(&rng_cmp.target)[0])
                .unwrap(),
            &rng_cmp.interval,
        ) {
            self.execute(&rng_cmp.command);
        }
    }

    fn does_match(&self, value: i32, interval: &Interval) -> bool {
        match interval {
            Interval::Value(v) => value == *v,
            Interval::Bounded(a, b) => *a <= value && value <= *b,
            _ => false,
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

fn get_operation(operation_type: &OperationType) -> Box<dyn Fn(i32, i32) -> (i32, i32)> {
    match operation_type {
        OperationType::Addition => Box::new(|a, b| (a + b, b)),
        OperationType::Subtraction => Box::new(|a, b| (a - b, b)),
        OperationType::Multiplication => Box::new(|a, b| (a * b, b)),
        OperationType::Division => Box::new(|a, b| (a / b, b)),
        OperationType::Modulus => Box::new(|a, b| (a % b, b)),
        OperationType::Assign => Box::new(|_, b| (b, b)),
        OperationType::Min => Box::new(|a, b| (cmp::min(a, b), b)),
        OperationType::Max => Box::new(|a, b| (cmp::max(a, b), b)),
        OperationType::Swap => Box::new(|a, b| (b, a)),
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

    #[test]
    fn scoreboard_players_add_nonexistant_objective() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&Command::Scoreboard(Scoreboard::Players(Players::Add(
            PlayersAdd {
                targets: Target::Name(String::from("player")),
                objective: String::from("obj"),
                score: 16,
            },
        ))));
        logger.assert_logged(Level::Fail, "Unknown scoreboard objective 'obj'");
    }

    #[test]
    fn scoreboard_players_remove() {
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
        game.execute(&Command::Scoreboard(Scoreboard::Players(Players::Remove(
            PlayersRemove {
                targets: Target::Name(String::from("player1")),
                objective: String::from("obj"),
                score: 5,
            },
        ))));
        assert_eq!(
            game.objectives
                .get("obj")
                .unwrap()
                .data
                .get("player1")
                .unwrap(),
            &-5
        );
        game.execute(&Command::Scoreboard(Scoreboard::Players(Players::Remove(
            PlayersRemove {
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
            &-9
        );
        logger.skip();
        logger.assert_logged(
            Level::Info,
            "Removed 5 from [display name] for player1 (now -5)",
        );
        logger.assert_logged(
            Level::Info,
            "Removed 4 from [display name] for player1 (now -9)",
        );
    }

    #[test]
    fn scoreboard_players_remove_nonexistant_objective() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&Command::Scoreboard(Scoreboard::Players(Players::Remove(
            PlayersRemove {
                targets: Target::Name(String::from("player")),
                objective: String::from("obj"),
                score: 16,
            },
        ))));
        logger.assert_logged(Level::Fail, "Unknown scoreboard objective 'obj'");
    }

    #[test]
    fn scoreboard_players_set() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        game.execute(&Command::Scoreboard(Scoreboard::Objectives(
            Objectives::Add(ObjectivesAdd {
                objective: String::from("obj"),
                criteria: Criteria::Dummy,
                display_name: Some(String::from("display name")),
            }),
        )));
        game.execute(&Command::Scoreboard(Scoreboard::Players(Players::Set(
            PlayersSet {
                targets: Target::Name(String::from("player")),
                objective: String::from("obj"),
                score: -23,
            },
        ))));
        assert_eq!(game.objectives["obj"].data["player"], -23);
        logger.skip();
        logger.assert_logged(Level::Info, "Set [display name] for player to -23");
    }

    fn operate(
        game: &mut Game<LoggerSpy>,
        target_score: i32,
        source_score: i32,
        operation: OperationType,
    ) {
        game.execute(&Command::Scoreboard(Scoreboard::Objectives(
            Objectives::Add(ObjectivesAdd {
                objective: String::from("obj"),
                criteria: Criteria::Dummy,
                display_name: Some(String::from("display name")),
            }),
        )));
        game.execute(&Command::Scoreboard(Scoreboard::Players(Players::Set(
            PlayersSet {
                targets: Target::Name(String::from("target")),
                objective: String::from("obj"),
                score: target_score,
            },
        ))));
        game.execute(&Command::Scoreboard(Scoreboard::Players(Players::Set(
            PlayersSet {
                targets: Target::Name(String::from("source")),
                objective: String::from("obj"),
                score: source_score,
            },
        ))));
        game.execute(&Command::Scoreboard(Scoreboard::Players(
            Players::Operation(PlayersOperation {
                targets: Target::Name(String::from("target")),
                target_objective: String::from("obj"),
                operation,
                source: Target::Name(String::from("source")),
                source_objective: String::from("obj"),
            }),
        )));
        game.logger.skip();
        game.logger.skip();
        game.logger.skip();
    }

    #[test]
    fn scoreboard_players_operation_addition() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        operate(&mut game, 5, 3, OperationType::Addition);
        assert_eq!(game.objectives["obj"].data["target"], 8);
        logger.assert_logged(Level::Info, "Set [display name] for target to 8");
    }

    #[test]
    fn scoreboard_players_operation_subtraction() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        operate(&mut game, 6, 2, OperationType::Subtraction);
        assert_eq!(game.objectives["obj"].data["target"], 4);
        logger.assert_logged(Level::Info, "Set [display name] for target to 4");
    }

    #[test]
    fn scoreboard_players_operation_multiplication() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        operate(&mut game, 3, 4, OperationType::Multiplication);
        assert_eq!(game.objectives["obj"].data["target"], 12);
        logger.assert_logged(Level::Info, "Set [display name] for target to 12");
    }

    #[test]
    fn scoreboard_players_operation_division() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        operate(&mut game, 14, 3, OperationType::Division);
        assert_eq!(game.objectives["obj"].data["target"], 4);
        logger.assert_logged(Level::Info, "Set [display name] for target to 4");
    }

    #[test]
    fn scoreboard_players_operation_modulus() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        operate(&mut game, 17, 5, OperationType::Modulus);
        assert_eq!(game.objectives["obj"].data["target"], 2);
        logger.assert_logged(Level::Info, "Set [display name] for target to 2");
    }

    #[test]
    fn scoreboard_players_operation_assign() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        operate(&mut game, 12, -5, OperationType::Assign);
        assert_eq!(game.objectives["obj"].data["target"], -5);
        logger.assert_logged(Level::Info, "Set [display name] for target to -5");
    }

    #[test]
    fn scoreboard_players_operation_min_source_less() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        operate(&mut game, 12, -5, OperationType::Min);
        assert_eq!(game.objectives["obj"].data["target"], -5);
        logger.assert_logged(Level::Info, "Set [display name] for target to -5");
    }

    // TODO: Investigate log behavior
    #[test]
    fn scoreboard_players_operation_min_source_more() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        operate(&mut game, 12, 500, OperationType::Min);
        assert_eq!(game.objectives["obj"].data["target"], 12);
    }

    // TODO: Investigate log behavior
    #[test]
    fn scoreboard_players_operation_max_source_less() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        operate(&mut game, 12, -5, OperationType::Max);
        assert_eq!(game.objectives["obj"].data["target"], 12);
    }

    // TODO: Investigate log behavior
    #[test]
    fn scoreboard_players_operation_max_source_more() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        operate(&mut game, 12, 500, OperationType::Max);
        assert_eq!(game.objectives["obj"].data["target"], 500);
    }

    // TODO: Investigate log behavior
    #[test]
    fn scoreboard_players_operation_swap() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        operate(&mut game, -5, 7, OperationType::Swap);
        assert_eq!(game.objectives["obj"].data["target"], 7);
        assert_eq!(game.objectives["obj"].data["source"], -5);
    }

    #[test]
    fn function_with_namespace() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        let datapack = Datapack {
            name: String::from("datapack"),
            functions: vec![Function {
                identifier: FunctionIdentifier {
                    namespace: Some(String::from("namespace")),
                    name: String::from("func"),
                },
                commands: vec![
                    Command::Scoreboard(Scoreboard::Objectives(Objectives::Add(ObjectivesAdd {
                        objective: String::from("obj"),
                        criteria: Criteria::Dummy,
                        display_name: None,
                    }))),
                    Command::Scoreboard(Scoreboard::Players(Players::Add(PlayersAdd {
                        targets: Target::Name(String::from("player")),
                        objective: String::from("obj"),
                        score: 7,
                    }))),
                ],
            }],
        };
        let datapack = Some(datapack);
        game.datapack = &datapack;
        game.execute(&Command::Function(FunctionIdentifier {
            namespace: Some(String::from("namespace")),
            name: String::from("func"),
        }));
        assert_eq!(game.objectives["obj"].data["player"], 7);
    }

    #[test]
    fn execute_if_score_matches_value_no_match() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        compare_match(&mut game, 7, 8, Interval::Value(-55));
        assert_eq!(game.objectives["obj"].data["player"], 7);
    }

    #[test]
    fn execute_if_score_matches_value_matches() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        compare_match(&mut game, 7, 8, Interval::Value(7));
        assert_eq!(game.objectives["obj"].data["player"], 8);
    }

    #[test]
    fn execute_if_score_matches_bounded_range_min() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        compare_match(&mut game, -3, 7, Interval::Bounded(-3, 5));
        assert_eq!(game.objectives["obj"].data["player"], 7);
    }

    #[test]
    fn execute_if_score_matches_bounded_range_max() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        compare_match(&mut game, 5, 7, Interval::Bounded(-3, 5));
        assert_eq!(game.objectives["obj"].data["player"], 7);
    }

    #[test]
    fn execute_if_score_matches_bounded_range_middle() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        compare_match(&mut game, 0, 7, Interval::Bounded(-3, 5));
        assert_eq!(game.objectives["obj"].data["player"], 7);
    }

    #[test]
    fn execute_if_score_matches_bounded_range_not_in_range() {
        let mut logger = LoggerSpy::new();
        let mut game = Game::new(&mut logger);
        compare_match(&mut game, -20, 7, Interval::Bounded(-3, 5));
        assert_eq!(game.objectives["obj"].data["player"], -20);
    }

    fn compare_match<T: Log>(
        game: &mut Game<T>,
        start_score: i32,
        new_score: i32,
        interval: Interval,
    ) {
        game.execute(&Command::Scoreboard(Scoreboard::Objectives(
            Objectives::Add(ObjectivesAdd {
                objective: String::from("obj"),
                criteria: Criteria::Dummy,
                display_name: None,
            }),
        )));
        game.execute(&Command::Scoreboard(Scoreboard::Players(Players::Set(
            PlayersSet {
                objective: String::from("obj"),
                score: start_score,
                targets: Target::Name(String::from("player")),
            },
        ))));
        game.execute(&Command::Execute(Execute::If(If::Score(Score::Matches(
            RangeComparison {
                target: Target::Name(String::from("player")),
                target_objective: String::from("obj"),
                interval,
                command: Box::new(Command::Scoreboard(Scoreboard::Players(Players::Set(
                    PlayersSet {
                        objective: String::from("obj"),
                        score: new_score,
                        targets: Target::Name(String::from("player")),
                    },
                )))),
            },
        )))));
    }
}
