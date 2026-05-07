use crate::*;
use hdk::prelude::*;

/// Attest all entries of a given entry-type in the local source-chain
pub fn attest_all_local_typed<R: TryFrom<Entry>>(entry_type: EntryType) -> ExternResult<()> {
   let me = agent_info()?.agent_initial_pubkey;
   /// Form Delete signal
   let records = query_all_entry_delete(entry_type.clone())?;
   let mut pulses: Vec<ZomeSignalProtocol> = records
      .into_iter()
      .map(|record| {
         let validation = determine_record_validation(record.clone(), &me);
         let entry_pulse = EntryPulse::try_with_delete_action_no_bytes(
            record.action_hashed().clone(),
            entry_type.clone(),
            validation,
            false,
         )
         .unwrap();
         return ZomeSignalProtocol::Entry(entry_pulse);
      })
      .collect();
   /// Form Entry signal
   let tuples = query_all_entry(entry_type.clone())?;
   tuples.into_iter().for_each(|(record, _entry)| {
      let validation = determine_record_validation(record.clone(), &me);
      let entry_pulse = EntryPulse::try_from_new_record(record, validation, false).unwrap();
      pulses.push(ZomeSignalProtocol::Entry(entry_pulse));
   });
   /// Emit Signal
   emit_zome_signal(pulses)?;
   /// Done
   Ok(())
}

/// Return all entries of a given entry type present in the local source-chain
fn query_all_entry(entry_type: EntryType) -> ExternResult<Vec<(Record, Entry)>> {
   /// Query
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .action_type(ActionType::Create)
      .action_type(ActionType::Update)
      .entry_type(entry_type);
   let records = query(query_args)?;
   /// Get entries of all results
   let mut entries = Vec::new();
   for record in records {
      let RecordEntry::Present(entry) = record.entry() else {
         return Err(wasm_error!("Record should hold entry data"));
      };
      entries.push((record.clone(), entry.clone()))
   }
   /// Done
   Ok(entries)
}

/// Return all entries of a given entry type present in the local source-chain
fn query_all_entry_delete(entry_type: EntryType) -> ExternResult<Vec<Record>> {
   /// Query
   let query_args = ChainQueryFilter::default()
      .include_entries(false)
      .action_type(ActionType::Delete)
      .entry_type(entry_type);
   let records = query(query_args)?;
   /// Done
   Ok(records)
}
