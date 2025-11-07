use hdk::prelude::*;
#[allow(unused_imports)]
use agent_directory_integrity::*;
use crate::*;

/// Returns the addresses of all agents who have accessed the DNA
#[hdk_extern]
pub fn get_registered_agents(_:()) -> ExternResult<Vec<AgentPubKey>> {
   let child_links = get_agent_directory_typed_path().children_paths()?;
   let agent_keys = child_links.iter()
      .map(|typed_link| {
         path_to_agent(&typed_link.path)
      })
      .filter_map(Result::ok)
      .collect();
   Ok(agent_keys)
}
