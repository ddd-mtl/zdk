use hdk::prelude::*;
use zome_path::ItemLink;

mod get_latest_time_indexed_links;
mod index_item;
mod sweep_interval;
mod timed_item_tag;
mod timepath_utils;

pub use get_latest_time_indexed_links::*;
pub use index_item::*;
pub use sweep_interval::*;
pub use timed_item_tag::*;
pub use timepath_utils::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SweepResponse {
   pub sweeped_interval: SweepInterval, // From begin-bucket start time to end-bucket finish time.
   pub found_items: Vec<(Timestamp, ItemLink)>, // Bucket start time
}

impl SweepResponse {
   pub fn new(sweeped_interval: SweepInterval, found_items: Vec<(Timestamp, ItemLink)>) -> Self {
      Self {
         sweeped_interval,
         found_items,
      }
   }
}
