Traces to Chrome's [trace_event format](https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview)

# Usage
    trace_scoped!(true, "complete");
    trace_expr!(true, "trace_expr", println!("trace_expr"));
    trace_begin!(true, "duration");
    println!("trace_duration");
    trace_end!(true, "duration");
also possible to add custom data to all the macros formated like 
the [serde_json::json!](https://docs.serde.rs/serde_json/macro.json.html) macro e.g.

    trace_scoped!(true, "complete","custom":230,"more":"data");