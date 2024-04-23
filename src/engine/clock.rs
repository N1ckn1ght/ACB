use std::cmp::{max, min};

enum TimeControl {
    Conventional,
    Incremental,
    Deadline 
}

pub struct Clock {
    time:               u128,               // our time
    otim:               u128,               // opfor time
    time_control:       TimeControl,        // type of time control
    mps:                i16,                // moves per base time increment (for TimeControl::Conventional)
    bt:                 u128,               // base time (for TimeControl::Conventional)
    inc:                u128,               // fixed time per move / increment
}

impl Default for Clock {
    fn default() -> Clock {
        Self {
            time:           60000,
            otim:           60000,
            time_control:   TimeControl::Incremental,
            mps:            1,
            bt:             60000,
            inc:            0
        }
    }
}

impl Clock {
    // in fact, board.no is required
    pub fn time_alloc(&mut self, halfmove_counter: i16, is_ponder_on: bool) -> u128 {
        let fullmove_counter = halfmove_counter / 2;
        
        match self.time_control {
            TimeControl::Conventional => {
                let ka = if is_ponder_on {
                    8
                } else {
                    12
                };
                let kb = if is_ponder_on {
                    90
                } else {
                    50
                };
                let fraction = self.mps - fullmove_counter % self.mps;
                let mut alloc = self.time / fraction as u128;
                alloc -= min(alloc / ka, kb + alloc / 100);
                self.time -= alloc;
                return alloc + self.inc;
            },
            TimeControl::Incremental => {
                let divider = 70 - min(fullmove_counter, 10) * 2 - min(fullmove_counter, 20);
                let alloc = self.time / divider as u128;
                self.time -= alloc;
                if self.time > self.inc {
                    return alloc + ((self.inc * 15) >> 4);
                }
                // todo: fix when it's not 2 am
                self.time += alloc;
                let nalloc = self.time - 200;
                self.time -= nalloc;
                return nalloc;
            },
            TimeControl::Deadline => {
                let ka = if is_ponder_on {
                    6
                } else {
                    12
                };
                let kb = if is_ponder_on {
                    140
                } else {
                    50
                };
                return self.inc - min(self.inc / ka, kb + self.inc / 100);
            }
        }
    }

    // ?
    pub fn is_it_time_for_draw(&self) -> i32 {
        match self.time_control {
            TimeControl::Conventional => {
                if self.time < 60000 || self.otim < 60000 {
                    return max(min((i32::try_from(self.otim).unwrap_or(120000) - i32::try_from(self.time).unwrap_or(120000)) / 100, 400), -400);
                } else {
                    return -200;
                }
            },
            TimeControl::Incremental => {
                if self.time < 60000 || self.otim < 60000 {
                    return max(min((i32::try_from(self.otim).unwrap_or(120000) - i32::try_from(self.time).unwrap_or(120000)) / 100, 400), -400);
                } else {
                    return -200;
                }
            },
            TimeControl::Deadline => {
                return 0;
            },
        }
    }

    /* Chess Engine Communication Protocol (XBoard) */

    pub fn level(&mut self, mps: &str, btr: &str, inc: &str) {
        self.mps = mps.parse::<i16>().unwrap();

        let bts = btr.split(":").collect::<Vec<&str>>();
        let mut bt = bts[0].parse::<u128>().unwrap() * 60 * 1000; 
        if bts.len() > 1 {
            bt += bts[1].parse::<u128>().unwrap() * 1000;
        }
        self.bt = bt;

        self.inc = inc.parse::<u128>().unwrap() * 1000;

        if self.mps == 0 {
            self.time_control = TimeControl::Incremental;
        } else {
            self.time_control = TimeControl::Conventional;
        }
        
        self.time = bt;
        self.otim = bt;
    }

    pub fn otim(&mut self, time: &str) {
        self.otim = time.parse::<u128>().unwrap() * 10;
    }

    pub fn st(&mut self, time: &str) {
        self.time_control = TimeControl::Deadline;
        self.inc = time.parse::<u128>().unwrap() * 1000;
        
        self.time = self.inc;
        self.otim = self.inc;
    }

    pub fn time(&mut self, time: &str) {
        self.time = time.parse::<u128>().unwrap() * 10;
    }
}