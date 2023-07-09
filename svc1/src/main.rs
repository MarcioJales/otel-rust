use opentelemetry::{
    sdk::runtime,
    trace::Tracer,
};

use opentelemetry_otlp::{Protocol, WithExportConfig};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {

    let export_config = opentelemetry_otlp::ExportConfig {
        endpoint: "http://localhost:4317".to_string(),
        timeout: Duration::from_secs(3),
        protocol: Protocol::Grpc
    };

    let otlp_exporter_build = opentelemetry_otlp::new_exporter()
    .tonic()
    .with_export_config(export_config);
    
    let otlp_tracer = opentelemetry_otlp::new_pipeline().
    tracing().
    with_exporter(otlp_exporter_build).
    install_batch(runtime::AsyncStd).unwrap();

    otlp_tracer.in_span("doing_work", |cx| {
        println!("Poing");
    });
    
    Ok(())
}