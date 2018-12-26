#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CauseOfDeath {
    Shields,
    Ceres,
    Both,
}

#[derive(Debug)]
pub enum PositionDelta {
    Nothing,
    Forward(u8),
    Jump(u8),
    Stuck(u8),
    ToZero,
}

#[derive(Debug)]
pub enum StateDelta {
    Nothing,
    Shield,
    Jumping,
    Stuck,
    Unstuck,
    Death(CauseOfDeath),
}

#[derive(Debug)]
enum Delta {
    Pos(PositionDelta),
    State(StateDelta),
    ClearHistory
}

#[derive(Debug)]
pub struct State {
    pub position: u8,
    pub history: u8,
    pub shield: bool,
    pub jumping: bool,
    pub stuck: bool,
    pub dead: bool,
    pub cause_of_death: Option<CauseOfDeath>,
}

impl State {
    pub fn new() -> State {
        State {
            position: 0,
            history: 0,
            shield: true,
            jumping: false,
            stuck: false,
            dead: false,
            cause_of_death: None,
        }
    }

    fn get_changes(&self, dice: u8) -> Vec<Delta> {
        let mut changes = vec![];

        if self.jumping {
            changes.push(Delta::Pos(PositionDelta::Jump(dice)));
        } else if self.position + 1 == dice {
            if self.stuck {
                changes.push(Delta::Pos(PositionDelta::Stuck(dice)));
            } else {
                changes.push(Delta::Pos(PositionDelta::Forward(dice)));
            }
        }

        if dice == 5 && self.stuck {
            changes.push(Delta::State(StateDelta::Unstuck));
        }

        if self.history == dice {
            if !self.shield {
                if dice == 6 {
                    changes.push(Delta::State(StateDelta::Death(CauseOfDeath::Both)));
                } else {
                    changes.push(Delta::State(StateDelta::Death(CauseOfDeath::Shields)));
                }
                return changes;
            }

            match dice {
                1 => changes.push(Delta::State(StateDelta::Shield)),
                2 => {}
                3 => changes.push(Delta::Pos(PositionDelta::ToZero)),
                4 => changes.push(Delta::State(StateDelta::Jumping)),
                5 => changes.push(Delta::State(StateDelta::Stuck)),
                6 => changes.push(Delta::State(StateDelta::Death(CauseOfDeath::Ceres))),
                _ => panic!("Invalid input"),
            };

            changes.push(Delta::ClearHistory);
        }

        changes
    }

    pub fn update(&mut self, dice: u8) -> (PositionDelta, StateDelta) {
        let changes = self.get_changes(dice);

        self.history = dice;
        self.jumping = false;

        for delta in &changes {
            match delta {
                Delta::Pos(PositionDelta::Nothing) => {},
                Delta::Pos(PositionDelta::Forward(x)) => self.position = *x,
                Delta::Pos(PositionDelta::Jump(x)) => self.position = *x,
                Delta::Pos(PositionDelta::Stuck(_)) => {}
                Delta::Pos(PositionDelta::ToZero) => self.position = 0,

                Delta::State(StateDelta::Nothing) => {},
                Delta::State(StateDelta::Shield) => self.shield = false,
                Delta::State(StateDelta::Jumping) => self.jumping = true,
                Delta::State(StateDelta::Stuck) => self.stuck = true,
                Delta::State(StateDelta::Unstuck) => self.stuck = false,
                Delta::State(StateDelta::Death(cause)) => {
                    self.dead = true;
                    self.cause_of_death = Some(*cause);
                },

                Delta::ClearHistory => self.history = 0
            }
        }

        let mut pos_delta = None;
        let mut state_delta = None;
        for x in changes {
            match x {
                Delta::Pos(x) => pos_delta = Some(x),
                Delta::State(x) => state_delta = Some(x),
                _ => {}
            }
        }

        (pos_delta.unwrap_or(PositionDelta::Nothing), state_delta.unwrap_or(StateDelta::Nothing))
    }

    pub fn victory(&self) -> Option<bool> {
        match (self.dead, self.position) {
            (true, _) => Some(false),
            (false, 6) => Some(true),
            _ => None,
        }
    }
}
