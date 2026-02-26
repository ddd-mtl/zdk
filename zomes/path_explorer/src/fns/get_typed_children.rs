use hdk::prelude::*;
use path_explorer_types::*;

/// Return all children Anchors of same link-type as parent Anchor
#[hdk_extern]
pub fn get_typed_children_from_local(parent_ta: TypedAnchor) -> ExternResult<Vec<TypedAnchor>> {
   let children = parent_ta.children(GetStrategy::Local)?;
   debug!("children: {:?}", children);
   let children_tas = batch_convert_path_to_anchor(children)?;
   Ok(children_tas)
}

/// Return all children Anchors of same link-type as parent Anchor
#[hdk_extern]
pub fn get_typed_children_from_network(parent_ta: TypedAnchor) -> ExternResult<Vec<TypedAnchor>> {
   let children = parent_ta.children(GetStrategy::Network)?;
   debug!("children: {:?}", children);
   let children_tas = batch_convert_path_to_anchor(children)?;
   Ok(children_tas)
}
