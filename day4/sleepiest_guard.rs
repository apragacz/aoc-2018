#![allow(non_snake_case)]
use std::io;
use std::io::prelude::*;
use std::cmp;
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Date {
    y: u32,
    m: u32,
    d: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Time {
    h: u32,
    m: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct DateTime {
    date: Date,
    time: Time,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum EventType {
    BeginsShift,
    FallsAsleep,
    WakesUp,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Event {
    datetime: DateTime,
    event_type: EventType,
    guard_id: Option<u32>,
}

fn split_and_parse(string: &str, ch: char) -> Vec<u32> {
    let segments: Vec<&str> = string.split(ch).collect();
    return segments.iter().map(|x| x.parse().expect("not an integer")).collect();
}

fn parse_date(string: &str) -> Date {
    let numbers = split_and_parse(string, '-');
    match numbers.as_slice() {
        [y, m, d] => Date {y: *y, m: *m, d: *d},
        _ => panic!("not a valid date format"),
    }
}

fn parse_time(string: &str) -> Time {
    let numbers = split_and_parse(string, ':');
    match numbers.as_slice() {
        [h, m] => Time {h: *h, m: *m},
        _ => panic!("not a valid time format"),
    }
}

fn parse_guard_id(string: &str) -> u32 {
    return string.trim_start_matches('#').parse().expect("not an integer");
}

fn main() {
    let mut events = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let segments: Vec<&str> = l.trim().split(' ').collect();
        let date_str: &str = segments[0].trim_start_matches('[');
        let time_str: &str = segments[1].trim_end_matches(']');
        let date = parse_date(date_str);
        let time = parse_time(time_str);
        let datetime = DateTime {date, time};
        let (event_type, guard_id) = match segments.as_slice() {
            [_, _, "Guard", num_segment, "begins", "shift"] => (
                EventType::BeginsShift, Some(parse_guard_id(num_segment))),
            [_, _, "wakes", "up"] => (EventType::WakesUp, None),
            [_, _, "falls", "asleep"] => (EventType::FallsAsleep, None),
            _ => panic!("Unknown event log")
        };
        events.push(Event {datetime, event_type, guard_id});
    }
    events.sort();

    let mut guard_id_opt = None;
    let mut start_min_opt = None;
    let mut guard_counters: HashMap<u32, usize> = HashMap::new();
    let mut guard_sleep_minutes = HashMap::new();

    for event in &events {
        match &event.event_type {
            EventType::BeginsShift => {
                assert!(start_min_opt == None);
                guard_id_opt = event.guard_id;
            },
            EventType::FallsAsleep => {
                assert!(event.datetime.time.h == 0);
                start_min_opt = Some(event.datetime.time.m);
            }
            EventType::WakesUp => {
                assert!(event.datetime.time.h == 0);
                let guard_id = guard_id_opt.unwrap();
                let start_min: usize = start_min_opt.unwrap() as usize;
                let end_min: usize = event.datetime.time.m as usize;

                let counter = guard_counters.entry(guard_id).or_insert(0);
                *counter += end_min - start_min;

                let sleep_minutes = guard_sleep_minutes.entry(guard_id).or_insert([0 as usize; 60]);

                for i in start_min..end_min {
                    sleep_minutes[i] += 1;
                }

                start_min_opt = None;
            }
        }
    }

    let mut sleepiest_guard_id_opt = None;
    let mut sleepiest_guard_time_opt = None;
    for (guard_id, sleep_time) in &guard_counters {
        match sleepiest_guard_time_opt {
            Some(time) => {
                if *sleep_time > time {
                    sleepiest_guard_id_opt = Some(*guard_id);
                    sleepiest_guard_time_opt = Some(*sleep_time);
                }
            },
            None => {
                sleepiest_guard_id_opt = Some(*guard_id);
                sleepiest_guard_time_opt = Some(*sleep_time);
            }
        }
    }

    let sleepiest_guard_id: u32 = sleepiest_guard_id_opt.unwrap();
    let sleepiest_guard_minutes = guard_sleep_minutes.get(&sleepiest_guard_id).unwrap();
    let sleepy_minute_max_agg = sleepiest_guard_minutes.iter().fold(0, |m, el| cmp::max(m, *el));
    let max_sleepy_minute: usize = sleepiest_guard_minutes.iter().enumerate().filter(|(_, x)| **x == sleepy_minute_max_agg).map(|(i, _)| i).next().unwrap();

    println!("{:?}", (sleepiest_guard_id as usize) * max_sleepy_minute);
}
