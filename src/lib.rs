#[derive(Debug, PartialEq)]
pub enum CauseOfDeath {
    Shields,
    Ceres,
    Both,
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

    pub fn update(&mut self, dice: u8) {
        if self.jumping || (!self.stuck && self.position + 1 == dice) {
            self.position = dice;
        }
        self.jumping = false;
        if dice == 5 {
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
                return;
            }

            match dice {
                1 => self.shield = false,
                2 => {}
                3 => self.position = 0,
                4 => self.jumping = true,
                5 => self.stuck = true,
                6 => {
                    self.dead = true;
                    self.cause_of_death = Some(CauseOfDeath::Ceres);
                }
                _ => panic!("Invalid input"),
            };
            self.history = 0;
        } else {
            self.history = dice;
        }
    }

    pub fn victory(&self) -> Option<bool> {
        match (self.dead, self.position) {
            (true, _) => Some(false),
            (false, 6) => Some(true),
            _ => None,
        }
    }
}
