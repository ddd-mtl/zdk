use crate::*;
use hdk::prelude::*;

///
pub fn emit_zome_signal(pulses: Vec<ZomeSignalProtocol>) -> ExternResult<()> {
   if pulses.is_empty() {
      return Ok(());
   }
   let signal = ZomeSignal {
      from: agent_info()?.agent_initial_pubkey,
      pulses,
   };
   return emit_signal(&signal);
}

///-------------------------------------------------------------------------------------------------
/// System
///-------------------------------------------------------------------------------------------------

///
pub fn emit_system_signal(sys: SystemAttestation) -> ExternResult<()> {
   let signal = ZomeSignal {
      from: agent_info()?.agent_initial_pubkey,
      pulses: vec![ZomeSignalProtocol::System(sys)],
   };
   return emit_signal(&signal);
}

///-------------------------------------------------------------------------------------------------
/// Entry
///-------------------------------------------------------------------------------------------------

///
pub fn attest_entry_created(record: Record, validation: ValidatedBy, is_new: bool) -> ExternResult<()> {
   let pulse = EntryPulse::try_from_new_record(record, validation, is_new)?;
   return emit_zome_signal(vec![ZomeSignalProtocol::Entry(pulse)]);
}

///
pub fn attest_entry_deleted(
   delete_action: ActionHashed,
   create_record: Record,
   validation: ValidatedBy,
   is_new: bool,
) -> ExternResult<()> {
   let pulse = EntryPulse::try_with_delete_action(delete_action, create_record, validation, is_new)?;
   return emit_zome_signal(vec![ZomeSignalProtocol::Entry(pulse)]);
}

/// Should only be used for entries authored by this agent
pub fn attest_new_entry(sah: SignedActionHashed, validation: ValidatedBy) -> ExternResult<()> {
   let Some(eh) = sah.action().entry_hash() else {
      return Err(wasm_error!("Action has no Entry"));
   };
   let entry = must_get_entry(eh.to_owned())?.content;
   let record = Record::new(sah, RecordEntry::Present(entry));
   /// Emit Signal
   attest_entry_created(record, validation, true)?;
   Ok(())
}

///-------------------------------------------------------------------------------------------------
/// Link
///-------------------------------------------------------------------------------------------------

///
pub fn attest_link_deleted(
   delete: &Action,
   create: &Action,
   validation: ValidatedBy,
   is_new: bool,
) -> ExternResult<()> {
   let link = link_from_delete(delete, create)?;
   let pulse = LinkPulse {
      link,
      state: StateChange::Delete(is_new),
      validation,
   };
   return emit_zome_signal(vec![ZomeSignalProtocol::Link(pulse)]);
}

///
pub fn attest_link_created(
   link_ah: ActionHash,
   create: &Action,
   validation: ValidatedBy,
   is_new: bool,
) -> ExternResult<()> {
   let link = link_from_create(link_ah, create)?;
   return emit_zome_signal(vec![ZomeSignalProtocol::Link(LinkPulse {
      link,
      state: StateChange::Create(is_new),
      validation,
   })]);
}

///
pub fn attest_link(link: Link, state: StateChange, validation: ValidatedBy) -> ExternResult<()> {
   return emit_zome_signal(vec![ZomeSignalProtocol::Link(LinkPulse {
      link,
      state,
      validation,
   })]);
}

///
pub fn attest_links(links: Vec<Link>, validation: ValidatedBy) -> ExternResult<()> {
   let pulses = links
      .into_iter()
      .map(|link| {
         ZomeSignalProtocol::Link(LinkPulse {
            link,
            state: StateChange::Create(false),
            validation: validation.clone(),
         })
      })
      .collect();
   emit_zome_signal(pulses)?;
   Ok(())
}

/// `create` must be a CreateLink Action. Common fields come from its header,
/// link fields from its `CreateLinkData`.
pub fn link_from_create(create_ah: ActionHash, create: &Action) -> ExternResult<Link> {
   let ActionData::CreateLink(create_data) = &create.data else {
      return Err(wasm_error!("Action is not a CreateLink"));
   };
   Ok(Link {
      author: create.header.author.clone(),
      base: create_data.base_address.clone(),
      target: create_data.target_address.clone(),
      timestamp: create.header.timestamp,
      zome_index: create_data.zome_index,
      link_type: create_data.link_type,
      tag: LinkTag::from(create_data.tag.clone().into_inner()),
      create_link_hash: create_ah,
   })
}

/// `delete` must be a DeleteLink Action and `create` the CreateLink Action it deletes.
pub fn link_from_delete(delete: &Action, create: &Action) -> ExternResult<Link> {
   let ActionData::DeleteLink(delete_data) = &delete.data else {
      return Err(wasm_error!("Action is not a DeleteLink"));
   };
   let ActionData::CreateLink(create_data) = &create.data else {
      return Err(wasm_error!("Action is not a CreateLink"));
   };
   Ok(Link {
      author: delete.header.author.clone(),
      base: create_data.base_address.clone(),
      target: create_data.target_address.clone(),
      timestamp: delete.header.timestamp,
      zome_index: create_data.zome_index,
      link_type: create_data.link_type,
      tag: LinkTag::from(create_data.tag.clone().into_inner()),
      create_link_hash: delete_data.link_add_address.clone(),
   })
}
