//! Asteroid roulette game lbrary. The aim of the game is to get from position 0 to position 6, and
//! dodge deadly asteroids. See [the game update function](struct.State.html#method.update) for
//! details.

/// Various causes of death.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CauseOfDeath {
    /// The player got hit by an asteroid without having shields.
    Shields,
    /// The player got hit by Ceres (that is, threw 6 twice).
    Ceres,
    /// The player hit by Ceres without having shields.
    Both,
}

/// Changes to the position of the player.
#[derive(Debug)]
pub enum PositionDelta {
    /// Nothing happened.
    Nothing,
    /// The player moved forward to ```u8``` [1..6]
    Forward(u8),
    /// The player was moved to position 0.
    ToZero,
    /// The player teleported to ```u8``` [1..6]
    Jump(u8),
    /// The would have moved to ```u8``` [1..6], but is stuck and didn't move.
    Stuck(u8),
}

/// Changes to the state of the player.
#[derive(Debug, PartialEq)]
pub enum StateDelta {
    /// Nothing happened.
    Nothing,
    /// The player lost shields.
    Shield,
    /// The player was moved to position 0.
    ToZero,
    /// The next throw will be a teleport.
    Jumping,
    /// The player is stuck.
    Stuck,
    /// The player is no longer stuck.
    Unstuck,
    /// The player was killed by ```CauseOfDeath```.
    Death(CauseOfDeath),
}

#[derive(Debug)]
struct Delta {
    pos: PositionDelta,
    effect: StateDelta
}

/// Current state of the game.
#[derive(Debug)]
pub struct State {
    /// Current position of the player, in range [0, 6].
    pub position: u8,
    /// Previous dice throw, or 0 if none. In range [0..6].
    pub history: u8,
    /// Shield status, if true, the player has shields up.
    pub shield: bool,
    /// If true, next throw will be a teleport instead of a normal progression.
    pub jumping: bool,
    /// If true, the player can only progress by teleporting.
    pub stuck: bool,
    /// If true, the player is dead.
    pub dead: bool,
    /// If the player is dead, this contains the cause of death.
    pub cause_of_death: Option<CauseOfDeath>,
}

impl State {
    /// Start a new game.
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

    /* Get changes to the game state. Check State.update() for details. */
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
            // This is always true, so...
            assert!(self.history != 5);
            // ...this won't be overridden
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

    /// Update the state according to the game rules:
    /// 1. If the player is teleporting, move the player to the position specified by the dice. 
    /// 2. Else, if the player is not stuck and the dice is one bigger than the player position,
    /// move the player forward.
    /// 3. If the player is stuck and the dice throw equals 5, the player is no longer stuck.
    /// 4. If the dice throw is equal to the previous throw, the player is hit by an asteroid.
    /// If the player doesn't have shields, the player dies. Otherwise, the effect is determined
    /// by the dice:
    ///     1. The player loses shields.
    ///     2. Nothing happens.
    ///     3. The player is moved to position 0.
    ///     4. The next throw will teleport the player.
    ///     5. The player is stuck.
    ///     6. The player dies instantly.
    ///
    /// Returns the changes to the position and state.
    pub fn update(&mut self, dice: u8) -> (PositionDelta, StateDelta) {
        let changes = self.get_changes(dice);
        if dice < 1 || dice > 6 {
            panic!("Invalid dice number.");
        }

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

    /// Returns ```Some(true)``` if the player won, and ```Some(false)``` if the player died.
    /// Returns ```None``` if the game is still running. 
    pub fn victory(&self) -> Option<bool> {
        match (self.dead, self.position) {
            (true, _) => Some(false),
            (false, 6) => Some(true),
            _ => None,
        }
    }
}
