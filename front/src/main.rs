use opentelemetry::{
    Context,
    sdk::trace::TracerProvider,
    trace::{
        TracerProvider as _,
        TraceContextExt,
        Tracer,
    }
};

use opentelemetry_otlp::{SpanExporter, ExportConfig, TonicConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let exporter = SpanExporter::new_tonic(ExportConfig::default(), TonicConfig::default())?;

    let provider = TracerProvider::builder()
    .with_simple_exporter(exporter)
    .build();

    let tracer = provider.tracer("otlp_eu");
        
    let parent = tracer.start("poing");
    println!("{:?}", parent);
    let parent_cx = Context::current_with_span(parent);
    println!("{:?}", parent_cx);
    println!("Poing");
    
    Ok(())
}