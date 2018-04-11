Traces to Chrome's [trace_event format](https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview)

# Usage
    trace_scoped!("complete");
    trace_expr!("trace_expr", println!("trace_expr"));
    trace_begin!("duration");
    println!("trace_duration");
    trace_end!("duration");
also possible to add custom data to all the macros formated like 
the [serde_json::json!](https://docs.serde.rs/serde_json/macro.json.html) macro e.g.

    trace_scoped!("complete","custom":230,"more":"data");
