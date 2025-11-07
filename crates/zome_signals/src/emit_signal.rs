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
pub fn attest_entry_created(record: Record, is_new: bool) -> ExternResult<()> {
   let pulse = EntryPulse::try_from_new_record(record, ValidatedBy::Me, is_new)?;
   return emit_zome_signal(vec![ZomeSignalProtocol::Entry(pulse)]);
}

///
pub fn attest_entry_deleted(ha: ActionHashed, entry: Entry, is_new: bool) -> ExternResult<()> {
   let pulse = EntryPulse::try_from_delete_record(ha, entry, ValidatedBy::Me, is_new)?;
   return emit_zome_signal(vec![ZomeSignalProtocol::Entry(pulse)]);
}

///
pub fn attest_new_entry(sah: SignedActionHashed) -> ExternResult<()> {
   let Some(eh) = sah.action().entry_hash() else {
      return Err(wasm_error!("Action has no Entry"));
   };
   let entry = must_get_entry(eh.to_owned())?.content;
   let record = Record::new(sah, Some(entry));
   /// Emit Signal
   attest_entry_created(record, true)?;
   Ok(())
}

///-------------------------------------------------------------------------------------------------
/// Link
///-------------------------------------------------------------------------------------------------

///
pub fn attest_link_deleted(delete: &DeleteLink, create: &CreateLink, is_new: bool) -> ExternResult<()> {
   let link = link_from_delete(delete, create);
   let pulse = LinkPulse {
      link,
      state: StateChange::Delete(is_new),
      validation: ValidatedBy::Me,
   };
   return emit_zome_signal(vec![ZomeSignalProtocol::Link(pulse)]);
}

///
pub fn attest_link_created(link_ah: ActionHash, create: &CreateLink, is_new: bool) -> ExternResult<()> {
   let link = link_from_create(link_ah, create);
   return emit_zome_signal(vec![ZomeSignalProtocol::Link(LinkPulse {
      link,
      state: StateChange::Create(is_new),
      validation: ValidatedBy::Me,
   })]);
}

///
pub fn attest_link(link: Link, state: StateChange) -> ExternResult<()> {
   return emit_zome_signal(vec![ZomeSignalProtocol::Link(LinkPulse {
      link,
      state,
      validation: ValidatedBy::Me,
   })]);
}

///
pub fn attest_links(links: Vec<Link>) -> ExternResult<()> {
   let pulses = links
      .into_iter()
      .map(|link| {
         ZomeSignalProtocol::Link(LinkPulse {
            link,
            state: StateChange::Create(false),
            validation: ValidatedBy::Me,
         })
      })
      .collect();
   emit_zome_signal(pulses)?;
   Ok(())
}

///
pub fn link_from_create(create_ah: ActionHash, create: &CreateLink) -> Link {
   Link {
      author: create.author.clone(),
      base: create.base_address.clone(),
      target: create.target_address.clone(),
      timestamp: create.timestamp,
      zome_index: create.zome_index,
      link_type: create.link_type,
      tag: LinkTag::from(create.tag.clone().into_inner()),
      create_link_hash: create_ah,
   }
}

///
pub fn link_from_delete(delete: &DeleteLink, create: &CreateLink) -> Link {
   Link {
      author: delete.author.clone(),
      base: create.base_address.clone(),
      target: create.target_address.clone(),
      timestamp: delete.timestamp,
      zome_index: create.zome_index,
      link_type: create.link_type,
      tag: LinkTag::from(create.tag.clone().into_inner()),
      create_link_hash: delete.link_add_address.clone(),
   }
}
