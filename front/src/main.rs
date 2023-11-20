use opentelemetry::{
    global,
    sdk::trace::TracerProvider,
    trace::{
        TracerProvider as _,
        TraceContextExt,
        Tracer,
    }
};

use opentelemetry_otlp::{SpanExporter, ExportConfig, TonicConfig};

use build_html::{self, Html, HtmlContainer};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let exporter = SpanExporter::new_tonic(ExportConfig::default(), TonicConfig::default())?;

    let provider = TracerProvider::builder()
    .with_simple_exporter(exporter)
    .build();

    let _ = global::set_tracer_provider(provider);

    HttpServer::new(|| {
        App::new()
            .service(front)
    })
    .bind(("127.0.0.1", 7979))?
    .run()
    .await?;

    Ok(())
}


#[get("/")]
async fn front() -> impl Responder {
    let tracer = global::tracer("front");
    let _ = tracer.start("main");

    let page = build_html::HtmlPage::new()
    .with_paragraph("Some Text")
    .to_html_string();

    HttpResponse::Ok().body(page)
}