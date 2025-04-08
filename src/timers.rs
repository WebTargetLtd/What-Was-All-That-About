/*!
 * # Timers Utilities Module
 *
 * This module provides utility structures and macros for measuring and managing time durations
 * in the application. It is designed to help track execution times, calculate rates, and manage
 * multiple timers efficiently.
 *
 * ## Features
 * - **Timer Struct**: Represents a single timer with start and end times, and provides methods to calculate the duration.
 * - **Timers Struct**: A collection of named timers stored in a `HashMap`, allowing multiple timers to be managed simultaneously.
 * - **Macros**:
 *   - **TC!**: Creates a new `Timers` instance with a specified timer name.
 *   - **TE!**: Ends a timer and calculates its duration.
 *   - **TA!**: Adds a new timer to an existing `Timers` instance.
 *   - **TR!**: Calculates the rate (e.g., operations per second) for a timer.
 *   - **TD!**: Retrieves the duration of a timer.
 *
 * ## Dependencies
 * - `chrono`: Used for date and time operations, such as tracking start and end times.
 * - `std::collections::HashMap`: Used to store and manage multiple timers.
 *
 * ## Usage
 * This module is intended for applications that require precise timing and performance measurement,
 * such as benchmarking, logging, or monitoring utilities.
 *
 * ## Structs
 * ### Timer
 * - Tracks the start and end times of an operation.
 * - Provides methods:
 *   - `new()`: Creates a new timer with the current time as the start and end time.
 *   - `end()`: Updates the end time to the current time.
 *   - `duration()`: Calculates the duration in milliseconds between the start and end times.
 *
 * ### Timers
 * - Manages multiple timers using a `HashMap`.
 * - Provides methods:
 *   - `new(timer_name: String)`: Creates a new `Timers` instance with a single timer.
 *   - `rate(timer_name: String, qty: i64)`: Calculates the rate (e.g., operations per second) for a timer.
 *   - `duration(timer_name: String)`: Retrieves the duration of a timer.
 *   - `end(timer_name: &String)`: Ends a timer and returns its duration.
 *
 * ## Macros
 * - **TC!**: Shorthand for creating a `Timers` instance.
 * - **TE!**: Shorthand for ending a timer and retrieving its duration.
 * - **TA!**: Shorthand for adding a new timer to an existing `Timers` instance.
 * - **TR!**: Shorthand for calculating the rate of operations for a timer.
 * - **TD!**: Shorthand for retrieving the duration of a timer.
 *
 */

use chrono::{DateTime, Local};
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Timer {
    pub starttime: DateTime<Local>,
    pub endtime: DateTime<Local>,
}
impl Timer {
    pub fn duration(&self) -> i64 {
        // If the timer hasn't been updated, get the current time
        if self.endtime == self.starttime {
            (Local::now() - self.starttime).num_milliseconds()
        } else {
            (self.endtime - self.starttime).num_milliseconds()
        }
    }

    pub fn end(&mut self) {
        self.endtime = Local::now()
    }
    pub fn new() -> Self {
        let _time = Local::now();
        Self {
            starttime: _time,
            endtime: _time,
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive()]
pub struct Timers {
    pub timer: HashMap<String, Timer>,
}
impl Timers {
    pub fn new(timer_name: String) -> Self {
        let new_timer: Timer = Timer::new();
        let mut _map: HashMap<String, Timer> = HashMap::new();
        _map.insert(timer_name, new_timer);
        Self { timer: _map }
    }

    pub fn rate(&mut self, timer_name: String, qty: i64) -> i64 {
        let mut _timer = self.timer.get(&timer_name);
        if let Some(_timer) = _timer {
            // println!("Qty is {}", &qty);
            let _duration: i64 = if (_timer.duration()) == 0 {
                1
            } else {
                _timer.duration()
            };
            qty * 1000 / _duration
        } else {
            -1
        }
    }

    pub fn duration(&mut self, timer_name: String) -> i64 {
        let mut _timer = self.timer.get(&timer_name);
        if let Some(_timer) = _timer {
            _timer.duration()
        } else {
            -1
        }
    }

    pub fn end(&mut self, timer_name: &String) -> i64 {
        let mut _timer = self.timer.get_mut(timer_name);
        if let Some(_timer) = _timer {
            _timer.end();
            _timer.duration()
        } else {
            -1
        }
     
    }
}
impl Default for Timers {
    fn default() -> Self {
        Self {
            timer: HashMap::new(),
        }
    }
}
// Shorthand for creating a Timer.
#[macro_export]
macro_rules! TC {
    ($e:expr) => { wolves_cli_helper::timers::Timers::new(String::from($e)) };
}

// Shorthand for ending a Timer.
#[macro_export]
macro_rules! TE {
    ($name:expr, $var:expr) => {
        $var.end(&String::from($name))
    }
}

#[macro_export]
macro_rules! TA {
    ($timer:expr, $timerobject:expr) => {
        $timerobject.add($timer.to_string())
    };
}
#[macro_export]
macro_rules! TR {
    ($timer:expr, $timerobject:expr, $count:expr) => {
        $timerobject.rate($timer.to_string(), $count)
    };
}
#[macro_export]
macro_rules! TD {
    ($timer:expr, $timerobject:expr) => {
        $timerobject.duration($timer.to_string())
    };
}
