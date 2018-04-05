//! rs_tracing is a crate that outputs trace events in the [trace event format]
//! that is used by chrome://tracing the output can also be converted to html
//! with [trace2html]
//! 
//! [trace2html]: https://github.com/catapult-project/catapult/blob/master/tracing/README.md
//! [trace event format]: https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview#
//!

#![feature(getpid)]
#![feature(use_extern_macros)]

#[cfg(feature = "rs_tracing")]
extern crate time;
#[cfg(feature = "rs_tracing")]
extern crate serde;
#[cfg(feature = "rs_tracing")]
extern crate serde_json;

#[doc(hidden)]
pub use internal::*;

/// Trace time used from invocation until end of current scope.
/// The event type is [Complete Event (X)] with start time and duration.
/// 
/// $cond: condition if tracing is active or not.
/// 
/// $name: name of the trace event.
/// 
/// $json: optional custom data formated as serdes [json] macro.
/// 
/// [Complete Event (X)]: https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview#heading=h.lpfof2aylapb
/// [json]: https://docs.serde.rs/serde_json/macro.json.html
/// # Examples
///
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// {
/// trace_scoped!(true,"event name");
/// println!("this is timed");
/// }
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// {
/// trace_scoped!(true,"event name","custom":"data","u32":4);
/// println!("this is timed");
/// }
/// # }
/// ```
#[macro_export]
macro_rules! trace_scoped {
    ($cond:expr, $name: expr) => {
        trace_scoped_internal!($cond, $name)
    };
    ($cond:expr, $name: expr, $($json:tt)+) =>{
        trace_scoped_internal!($cond, $name, $($json)+)
    }
}

/// trace time used for expression to finish.
/// The event type is [Complete Event (X)] with start time and duration.
/// 
/// $cond: condition if tracing is active or not.
/// 
/// $name: name of the trace event.
/// 
/// $expr: expression to trace.
/// 
/// $json: optional custom data formated as serdes [json] macro.
/// 
/// [Complete Event (X)]: https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview#heading=h.lpfof2aylapb
/// [json]: https://docs.serde.rs/serde_json/macro.json.html
/// # Examples
///
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// trace_expr!(true,"event name", println!("this is timed"));
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// trace_expr!(true,"event name",println!("this is timed"),"custom":"data","u32":4);
/// # }
/// ```
#[macro_export]
macro_rules! trace_expr {
    ($cond:expr, $name: expr, $expr: expr) => {
        trace_expr_internal!($cond, $name, $expr)
    };
    ($cond:expr, $name: expr, $expr: expr, $($json:tt)+) =>{
        trace_expr_internal!($cond, $name, $expr, $($json)+)
    }
}

/// Mark beginning of event, needs to be followed by corresponding trace_end.
/// The event type is [Duration Event (B)] with an instant time.
/// Start and end of the event must be on the same thread.
/// If you provide custom data to both the trace_begin and trace_end then 
/// the arguments will be merged.
/// 
/// $cond: condition if tracing is active or not.
/// 
/// $name: name of the trace event.
/// 
/// $json: optional custom data formated as serdes [json] macro.
/// 
/// [Duration Event (B)]: https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview#heading=h.nso4gcezn7n1
/// [json]: https://docs.serde.rs/serde_json/macro.json.html
/// # Examples
///
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// trace_begin!(true,"event name");
/// println!("this is timed");
/// trace_end!(true,"event name");
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// trace_begin!(true,"event name","custom":"data");
/// println!("this is timed");
/// trace_end!(true,"event name","u32":4);
/// # }
/// ```
#[macro_export]
macro_rules! trace_begin {
    ($cond:expr, $name: expr) => {
        trace_duration_internal!($cond, $name, $crate::EventType::DurationBegin)
    };
    ($cond:expr, $name: expr, $($json:tt)+) =>{
        trace_duration_internal!($cond, $name, $crate::EventType::DurationBegin, $($json)+)
    }
}

/// Mark end of event, needs to be proceeded by corresponding trace_begin.
/// The event type is [Duration Event (E)] with an instant time.
/// Start and end of the event must be on the same thread.
/// If you provide custom data to both the trace_begin and trace_end then 
/// the arguments will be merged.
/// 
/// $cond: condition if tracing is active or not.
/// 
/// $name: name of the trace event.
/// 
/// $json: optional custom data formated as serdes [json] macro.
///
/// [Duration Event (E)]: https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview#heading=h.nso4gcezn7n1
/// [json]: https://docs.serde.rs/serde_json/macro.json.html
/// # Examples
///
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// trace_begin!(true,"event name");
/// println!("this is timed");
/// trace_end!(true,"event name");
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// trace_begin!(true,"event name","custom":"data");
/// println!("this is timed");
/// trace_end!(true,"event name","u32":4);
/// # }
/// ```
#[macro_export]
macro_rules! trace_end {
    ($cond:expr, $name: expr) => {
        trace_duration_internal!($cond, $name, $crate::EventType::DurationEnd)
    };
    ($cond:expr, $name: expr, $($json:tt)+) =>{
        trace_duration_internal!($cond, $name, $crate::EventType::DurationEnd, $($json)+)
    }
}

#[cfg(feature = "rs_tracing")]
mod internal {

#[doc(hidden)]
pub use serde_json::{json, json_internal};

use std::io::{self, Write};
use std::process;
use std::thread::{self, ThreadId};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json;
use time;
use std::mem::transmute;

#[doc(hidden)]
pub enum EventType {
    DurationBegin,
    DurationEnd,
    Complete,
}

impl Serialize for EventType {
    #[doc(hidden)]
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

#[doc(hidden)]
pub struct TraceEvent<'a> {
    name: &'a str,
    ph: EventType,
    pub ts: u64,
    pid: u32,
    tid: u64,
    pub dur: Option<u64>,
    args: Option<serde_json::Value>,
}

impl<'a> Serialize for TraceEvent<'a> {
    #[doc(hidden)]
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
    #[doc(hidden)]
    pub fn new(name: &'a str, event_type: EventType, args: Option<serde_json::Value>) -> Self {
        TraceEvent {
            name,
            ph: event_type,
            ts: precise_time_microsec(),
            pid: process::id(),
            tid: unsafe {
                // only want an unique identifier per thread think this is ok.
                transmute::<ThreadId, u64>(thread::current().id())
            },
            dur: None,
            args,
        }
    }
}

#[doc(hidden)]
pub struct EventGuard<'a> {
    event: TraceEvent<'a>,
}

impl<'a> EventGuard<'a> {
    #[doc(hidden)]
    pub fn new(name: &'a str, args: Option<serde_json::Value>) -> EventGuard<'a> {
        EventGuard {
            event: TraceEvent::new(name, EventType::Complete, args),
        }
    }
}

impl<'a> Drop for EventGuard<'a> {
    #[doc(hidden)]
    fn drop(&mut self) {
        self.event.dur = Some(precise_time_microsec() - self.event.ts);
        print_trace_event(&self.event);
    }
}

#[doc(hidden)]
pub fn print_trace_event(event: &TraceEvent) {
    let mut json_buffer = Vec::with_capacity(256);
    serde_json::to_writer(&mut json_buffer, event).unwrap();
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    lock.write_all(&json_buffer).unwrap();
    lock.write_all(b",\n").unwrap();
}

#[doc(hidden)]
pub fn precise_time_microsec() -> u64 {
    time::precise_time_ns()/1000
}

#[doc(hidden)]
#[macro_export]
macro_rules! trace_scoped_internal {
    ($cond:expr, $name: expr) => {
        let _guard = if $cond {
            Some($crate::EventGuard::new($name, None))
        }else{
            None
        };
    };
    ($cond:expr, $name: expr, $($json:tt)+) =>{
        let _guard = if $cond {
            Some($crate::EventGuard::new($name, Some(json!({$($json)+}))))
        }else{
            None
        };
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! trace_expr_internal {
    ($cond:expr, $name: expr, $expr: expr) => {
        if $cond {
            let mut event = $crate::TraceEvent::new($name, $crate::EventType::Complete, None);
            let result = $expr;
            event.dur = Some($crate::precise_time_microsec() - event.ts);
            $crate::print_trace_event(&event);
            result
        }else{
            $expr
        }
    };
    ($cond:expr, $name: expr, $expr: expr, $($json:tt)+) =>{
        if $cond {
            let mut event = $crate::TraceEvent::new($name, $crate::EventType::Complete, Some(json!({$($json)+})));
            let result = $expr;
            event.dur = Some($crate::precise_time_microsec() - event.ts);
            $crate::print_trace_event(&event);
            result
        }else{
            $expr
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! trace_duration_internal {
    ($cond:expr, $name: expr, $event_type: expr) => {
        if $cond {
            let event = $crate::TraceEvent::new($name, $event_type, None);
            $crate::print_trace_event(&event);
        }
    };
    ($cond:expr, $name: expr, $event_type: expr, $($json:tt)+) =>{
        if $cond {
            let event = $crate::TraceEvent::new($name, $event_type, Some(json!({$($json)+})));
            $crate::print_trace_event(&event);
        }
    }
}

} // mod internal



#[cfg(not(feature = "rs_tracing"))]
mod internal {
#[doc(hidden)]
#[macro_export]
macro_rules! trace_scoped_internal {
    ($($some:tt)+) =>{};
}

#[doc(hidden)]
#[macro_export]
macro_rules! trace_expr_internal {
    ($($some:tt)+) =>{};
}

#[doc(hidden)]
#[macro_export]
macro_rules! trace_duration_internal {
    ($($some:tt)+) =>{};
}
} // mod internal