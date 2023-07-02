use opentelemetry::{global, sdk::export::trace::stdout, trace::Tracer};

fn main() {
    // Create a new trace pipeline that prints to stdout
    let tracer = stdout::new_pipeline().with_pretty_print(true).install_simple();

    tracer.in_span("doing_work", |cx| {
        println!("Poig");
    });

    // Shutdown trace pipeline
    global::shutdown_tracer_provider();
}