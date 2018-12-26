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
    ToZero,
    Jump(u8),
    Stuck(u8),
}

#[derive(Debug, PartialEq)]
pub enum StateDelta {
    Nothing,
    Shield,
    ToZero,
    Jumping,
    Stuck,
    Unstuck,
    Death(CauseOfDeath),
}

#[derive(Debug)]
struct Delta {
    pos: PositionDelta,
    effect: StateDelta
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

    fn get_changes(&self, dice: u8) -> Delta {
        let mut pos = PositionDelta::Nothing;
        let mut effect = StateDelta::Nothing;

        if self.jumping {
            pos = PositionDelta::Jump(dice);
        } else if self.position + 1 == dice {
            if self.stuck {
                pos = PositionDelta::Stuck(dice);
            } else {
                pos = PositionDelta::Forward(dice);
            }
        }

        if dice == 5 && self.stuck {
            // This won't be overridden
            effect = StateDelta::Unstuck;
        }

        if self.history == dice {
            if !self.shield {
                if dice == 6 {
                    effect = StateDelta::Death(CauseOfDeath::Both);
                } else {
                    effect = StateDelta::Death(CauseOfDeath::Shields);
                }
                return Delta {pos, effect};
            }

            match dice {
                1 => effect = StateDelta::Shield,
                2 => {}
                3 => effect = StateDelta::ToZero,
                4 => effect = StateDelta::Jumping,
                5 => effect = StateDelta::Stuck,
                6 => effect = StateDelta::Death(CauseOfDeath::Ceres),
                _ => panic!("Invalid input"),
            };
        }

        Delta {pos, effect}
    }

    pub fn update(&mut self, dice: u8) -> (PositionDelta, StateDelta) {
        let changes = self.get_changes(dice);

        self.history = dice;
        self.jumping = false;

        match changes.pos {
            PositionDelta::Nothing => {},
            PositionDelta::Forward(x) => self.position = x,
            PositionDelta::Jump(x) => self.position = x,
            PositionDelta::Stuck(_) => {}
            PositionDelta::ToZero => self.position = 0,
        }

        match changes.effect {
            StateDelta::Nothing => {},
            StateDelta::Shield => self.shield = false,
            StateDelta::Jumping => self.jumping = true,
            StateDelta::Stuck => self.stuck = true,
            StateDelta::Unstuck => self.stuck = false,
            StateDelta::ToZero => {},
            StateDelta::Death(cause) => {
                self.dead = true;
                self.cause_of_death = Some(cause);
            },
        }

        if changes.effect != StateDelta::Nothing {
            self.history = 0;
        }

        (changes.pos, changes.effect)
    }

    pub fn victory(&self) -> Option<bool> {
        match (self.dead, self.position) {
            (true, _) => Some(false),
            (false, 6) => Some(true),
            _ => None,
        }
    }
}
