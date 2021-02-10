/*
* Copyright (C) 2020, Miklos Maroti
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

//! A uniform way to monitor the progress of a computation

use std::collections::HashMap;
use std::sync::Mutex;
use std::thread::{sleep, spawn};
use std::time::Duration;

/// Struct to hold all monitored variables and their value.
#[derive(Default)]
struct Monitor {
    running: bool,
    elapsed: u64,
    vars: HashMap<&'static str, u64>,
}

lazy_static! {
    /// The single static instance of the monitor struct.
    static ref MONITOR: Mutex<Monitor> = Default::default();
}

/// Worker function that is spawned within a thread to
/// print out the value of monitored variables.
fn worker() {
    #[cfg(not(test))]
    eprintln!("progress: monitoring thread started");
    loop {
        const SECS: u64 = 10;
        sleep(Duration::from_secs(SECS));

        let mut monitor = MONITOR.lock().unwrap();
        monitor.elapsed += SECS;
        let mut result = format!("progress: time={}s", monitor.elapsed);
        for (name, value) in &monitor.vars {
            result = format!("{}, {}={}", &result, name, value);
        }

        if result.is_empty() {
            monitor.running = false;
            break;
        } else {
            drop(monitor);
            #[cfg(not(test))]
            eprintln!("{}", &result);
        }
    }
    #[cfg(not(test))]
    eprintln!("progress: monitoring thread stopped");
}

/// Creates a new monitored value. If this is the first monitored value,
/// then a worker thread will be started.
pub fn add_progress(name: &'static str) {
    let mut monitor = MONITOR.lock().unwrap();
    monitor.vars.insert(name, 0);
    if !monitor.running {
        monitor.running = true;
        spawn(worker);
    }
}

/// Removes the monitored value. If this was the last value to be
///  monitored, then the worker thread will be stopped.
pub fn del_progress(name: &'static str) {
    let mut monitor = MONITOR.lock().unwrap();
    monitor.vars.remove(name);
}

/// Sets the value for the given monitored variable.
pub fn set_progress(name: &'static str, value: u64) {
    let mut monitor = MONITOR.lock().unwrap();
    if let Some(val) = monitor.vars.get_mut(name) {
        *val = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress() {
        add_progress("test");
        set_progress("test", 10);
        del_progress("test");
    }
}
