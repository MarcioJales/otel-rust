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

use build_html::{self, Html, HtmlContainer};

use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let exporter = SpanExporter::new_tonic(ExportConfig::default(), TonicConfig::default())?;

    let provider = TracerProvider::builder()
    .with_simple_exporter(exporter)
    .build();

    let tracer = provider.tracer("front");
        
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let parent = tracer.start("listener");
        println!("Connection established!");
    }

    let page = build_html::HtmlPage::new()
    .with_paragraph("Some Text")
    .to_html_string();
    
    Ok(())
}