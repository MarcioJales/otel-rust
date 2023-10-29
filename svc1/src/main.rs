use opentelemetry::{
    sdk::{
        Resource,
        runtime,
        trace::{
            self,
            Sampler,
        }
    },
    trace::Tracer,
};

use opentelemetry_otlp::WithExportConfig;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let otlp_exporter_build = opentelemetry_otlp::new_exporter()
    .tonic()
    .with_env();
    
    let otlp_tracer = opentelemetry_otlp::new_pipeline()
    .tracing()
    .with_exporter(otlp_exporter_build)
    .with_trace_config(
        trace::config()
            .with_resource(Resource::default())
            .with_sampler(Sampler::AlwaysOn))
    .install_batch(runtime::AsyncStd).unwrap();

    otlp_tracer.in_span("doing_work", |cx| {
        println!("Poing");
    });
    
    Ok(())
}