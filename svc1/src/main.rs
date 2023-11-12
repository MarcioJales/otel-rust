use opentelemetry::{
    Context,
    sdk::{
        Resource,
        runtime,
        trace::{
            self,
            Sampler,
        }
    },
    trace::{
        Tracer,
        TraceContextExt
    }
};

use opentelemetry_otlp::WithExportConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
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
    .install_batch(runtime::Tokio).unwrap();
        
    let parent = otlp_tracer.start("poing");
    println!("{:?}", parent);
    let parent_cx = Context::current_with_span(parent);
    println!("{:?}", parent_cx);
    println!("Poing");
    
    Ok(())
}