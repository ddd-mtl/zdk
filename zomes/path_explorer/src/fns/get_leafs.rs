use hdk::prelude::*;
use path_explorer_types::*;

/// Return all leafs of an Anchor
#[hdk_extern]
pub fn get_leafs_from_local(ta: TypedAnchor) -> ExternResult<Vec<TypedAnchor>> {
   debug!("get_leaf_anchors() {:?}", ta);
   let res = ta.walk(GetStrategy::Local)?;
   Ok(res)
}

/// Return all leafs of an Anchor
#[hdk_extern]
pub fn get_leafs_from_network(ta: TypedAnchor) -> ExternResult<Vec<TypedAnchor>> {
   debug!("get_leaf_anchors() {:?}", ta);
   let res = ta.walk(GetStrategy::Network)?;
   Ok(res)
}
