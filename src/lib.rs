#[derive(Debug, PartialEq)]
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
    Death,
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

    pub fn update(&mut self, dice: u8) -> (PositionDelta, StateDelta) {
        let mut position_delta;
        let mut state_delta = StateDelta::Nothing;

        position_delta = if self.jumping {
            self.position = dice;
            PositionDelta::Jump(dice)
        } else if self.position + 1 == dice {
            if self.stuck {
                PositionDelta::Stuck(dice)
            } else {
                self.position = dice;
                PositionDelta::Forward(dice)
            }
        } else {
            PositionDelta::Nothing
        };

        self.jumping = false;
        if dice == 5 && self.stuck {
            state_delta = StateDelta::Unstuck;
            self.stuck = false;
        }

        if self.history == dice {
            if !self.shield {
                self.dead = true;
                if dice == 6 {
                    self.cause_of_death = Some(CauseOfDeath::Both);
                } else {
                    self.cause_of_death = Some(CauseOfDeath::Shields);
                }
                return (position_delta, StateDelta::Death);
            }

            self.history = 0;

            match dice {
                1 => {
                    self.shield = false;
                    state_delta = StateDelta::Shield;
                }
                2 => {}
                3 => {
                    self.position = 0;
                    position_delta = PositionDelta::ToZero;
                }
                4 => {
                    self.jumping = true;
                    state_delta = StateDelta::Jumping;
                }
                5 => {
                    self.stuck = true;
                    state_delta = StateDelta::Stuck;
                }
                6 => {
                    self.dead = true;
                    state_delta = StateDelta::Death;
                    self.cause_of_death = Some(CauseOfDeath::Ceres);
                }
                _ => panic!("Invalid input"),
            };
        } else {
            self.history = dice;
        }

        (position_delta, state_delta)
    }

    pub fn victory(&self) -> Option<bool> {
        match (self.dead, self.position) {
            (true, _) => Some(false),
            (false, 6) => Some(true),
            _ => None,
        }
    }
}
