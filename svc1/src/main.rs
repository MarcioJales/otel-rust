use opentelemetry::{
    sdk::{
        runtime,
        trace::{
            self,
            Sampler,
        }
    },
    trace::Tracer,
};

use opentelemetry_otlp::{Protocol, WithExportConfig};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {

    let export_config = opentelemetry_otlp::ExportConfig {
        endpoint: "http://localhost:4318".to_string(),
        timeout: Duration::from_secs(3),
        protocol: Protocol::HttpBinary
    };

    let otlp_exporter_build = opentelemetry_otlp::new_exporter()
    .http()
    .with_export_config(export_config);
    
    let otlp_tracer = opentelemetry_otlp::new_pipeline()
    .tracing()
    .with_exporter(otlp_exporter_build)
    .with_trace_config(
        trace::config()
            .with_sampler(Sampler::AlwaysOn))
    .install_batch(runtime::AsyncStd).unwrap();

    otlp_tracer.in_span("doing_work", |cx| {
        println!("Poing");
    });
    
    Ok(())
}