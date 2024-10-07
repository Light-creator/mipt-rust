#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

use crate::RoundOutcome::BothCheated;
use crate::RoundOutcome::BothCooperated;
use crate::RoundOutcome::RightCheated;
use crate::RoundOutcome::LeftCheated;


pub struct Game {
    lastStatus: RoundOutcome,
    left: Box<dyn Agent>,
    right: Box<dyn Agent>,
    left_score: i32,
    right_score: i32,
    left_status: bool,
    right_status: bool
}

impl Game {
    pub fn new(left: Box<dyn Agent>, right: Box<dyn Agent>) -> Self {
        Self {
            lastStatus: BothCooperated,
            left: left,
            right: right,
            left_score: 0,
            right_score: 0,
            left_status: false,
            right_status: false
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left_score
    }

    pub fn right_score(&self) -> i32 {
        self.right_score  
    }

    pub fn play_round(&mut self) -> RoundOutcome {

        match self.lastStatus {
            BothCooperated => {
                self.left_status = self.left.cheated(false);
                self.right_status = self.right.cheated(false);
            },
            BothCheated => {
                self.left_status = self.left.cheated(true);
                self.right_status = self.right.cheated(true);
            },
            LeftCheated => {
                self.left_status = self.left.cheated(false);
                self.right_status = self.right.cheated(true);
            }
            RightCheated => {
                self.left_status = self.left.cheated(true);
                self.right_status = self.right.cheated(false);
            }
        }

        match (self.left_status, self.right_status) {
            (false, false) => {
                self.left_score += 2;
                self.right_score += 2;
                self.lastStatus = BothCooperated;
            },
            (true, true) => {
                self.lastStatus = BothCheated;
            },
            (true, false) => {
                self.left_score += 3;
                self.right_score -= 1;
                self.lastStatus = LeftCheated;
            },
            (false, true) => {
                self.left_score -= 1;
                self.right_score += 3;
                self.lastStatus = RightCheated;
            }
        }

        self.lastStatus
    }
}


pub trait Agent {
    fn cheated(&mut self, _opp_last_cheat: bool) -> bool;
}



////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CheatingAgent {}

impl Agent for CheatingAgent {
    fn cheated(&mut self, _opp_last_cheat: bool) -> bool {
        true
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {}

impl Agent for CooperatingAgent {
    fn cheated(&mut self, _opp_last_cheat: bool) -> bool {
        false
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct GrudgerAgent {
    opp_cheated: bool
}

impl Agent for GrudgerAgent {
    fn cheated(&mut self, _opp_last_cheat: bool) -> bool {
        match (self.opp_cheated, _opp_last_cheat) {
            (false, true) => {
                self.opp_cheated = true;
                true
            },
            (false, false) => {
                false
            },
            (_, _) => {
                self.opp_cheated = true;
                true
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CopycatAgent {}

impl Agent for CopycatAgent {
    fn cheated(&mut self, _opp_last_cheat: bool) -> bool {
        _opp_last_cheat
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct DetectiveAgent {
    round: i32,
    opp_cheated: bool
}

impl Agent for DetectiveAgent {
    fn cheated(&mut self, _opp_last_cheat: bool) -> bool {
        self.round += 1;
        if _opp_last_cheat {
            self.opp_cheated = true;
        }

        match (self.round-1, _opp_last_cheat, self.opp_cheated) {
            (0, _, _) => { false },
            (1, _, _) => { true },
            (2, _, _) => { false },
            (3, _, _) => { false },
            (_, _, false) => { true },
            (_, true, _) => { true },
            (_, false, _) => { false },
        }
    }
}
