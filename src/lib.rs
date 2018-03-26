#![feature(getpid)]

extern crate time;
#[macro_use]
extern crate serde_derive;

extern crate serde;
#[macro_use]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<serde_json::Value>,
}

impl<'a> TraceEvent<'a> {
    fn new(name: &'a str, event_type: EventType, args: Option<serde_json::Value>) -> Self {
        TraceEvent {
            name,
            ph: event_type,
            ts: time::precise_time_ns(),
            pid: process::id(),
            tid: thread_id::get(),
            dur: None,
            args,
        }
    }
}

struct EventGuard<'a> {
    event: Option<TraceEvent<'a>>,
}

impl<'a> EventGuard<'a> {
    fn new_trace_on(name: &'a str, args: Option<serde_json::Value>) -> EventGuard<'a> {
        EventGuard {
            event: Some(TraceEvent::new(name, EventType::Complete, args)),
        }
    }

    fn new_trace_off() -> EventGuard<'a> {
        EventGuard { event: None }
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
        let _guard = if $crate::TRACE.is_some(){
            $crate::EventGuard::new_trace_on($name, None)
        }else{
            $crate::EventGuard::new_trace_off()
        };
    };
    ($name: expr, $($json:tt)+) =>{
        let _guard = if $crate::TRACE.is_some(){
            $crate::EventGuard::new_trace_on($name, Some(json!({$($json)+})))
        }else{
            $crate::EventGuard::new_trace_off()
        };
    }
}

#[macro_export]
macro_rules! trace_fn {
    ($name: expr, $function: expr) => {
        if $crate::TRACE.is_some() {
            let mut event = $crate::TraceEvent::new($name, $crate::EventType::Complete, None);
            let result = $function;
            event.dur = Some($crate::time::precise_time_ns() - event.ts);
            $crate::print_trace_event(&event);
            result
        }else{
            $function
        }
    };
    ($name: expr, $function: expr, $($json:tt)+) =>{
        if $crate::TRACE.is_some() {
            let mut event = $crate::TraceEvent::new($name, $crate::EventType::Complete, Some(json!({$($json)+})));
            let result = $function;
            event.dur = Some($crate::time::precise_time_ns() - event.ts);
            $crate::print_trace_event(&event);
            result
        }else{
            $function
        }
    }
}

#[macro_export]
macro_rules! trace_begin {
    ($name: expr) => {
        if $crate::TRACE.is_some() {
            let event = $crate::TraceEvent::new($name, $crate::EventType::DurationBegin, None);
            $crate::print_trace_event(&event);
        }
    };
    ($name: expr, $($json:tt)+) =>{
        if $crate::TRACE.is_some() {
            let event = $crate::TraceEvent::new($name, $crate::EventType::DurationBegin, Some(json!({$($json)+})));
            $crate::print_trace_event(&event);
        }
    }
}

#[macro_export]
macro_rules! trace_end {
    ($name: expr) => {
        if $crate::TRACE.is_some() {
            let event = $crate::TraceEvent::new($name, $crate::EventType::DurationEnd, None);
            $crate::print_trace_event(&event);
        }
    };
    ($name: expr, $($json:tt)+) =>{
        if $crate::TRACE.is_some() {
            let event = $crate::TraceEvent::new($name, $crate::EventType::DurationEnd, Some(json!({$($json)+})));
            $crate::print_trace_event(&event);
        }
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

#[cfg(test)]
mod tests {

    fn trace_duration(name: &str) -> u32 {
        trace_begin!(name,"code": 200,"success": true,);
        trace_end!(name);
        42
    }

    #[test]
    fn test_scoped_trace() {
        trace_scoped!("complete");
        {
            let resut = trace_fn!("trace_fn", trace_duration("trace_fn_fn"),"code":100);
            assert_eq!(resut, 42);
            trace_duration("duration");
        }
    }
}
