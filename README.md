Traces to Chrome's [trace_event format](https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview)

# Usage
    trace_scoped!(true, "complete");
    trace_expr!(true, "trace_expr", println!("trace_expr"));
    trace_begin(true, "duration");
    trace_end(true, "duration");
also possible to add cusomt data to all the macros formated like the serde::Json! macro
    trace_scoped!(true, "complete","custom":230);