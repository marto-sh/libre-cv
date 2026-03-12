use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(module(digital_twin_error), visibility(pub(crate)))]
pub enum DigitalTwinError {
    #[snafu(display("tone must not be empty"))]
    EmptyTone,
}
