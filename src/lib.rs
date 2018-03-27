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

use std::io;
use std::io::Write;
use std::process;

lazy_static! {
    pub static ref TRACE: Option<&'static str> = option_env!("RS_TRACING");
}

#[derive(Serialize)]
pub enum EventType {
    #[serde(rename = "B")]
    DurationBegin,
    #[serde(rename = "E")]
    DurationEnd,
    #[serde(rename = "X")]
    Complete,
}

#[derive(Serialize)]
pub struct TraceEvent<'a> {
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
    pub fn new(name: &'a str, event_type: EventType, args: Option<serde_json::Value>) -> Self {
        TraceEvent {
            name,
            ph: event_type,
            ts: precise_time_ms(),
            pid: process::id(),
            tid: thread_id::get(),
            dur: None,
            args,
        }
    }
}

pub struct EventGuard<'a> {
    event: TraceEvent<'a>,
}

impl<'a> EventGuard<'a> {
    pub fn new(name: &'a str, args: Option<serde_json::Value>) -> EventGuard<'a> {
        EventGuard {
            event: TraceEvent::new(name, EventType::Complete, args),
        }
    }
}

impl<'a> Drop for EventGuard<'a> {
    pub fn drop(&mut self) {
        self.event.dur = Some(precise_time_ms() - self.event.ts);
        print_trace_event(&self.event);
    }
}

#[macro_export]
macro_rules! trace_scoped {
    ($name: expr) => {
        let _guard = if $crate::TRACE.is_some(){
            Some($crate::EventGuard::new($name, None))
        }else{
            None
        };
    };
    ($name: expr, $($json:tt)+) =>{
        let _guard = if $crate::TRACE.is_some(){
            Some($crate::EventGuard::new($name, Some(json!({$($json)+}))))
        }else{
            None
        };
    }
}

#[macro_export]
macro_rules! trace_fn {
    ($name: expr, $function: expr) => {
        if $crate::TRACE.is_some() {
            let mut event = $crate::TraceEvent::new($name, $crate::EventType::Complete, None);
            let result = $function;
            event.dur = Some($crate::precise_time_ms() - event.ts);
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
            event.dur = Some($crate::precise_time_ms() - event.ts);
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

pub fn print_trace_event(event: &TraceEvent) {
    let mut json_buffer = Vec::with_capacity(256);
    serde_json::to_writer(&mut json_buffer, event).unwrap();
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    lock.write_all(&json_buffer).unwrap();
    lock.write_all(b",\n").unwrap();
}

pub fn precise_time_ms() -> u64 {
    time::precise_time_ns()/1000_000
}

#[cfg(test)]
mod tests {

    fn trace_duration(name: &str) -> u32 {
        //trace_scoped!("lessComplete");
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
