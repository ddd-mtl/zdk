use hdi::hash_path::path::Component;
use hdk::prelude::*;
use hdk::prelude::holo_hash::EntryHashB64;
use crate::fns::get_all_root_anchors;
use path_explorer_types::*;


/// Determine TypedAnchor from anchor, by looking for parent
/// Return EntryHash of anchor if no parents found
#[hdk_extern]
pub fn get_typed_anchor(anchor: String) -> ExternResult<(EntryHashB64, Option<TypedAnchor>)> {
  let path = Path::from(anchor.clone());
  let path_eh = path.path_entry_hash()?;
  let path_hash = AnyLinkableHash::from(path_eh.clone());

  debug!("path_eh: {}", path_eh);

  /// Determine parent_path
  let maybe_parent_path = if path.as_ref().len() > 1 {
    let parent_vec: Vec<Component> =
      path.as_ref()[0..path.as_ref().len() - 1].to_vec();
    Some(Path::from(parent_vec))
  } else {
    None
  };

  debug!("maybe_parent_path: {:?}", maybe_parent_path);

  /// Check for parent's children and match which fn argument to get link info
  if let Some(parent_path) = maybe_parent_path {
    let child_links = get_any_children(parent_path, None)?;
    debug!("child_links: {:?}", child_links);
    for link in child_links {
      if link.target == path_hash {
        let ta = TypedAnchor {
          anchor: anchor.clone(),
          zome_index: link.zome_index.0,
          link_index: link.link_type.0,
        };
        return Ok((path_eh.into(), Some(ta)));
      }
    }
    debug!("no match found");
  } else {
    let root_anchors = get_all_root_anchors(())?;
    debug!("root_anchors: {:?}", root_anchors);
    for root_anchor in root_anchors {
      if root_anchor.anchor == anchor {
        return Ok((path_eh.into(), Some(root_anchor)));
      }
    }
    debug!("no match found (2)");
  }

  /// Done
  Ok((path_eh.into(), None))
}

