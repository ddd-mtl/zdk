use hdi::hash_path::path::{root_hash};
use hdk::prelude::*;
use zome_utils::*;
use path_explorer_types::*;


/// Return all RootAnchors from all Zomes.
/// A RootAnchor is a path linked from the ROOT entry.
#[hdk_extern]
pub fn get_all_root_anchors(_ : ()) -> ExternResult<Vec<TypedAnchor>> {
  /// Check for links on all link types
  let links = get_links(LinkQuery::new(
    root_hash()?,
    LinkTypeFilter::Dependencies(dna_zomes()).try_into_filter().unwrap(),
  ), GetStrategy::Network)?;
  let mut res = Vec::new();
  for link in links {
    let Ok(str) = compTag2str(&link.tag)
      else { continue };
    debug!("zome_index = {} | link_type = {} | tag = {}", link.zome_index.0, link.link_type.0, str);
    res.push(TypedAnchor::new(str, /*get_zome_index(&zome_name)*/ link.zome_index.0, link.link_type.0));
  }
  debug!("done | found: {}\n\n", res.len());
  ///
  Ok(res)
}
