#![feature(getpid)]

extern crate time;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate thread_id;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::io;
use std::io::Write;
use std::process;

lazy_static! {
    static ref TRACE: Option<String> = env::var("RS_TRACING").ok();
}

#[derive(Serialize)]
enum EventType {
    #[serde(rename = "B")]
    DurationBegin,
    #[serde(rename = "E")]
    DurationEnd,
    #[serde(rename = "X")]
    Complete,
}

#[derive(Serialize)]
struct TraceEvent<'a> {
    name: &'a str,
    ph: EventType,
    ts: u64,
    pid: u32,
    tid: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    dur: Option<u64>,
}

impl<'a> TraceEvent<'a> {
    fn new(name: &'a str, event_type: EventType) -> Self {
        TraceEvent {
            name,
            ph: event_type,
            ts: time::precise_time_ns(),
            pid: process::id(),
            tid: thread_id::get(),
            dur: None,
        }
    }
}

struct EventGuard<'a> {
    event: Option<TraceEvent<'a>>,
}

impl<'a> EventGuard<'a> {
    fn new(name: &'a str) -> EventGuard<'a> {
        if TRACE.is_some() {
            EventGuard {
                event: Some(TraceEvent::new(name, EventType::Complete)),
            }
        } else {
            EventGuard { event: None }
        }
    }
}

impl<'a> Drop for EventGuard<'a> {
    fn drop(&mut self) {
        if let Some(ref mut event) = self.event {
            event.dur = Some(time::precise_time_ns() - event.ts);
            print_trace_event(&event);
        }
    }
}

#[macro_export]
macro_rules! trace_scoped {
    ($name: expr) => {
        let _guard = $crate::EventGuard::new($name);
    };
}

pub fn trace_fn<T, F>(name: &str, function: F) -> T
where
    F: FnOnce() -> T,
{
    trace_scoped!(name);
    function()
}

pub fn trace_begin(name: &str) {
    if TRACE.is_some() {
        trace_duration(name, EventType::DurationBegin)
    }
}

pub fn trace_end(name: &str) {
    if TRACE.is_some() {
        trace_duration(name, EventType::DurationEnd)
    }
}

fn print_trace_event(event: &TraceEvent) {
    let mut json_buffer = Vec::with_capacity(256);
    serde_json::to_writer(&mut json_buffer, event).unwrap();
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    lock.write_all(&json_buffer).unwrap();
    lock.write_all(b",\n").unwrap();
}

fn trace_duration(name: &str, event_type: EventType) {
    let event = TraceEvent::new(name, event_type);
    print_trace_event(&event);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn trace_duration(name: &str) -> u32{
        trace_begin(name);
        trace_end(name);
        42
    }

    #[test]
    fn test_scoped_trace() {
        trace_scoped!("complete");
        {
            let resut = trace_fn("trace_fn", || trace_duration("trace_fn_fn"));
            assert_eq!(resut, 42);
            trace_duration("duration");
        }
    }
}
