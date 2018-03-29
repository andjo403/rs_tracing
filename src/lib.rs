#![feature(getpid)]
#![feature(macro_reexport)]

extern crate time;

extern crate serde;
#[macro_reexport(json,json_internal)]
extern crate serde_json;
extern crate thread_id;

#[macro_use]
extern crate lazy_static;

use std::io;
use std::io::Write;
use std::process;
use serde::ser::{Serialize, Serializer, SerializeStruct};


lazy_static! {
    pub static ref TRACE: Option<&'static str> = option_env!("RS_TRACING");
}

pub enum EventType {
    DurationBegin,
    DurationEnd,
    Complete,
}

impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            EventType::DurationBegin => serializer.serialize_unit_variant("EventType", 0, "B"),
            EventType::DurationEnd => serializer.serialize_unit_variant("EventType", 1, "E"),
            EventType::Complete => serializer.serialize_unit_variant("EventType", 2, "X"),
        }
    }
}

pub struct TraceEvent<'a> {
    name: &'a str,
    ph: EventType,
    pub ts: u64,
    pid: u32,
    tid: usize,
    pub dur: Option<u64>,
    args: Option<serde_json::Value>,
}

impl<'a> Serialize for TraceEvent<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut event = serializer.serialize_struct("TraceEvent", 7)?;
        event.serialize_field("name", &self.name)?;
        event.serialize_field("ph", &self.ph)?;
        event.serialize_field("ts", &self.ts)?;
        event.serialize_field("pid", &self.pid)?;
        event.serialize_field("tid", &self.tid)?;
        if let Some(ref dur) =  self.dur {
            event.serialize_field("dur", &dur)?;
        }
        if let Some(ref args) =  self.args {
            event.serialize_field("args", &args)?;
        }
        event.end()
    }
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
    fn drop(&mut self) {
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
        trace_begin!(name);
        trace_end!(name);
        42
    }

    #[test]
    fn test_scoped_trace() {
        trace_scoped!("complete");
        {
            let resut = trace_fn!("trace_fn", trace_duration("trace_fn_fn"));
            assert_eq!(resut, 42);
            trace_duration("duration");
        }
    }
}
