use hdk::prelude::*;
use std::fmt::*;

use crate::ts2anchor;

/// Time interval in us
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SweepInterval {
   pub begin: Timestamp,
   pub end: Timestamp,
}

impl Display for SweepInterval {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      let duration = self.duration().as_seconds_and_nanos().0;
      write!(
         f,
         "[{}, {}] (duration: {} secs)",
         self.begin.as_seconds_and_nanos().0,
         self.end.as_seconds_and_nanos().0,
         duration
      )
   }
}

impl Default for SweepInterval {
   fn default() -> Self {
      Self {
         begin: Timestamp::HOLOCHAIN_EPOCH,
         end: Timestamp::HOLOCHAIN_EPOCH,
      }
   }
}

impl SweepInterval {
   ///
   pub fn now() -> Self {
      Self {
         begin: Timestamp::HOLOCHAIN_EPOCH,
         end: sys_time().unwrap(),
      } // FIXME use dna_info.origin_time
   }

   ///
   pub fn with_beginning_at(begin: Timestamp) -> Self {
      Self {
         begin,
         end: sys_time().unwrap(),
      }
   }

   ///
   pub fn with_end_at(end: Timestamp) -> Self {
      Self {
         begin: Timestamp::HOLOCHAIN_EPOCH,
         end,
      } // FIXME use dna_info.origin_time
   }

   ///
   pub fn new(begin: Timestamp, end: Timestamp) -> ExternResult<Self> {
      if end < begin {
         return Err(wasm_error!(WasmErrorInner::Guest(
            "Invalid TimeInterval end < begin".to_string()
         )));
      }
      if begin.0 < 0 {
         return Err(wasm_error!(WasmErrorInner::Guest(
            "Invalid TimeInterval begin < 0".to_string()
         )));
      }
      Ok(Self { begin, end })
   }

   ///
   pub fn duration(&self) -> Timestamp {
      // let duration = self.end.sub(self.begin)?;
      // let ts: Timestamp = duration.into();
      // Ok(ts)
      let diff = self.end.0 - self.begin.0;
      Timestamp::from_micros(diff)
   }

   ///
   pub fn overlaps(&self, other: &Self) -> bool {
      self.begin <= other.end && self.end >= other.begin
   }

   /// Convert into start time of begin bucket and finish time of end bucket
   /// CAUTIOUS: end bucket finish time is equal to next bucket start time
   pub fn into_hour_buckets(&self) -> Self {
      let start_time_us = Timestamp::from_micros((self.begin.as_seconds_and_nanos().0 / 3600) * 3600 * 1000 * 1000);
      let end_hour_plus_1 = (self.end.as_seconds_and_nanos().0 / 3600) + 1;
      let finish_time_us = Timestamp::from_micros(end_hour_plus_1 * 3600 * 1000 * 1000);
      return SweepInterval::new(start_time_us, finish_time_us).unwrap();
   }

   ///
   pub fn get_end_bucket_start_time(&self) -> Timestamp {
      let hour_us = Timestamp::try_from(std::time::Duration::from_secs(3600)).unwrap();
      let diff = self.end.0 - hour_us.0;
      Timestamp::from_micros(diff)
   }

   /// Print as timepath anchor
   pub fn print_as_anchors(&self) -> String {
      return format!("[{}, {}]", ts2anchor(self.begin), ts2anchor(self.end));
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use chrono::DateTime;

   fn timestamp_from_rfc3339(value: &str) -> Timestamp {
      let dt = DateTime::parse_from_rfc3339(value).unwrap();
      Timestamp::from_micros(dt.timestamp_micros())
   }

   fn timestamp_from_micros(value: i64) -> Timestamp {
      Timestamp::from_micros(value)
   }

   #[test]
   fn default_starts_and_ends_at_holochain_epoch() {
      let interval = SweepInterval::default();

      assert_eq!(interval.begin, Timestamp::HOLOCHAIN_EPOCH);
      assert_eq!(interval.end, Timestamp::HOLOCHAIN_EPOCH);
   }

   #[test]
   fn with_end_at_starts_at_holochain_epoch_and_uses_given_end() {
      let time = timestamp_from_rfc3339("2023-04-13T12:34:56Z");

      let interval = SweepInterval::with_end_at(time);
      assert_eq!(interval.begin, Timestamp::HOLOCHAIN_EPOCH);
      assert_eq!(interval.end, time);
   }

   #[test]
   fn new_returns_interval_when_bounds_are_valid() {
      let begin = timestamp_from_micros(1_000);
      let end = timestamp_from_micros(2_500);

      let interval = SweepInterval::new(begin, end).unwrap();
      assert_eq!(interval.begin, begin);
      assert_eq!(interval.end, end);
   }

   #[test]
   fn new_allows_zero_duration_interval() {
      let timestamp = timestamp_from_micros(1_000);

      let interval = SweepInterval::new(timestamp, timestamp).unwrap();
      assert_eq!(interval.begin, timestamp);
      assert_eq!(interval.end, timestamp);
      assert_eq!(interval.duration(), Timestamp::from_micros(0));
   }

   #[test]
   fn new_returns_error_when_end_is_before_begin() {
      let begin = timestamp_from_micros(2_000);
      let end = timestamp_from_micros(1_000);

      let result = SweepInterval::new(begin, end);
      assert!(result.is_err());
   }

   #[test]
   fn new_returns_error_when_begin_is_negative() {
      let begin = timestamp_from_micros(-1);
      let end = timestamp_from_micros(1_000);

      let result = SweepInterval::new(begin, end);
      assert!(result.is_err());
   }

   #[test]
   fn duration_returns_difference_between_end_and_begin() {
      let interval = SweepInterval::new(timestamp_from_micros(1_000), timestamp_from_micros(3_500)).unwrap();
      assert_eq!(interval.duration(), Timestamp::from_micros(2_500));
   }

   #[test]
   fn overlaps_returns_true_when_intervals_partially_overlap() {
      let first = SweepInterval::new(timestamp_from_micros(1_000), timestamp_from_micros(3_000)).unwrap();
      let second = SweepInterval::new(timestamp_from_micros(2_000), timestamp_from_micros(4_000)).unwrap();

      assert!(first.overlaps(&second));
      assert!(second.overlaps(&first));
   }

   #[test]
   fn overlaps_returns_true_when_intervals_touch_at_boundary() {
      let first = SweepInterval::new(timestamp_from_micros(1_000), timestamp_from_micros(2_000)).unwrap();
      let second = SweepInterval::new(timestamp_from_micros(2_000), timestamp_from_micros(3_000)).unwrap();

      assert!(first.overlaps(&second));
      assert!(second.overlaps(&first));
   }

   #[test]
   fn overlaps_returns_true_when_one_interval_contains_the_other() {
      let outer = SweepInterval::new(timestamp_from_micros(1_000), timestamp_from_micros(5_000)).unwrap();
      let inner = SweepInterval::new(timestamp_from_micros(2_000), timestamp_from_micros(3_000)).unwrap();

      assert!(outer.overlaps(&inner));
      assert!(inner.overlaps(&outer));
   }

   #[test]
   fn overlaps_returns_false_when_intervals_are_disjoint() {
      let first = SweepInterval::new(timestamp_from_micros(1_000), timestamp_from_micros(2_000)).unwrap();
      let second = SweepInterval::new(timestamp_from_micros(2_001), timestamp_from_micros(3_000)).unwrap();

      assert!(!first.overlaps(&second));
      assert!(!second.overlaps(&first));
   }

   #[test]
   fn into_hour_buckets_expands_to_begin_hour_start_and_end_next_hour_start() {
      let interval = SweepInterval::new(
         timestamp_from_rfc3339("2023-04-13T12:34:56Z"),
         timestamp_from_rfc3339("2023-04-13T14:12:00Z"),
      )
      .unwrap();

      let bucketed = interval.into_hour_buckets();

      assert_eq!(bucketed.begin, timestamp_from_rfc3339("2023-04-13T12:00:00Z"));
      assert_eq!(bucketed.end, timestamp_from_rfc3339("2023-04-13T15:00:00Z"));
   }

   #[test]
   fn into_hour_buckets_expands_single_hour_interval_to_one_bucket() {
      let interval = SweepInterval::new(
         timestamp_from_rfc3339("2023-04-13T12:00:00Z"),
         timestamp_from_rfc3339("2023-04-13T12:59:59Z"),
      )
      .unwrap();

      let bucketed = interval.into_hour_buckets();

      assert_eq!(bucketed.begin, timestamp_from_rfc3339("2023-04-13T12:00:00Z"));
      assert_eq!(bucketed.end, timestamp_from_rfc3339("2023-04-13T13:00:00Z"));
   }

   #[test]
   fn get_end_bucket_start_time_subtracts_one_hour_from_end() {
      let interval = SweepInterval::new(
         timestamp_from_rfc3339("2023-04-13T12:00:00Z"),
         timestamp_from_rfc3339("2023-04-13T15:00:00Z"),
      )
      .unwrap();

      assert_eq!(
         interval.get_end_bucket_start_time(),
         timestamp_from_rfc3339("2023-04-13T14:00:00Z")
      );
   }
}
