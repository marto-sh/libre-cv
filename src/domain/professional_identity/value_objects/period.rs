use jiff::civil::Date;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Period {
    pub start: Date,
    pub end: Option<Date>,
}
