use crate::*;
#[allow(unused_imports)]
use agent_directory_integrity::*;
use hdk::prelude::*;
use zome_path::*;

/// Returns the addresses of all agents who have accessed the DNA
#[hdk_extern]
pub fn get_registered_agents(_: ()) -> ExternResult<Vec<AgentPubKey>> {
   let child_links = tp_children_paths(&get_agent_directory_typed_path(), GetStrategy::Local)?;
   let agent_keys = child_links
      .iter()
      .map(|typed_link| path_to_agent(&typed_link.path))
      .filter_map(Result::ok)
      .collect();
   Ok(agent_keys)
}
