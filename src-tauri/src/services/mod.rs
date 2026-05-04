pub mod calculator;
pub mod validator;
pub mod aggregator;

pub use calculator::calculate;
pub use validator::validate_record;
pub use aggregator::aggregate_records;
