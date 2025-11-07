use hdk::prelude::*;
use hdk::prelude::holo_hash::{AnyLinkableHashB64};
use zome_utils::*;
use path_explorer_types::*;


/// Return all ItemLinks from a LeafAnchor
#[hdk_extern]
pub fn get_all_items_from_anchor(leaf_anchor: String) -> ExternResult<Vec<ItemLink>>  {
  if leaf_anchor.is_empty() {
    return zome_error!("get_all_items() Failed. Input string is empty");
  }
  let path = Path::from(leaf_anchor.clone());
  let lls = get_all_itemlinks(path, None)?;
  debug!("leaf_anchor = {} ; found {}", leaf_anchor, lls.len());
  Ok(lls)
}


/// Return all ItemLinks from a hash
#[hdk_extern]
pub fn get_all_items(hash: AnyLinkableHash) -> ExternResult<Vec<ItemLink>>  {
  let /*mut*/ links = get_links(LinkQuery::new(hash, all_dna_link_types().try_into_filter().unwrap()), GetStrategy::Network)?;
  debug!("from hash: found {} children", links.len());
  /// Only need one of each hash.
  //links.sort_unstable_by(|a, b| a.tag.cmp(&b.tag));
  //links.dedup_by(|a, b| a.tag.eq(&b.tag));
  /// Convert to ItemLinks
  let res = links.into_iter().map(|link| ItemLink::from(link)).collect();
  Ok(res)
}


/// Return all ItemLinks from a B64 hash
#[hdk_extern]
pub fn get_all_items_from_b64(b64: AnyLinkableHashB64) -> ExternResult<Vec<ItemLink>>  {
  let hash: AnyLinkableHash = b64.clone().into();
  debug!("b64: {} -> {}", b64, hash);
  let /*mut*/ links = get_links(LinkQuery::new(hash, all_dna_link_types()), GetStrategy::Network)?;
  debug!("b64: found {} children", links.len());
  /// Only need one of each hash.
  //links.sort_unstable_by(|a, b| a.tag.cmp(&b.tag));
  //links.dedup_by(|a, b| a.tag.eq(&b.tag));
  /// Convert to ItemLinks
  let res = links.into_iter().map(|link| ItemLink::from(link)).collect();
  Ok(res)
}


// /// Return all itemLinks from a B64 hash
// #[hdk_extern]
// pub fn get_all_items_from_b64(b64: AnyDhtHashB64) -> ExternResult<Vec<ItemLink>>  {
//   let hash: AnyDhtHash = b64.into();
//   let anchor = hash2anchor(hash);
//   debug!("get_all_items_from_b64() {} -> {}", b64, anchor);
//   let lls = get_all_items(anchor)?;
//   Ok(lls)
// }

