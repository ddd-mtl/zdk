use hdk::prelude::*;

#[hdk_extern]
fn get_zome_info(_: ()) -> ExternResult<ZomeInfo> {
   return zome_info();
}

#[hdk_extern]
fn get_dna_info(_: ()) -> ExternResult<DnaInfo> {
   return dna_info();
}

#[hdk_extern]
fn get_my_agent_key_entry_hash(_: ()) -> ExternResult<AnyLinkableHash> {
   Ok(AnyLinkableHash::from(EntryHash::from(
      agent_info()?.agent_initial_pubkey,
   )))
}

#[hdk_extern]
fn get_record_author_local(dh: AnyDhtHash) -> ExternResult<AgentPubKey> {
   return zome_utils::get_author(dh, GetStrategy::Local);
}

#[hdk_extern]
fn get_record_author_network(dh: AnyDhtHash) -> ExternResult<AgentPubKey> {
   return zome_utils::get_author(dh, GetStrategy::Network);
}
