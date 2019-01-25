//! rs_tracing is a crate that outputs trace events in the [trace event format]
//! that is used by chrome://tracing the output can also be converted to html
//! with [trace2html]
//!
//! [trace2html]: https://github.com/catapult-project/catapult/blob/master/tracing/README.md
//! [trace event format]: https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview#
//!

#![feature(use_extern_macros)]

#[cfg(feature = "rs_tracing")]
extern crate serde;
#[cfg(feature = "rs_tracing")]
extern crate serde_json;
#[cfg(feature = "rs_tracing")]
extern crate time;
#[doc(hidden)]
#[cfg(feature = "rs_tracing")]
pub use serde_json::{json, json_internal};

#[doc(hidden)]
pub use internal::*;

/// Activate tracing
/// # Examples
///
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// {
/// trace_activate!();
/// }
/// # }
/// ```
#[macro_export]
macro_rules! trace_activate {
    () => {
        trace_state_change!(&$crate::TraceState::Active)
    };
}

/// Deactivate tracing
/// # Examples
///
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// {
/// trace_deactivate!();
/// }
/// # }
/// ```
#[macro_export]
macro_rules! trace_deactivate {
    () => {
        trace_state_change!(&$crate::TraceState::InActive)
    };
}

/// opens a new trace file with the name <pid>.trace in the dir specified.
/// # Examples
///
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// {
/// open_trace_file!("/tmp");
/// {
///     trace_scoped!("event name");
///     println!("this is timed");
/// }
/// close_trace_file!();
/// }
/// # }
/// ```
#[macro_export]
macro_rules! open_trace_file {
    ($dir:expr) => {
        trace_to_file_internal!($dir)
    };
}

/// closes trace file
/// note will not trace the trace_scoped trace if called from the same scope.
#[macro_export]
macro_rules! close_trace_file {
    () => {
        close_trace_file_internal!()
    };
}

/// Trace time used from invocation until end of current scope.
/// The event type is [Complete Event (X)] with start time and duration.
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
/// trace_scoped!("event name");
/// println!("this is timed");
/// }
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// {
/// trace_scoped!("event name","custom":"data","u32":4);
/// println!("this is timed");
/// }
/// # }
/// ```
#[macro_export]
macro_rules! trace_scoped {
    ($name: expr) => {
        trace_scoped_internal!($name)
    };
    ($name: expr, $($json:tt)+) =>{
        trace_scoped_internal!($name, $($json)+)
    }
}

/// trace time used for expression to finish.
/// The event type is [Complete Event (X)] with start time and duration.
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
/// let result = trace_expr!("event name", { println!("this is timed"); true});
/// assert!(result, "result wasn't true!");
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// let result = trace_expr!("event name",{ println!("this is timed"); true},"custom":"data","u32":4);
/// assert!(result, "result wasn't true!");
/// # }
/// ```
#[macro_export]
macro_rules! trace_expr {
    ($name: expr, $expr: expr) => {
        trace_expr_internal!($name, $expr)
    };
    ($name: expr, $expr: expr, $($json:tt)+) =>{
        trace_expr_internal!($name, $expr, $($json)+)
    }
}

/// Mark beginning of event, needs to be followed by corresponding trace_end.
/// The event type is [Duration Event (B)] with an instant time.
/// Start and end of the event must be on the same thread.
/// If you provide custom data to both the trace_begin and trace_end then
/// the arguments will be merged.
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
/// trace_begin!("event name");
/// println!("this is timed");
/// trace_end!("event name");
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// trace_begin!("event name","custom":"data");
/// println!("this is timed");
/// trace_end!("event name","u32":4);
/// # }
/// ```
#[macro_export]
macro_rules! trace_begin {
    ($name: expr) => {
        trace_duration_internal!($name, $crate::EventType::DurationBegin)
    };
    ($name: expr, $($json:tt)+) =>{
        trace_duration_internal!($name, $crate::EventType::DurationBegin, $($json)+)
    }
}

/// Mark end of event, needs to be proceeded by corresponding trace_begin.
/// The event type is [Duration Event (E)] with an instant time.
/// Start and end of the event must be on the same thread.
/// If you provide custom data to both the trace_begin and trace_end then
/// the arguments will be merged.
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
/// trace_begin!("event name");
/// println!("this is timed");
/// trace_end!("event name");
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate rs_tracing;
/// # fn main() {
/// trace_begin!("event name","custom":"data");
/// println!("this is timed");
/// trace_end!("event name","u32":4);
/// # }
/// ```
#[macro_export]
macro_rules! trace_end {
    ($name: expr) => {
        trace_duration_internal!($name, $crate::EventType::DurationEnd)
    };
    ($name: expr, $($json:tt)+) =>{
        trace_duration_internal!($name, $crate::EventType::DurationEnd, $($json)+)
    }
}

#[cfg(feature = "rs_tracing")]
mod internal {

    use serde::ser::{Serialize, SerializeStruct, Serializer};
    use serde_json;
    use std::io::{self, BufWriter, Write};
    use std::mem::transmute;
    use std::process;
    use std::thread::{self, ThreadId};
    use time;

    use std::fs::{DirBuilder, File};
    use std::path::{Path, PathBuf};
    use std::sync::Mutex;

    pub enum TraceState {
        InActive,
        Active,
    }

    pub static mut TRACER: Option<Mutex<BufWriter<File>>> = None;
    pub static mut TRACE_STATE: &'static TraceState = &TraceState::Active;

    pub fn trace(event: &TraceEvent) {
        unsafe {
            if let Some(ref mut file) = TRACER {
                let mut file = file.lock().unwrap();
                serde_json::to_writer(&mut *file, event).unwrap();
                file.write_all(b",\n").unwrap();
            }
        }
    }

    pub fn set_trace_state(state: &'static TraceState) {
        unsafe {
            TRACE_STATE = state;
        }
    }

    pub fn is_trace_active() -> bool {
        unsafe {
            if let TraceState::Active = *TRACE_STATE {
                return true;
            }
            false
        }
    }

    #[doc(hidden)]
    pub enum EventType {
        DurationBegin,
        DurationEnd,
        Complete,
    }

    impl Serialize for EventType {
        #[doc(hidden)]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
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
        where
            S: Serializer,
        {
            let mut event = serializer.serialize_struct("TraceEvent", 7)?;
            event.serialize_field("name", &self.name)?;
            event.serialize_field("ph", &self.ph)?;
            event.serialize_field("ts", &self.ts)?;
            event.serialize_field("pid", &self.pid)?;
            event.serialize_field("tid", &self.tid)?;
            if let Some(ref dur) = self.dur {
                event.serialize_field("dur", &dur)?;
            }
            if let Some(ref args) = self.args {
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
            trace(&self.event);
        }
    }

    pub fn init_trace_to_file<P: AsRef<Path>>(dir: P) -> io::Result<()> {
        let mut dir_path = PathBuf::new();
        dir_path.push(dir);
        let mut file_path = dir_path.clone();
        file_path.push(process::id().to_string());
        file_path.set_extension("trace");
        let file = DirBuilder::new()
            .recursive(true)
            .create(dir_path)
            .and(File::create(file_path))?;
        let mut writer = BufWriter::new(file);
        writer.write_all(b"[")?;
        let file = Mutex::new(writer);
        unsafe {
            TRACER = Some(file);
        }
        Ok(())
    }

    pub fn close_trace_file_fn() {
        unsafe {
            if let Some(ref mut file) = TRACER {
                let mut file = file.lock().unwrap();
                (*file).flush().unwrap();
            }
            TRACER = None;
        }
    }

    #[doc(hidden)]
    pub fn precise_time_microsec() -> u64 {
        time::precise_time_ns() / 1000
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! trace_state_change {
        ($state:expr) => {
            $crate::set_trace_state($state)
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! trace_to_file_internal {
        ($dir:expr) => {
            $crate::init_trace_to_file($dir)
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! close_trace_file_internal {
        () => {
            $crate::close_trace_file_fn();
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! trace_scoped_internal {
    ($name: expr) => {
        let _guard = if $crate::is_trace_active() {
            Some($crate::EventGuard::new($name, None))
        }else{
            None
        };
    };
    ($name: expr, $($json:tt)+) =>{
        let _guard = if $crate::is_trace_active() {
            Some($crate::EventGuard::new($name, Some(json!({$($json)+}))))
        }else{
            None
        };
    }
}

    #[doc(hidden)]
    #[macro_export]
    macro_rules! trace_expr_internal {
    ($name: expr, $expr: expr) => {
        if $crate::is_trace_active() {
            let mut event = $crate::TraceEvent::new($name, $crate::EventType::Complete, None);
            let result = $expr;
            event.dur = Some($crate::precise_time_microsec() - event.ts);
            $crate::trace(&event);
            result
        }else{
            $expr
        }
    };
    ($name: expr, $expr: expr, $($json:tt)+) =>{
        if $crate::is_trace_active() {
            let mut event = $crate::TraceEvent::new($name, $crate::EventType::Complete, Some(json!({$($json)+})));
            let result = $expr;
            event.dur = Some($crate::precise_time_microsec() - event.ts);
            $crate::trace(&event);
            result
        }else{
            $expr
        }
    }
}

    #[doc(hidden)]
    #[macro_export]
    macro_rules! trace_duration_internal {
    ($name: expr, $event_type: expr) => {
        if $crate::is_trace_active() {
            let event = $crate::TraceEvent::new($name, $event_type, None);
            $crate::trace(&event);
        }
    };
    ($name: expr, $event_type: expr, $($json:tt)+) =>{
        if $crate::is_trace_active() {
            let event = $crate::TraceEvent::new($name, $event_type, Some(json!({$($json)+})));
            $crate::trace(&event);
        }
    }
}

} // mod internal

#[cfg(not(feature = "rs_tracing"))]
mod internal {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! trace_state_change {
        ($state:expr) => {};
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! trace_to_file_internal {
        ($dir:expr) => {};
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! close_trace_file_internal {
        () => {};
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! trace_scoped_internal {
        ($($some:tt)+) => {};
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! trace_expr_internal {
        ($name:expr, $expr:expr) => {
            $expr
        };
        ($name:expr, $expr:expr, $($json:tt)+) => {
            $expr
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! trace_duration_internal {
        ($($some:tt)+) => {};
    }
} // mod internal
