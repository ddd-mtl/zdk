mod validation;

use hdi::prelude::*;


/// All link types specific to this Zome
#[hdk_link_types]
pub enum AgentDirectoryLinkType {
   Agent,
}

/// Entry types are not necessary, but it is defined because otherwise holochain will fail.

#[hdk_entry_types]
#[unit_enum(AgentDirectoryEntryTypes)]
pub enum AgentDirectoryEntry {
   #[entry_type(required_validations = 1, visibility = "private")]
   Stub(Stub),
}

/// Bogus Entry
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Stub {}
