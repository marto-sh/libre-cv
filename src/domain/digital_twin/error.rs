use snafu::Snafu;

use super::value_objects::EmptyTone;

#[derive(Debug, Snafu)]
#[snafu(module(digital_twin_error), visibility(pub(crate)))]
pub enum DigitalTwinError {
    #[snafu(display("{source}"))]
    Tone { source: EmptyTone },
}
