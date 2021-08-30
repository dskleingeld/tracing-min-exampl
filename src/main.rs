use opentelemetry::global;
use tracing::{info, span};

fn setup_tracing() {
    // global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    // let tracer = opentelemetry_jaeger::new_pipeline()
    //     .with_service_name("minimal-example")
    //     .with_collector_endpoint("http://localhost:14268/api/traces")
    //     .install_simple()
    //     .unwrap();
    
    use opentelemetry::sdk::export::trace;
    let tracer = trace::stdout::new_pipeline().install_simple();

    use tracing_subscriber::prelude::*;
    let telemetry = tracing_opentelemetry::subscriber().with_tracer(tracer);
    let fmt_sub = tracing_subscriber::fmt::subscriber().with_target(false);

    let subscriber = tracing_subscriber::Registry::default()
        .with(fmt_sub)
        .with(telemetry);
    tracing::collect::set_global_default(subscriber).unwrap();

    let root = span!(tracing::Level::INFO, "in setup");
    let _enter = root.enter();
    info!("hello from setup");
}

fn main() {
    println!("Hello, world!");
    setup_tracing();
    let root = span!(tracing::Level::INFO, "after setup");
    let _enter = root.enter();
    info!("hello from main after setup ran");

    opentelemetry::global::shutdown_tracer_provider(); // sending remaining spans
    std::thread::sleep(std::time::Duration::from_secs(1));
}
