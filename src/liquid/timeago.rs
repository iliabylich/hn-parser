use chrono::prelude::DateTime;
use chrono::Utc;
use liquid_core::{
    Display_filter, Filter, FilterReflection, ParseFilter, Result, Runtime, Value, ValueView,
};
use std::time::{Duration, UNIX_EPOCH};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "timeago",
    description = "Converts unix timestamp to '... ago' format.",
    parsed(TimeAgoFilter)
)]
pub struct TimeAgo;

#[derive(Debug, Default, Display_filter)]
#[name = "timeago"]
struct TimeAgoFilter;

impl Filter for TimeAgoFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let unix_timstamp: u64 = input.to_kstr().parse().expect("not a numeric value");

        let moment = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(unix_timstamp));
        let now = Utc::now();
        let delta = (now - moment).to_std().unwrap_or_default();

        let mut formatter = timeago::Formatter::new();
        formatter.num_items(3);

        let human_delta = formatter.convert(delta);
        Ok(Value::scalar(human_delta))
    }
}
