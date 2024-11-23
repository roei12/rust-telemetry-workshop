use helpers::MockWriter;
use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_test_subscriber() -> MockWriter {
    let writer = MockWriter::new();
    let writer2 = writer.clone();
    tracing_subscriber::fmt()
        .with_writer(move || writer.clone())
        .with_span_events(FmtSpan::NEW | FmtSpan::EXIT)
        .json()
        .flatten_event(true)
        // TODO: we want JSON! Check out the test suite to understand the expected output.
        // Make sure to **flatten your events**!
        .init();
    writer2
}
