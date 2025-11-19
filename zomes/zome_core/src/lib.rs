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

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct GetDataTypeInput {
   hash: AnyDhtHash,
   role: Option<String>,
   dna: Option<DnaHash>,
   get_strategy: GetStrategy,
}

/// Return AppEntryName or data type name of data at hash in role or dna
#[hdk_extern]
fn get_data_type(input: GetDataTypeInput) -> ExternResult<String> {
   let target_cell = if let Some(role) = input.role {
      CallTargetCell::OtherRole(role)
   } else {
      if let Some(dna) = input.dna {
         let cell_id = CellId::new(dna, agent_info()?.agent_initial_pubkey);
         CallTargetCell::OtherCell(cell_id)
      } else {
         CallTargetCell::Local
      }
   };
   // let data_entry_type = zome_utils::get_entry_type_at(input.hash.clone())?;
   // let EntryType::App(def) = data_entry_type
   // else { return Ok(data_entry_type) };
   let (name, _record) = zome_utils::get_app_entry_name(input.hash, target_cell, input.get_strategy)?;
   Ok(name.0.into())
}
