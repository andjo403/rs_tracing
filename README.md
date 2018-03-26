Traces to Chrome's [trace_event format](https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview)

# Usage
    trace_scoped!("complete");
    trace_fn!("trace_fn", trace_duration("trace_fn_fn"));
    trace_begin("duration");
    trace_end("duration");
also possible to add cusomt data to all the macros formated like the serde::Json! macro
    trace_scoped!("complete","custom":230);