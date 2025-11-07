use crate::*;
use hdk::prelude::*;

/// Attest all entries of a given entry type in the local source-chain
pub fn attest_all_local_typed<R: TryFrom<Entry>>(entry_type: EntryType) -> ExternResult<()> {
   let tuples = query_all_entry(entry_type)?;
   /// Form signal
   let pulses = tuples
      .into_iter()
      .map(|(record, _entry)| {
         let entry_pulse = EntryPulse::try_from_new_record(record, ValidatedBy::Me, false).unwrap();
         return ZomeSignalProtocol::Entry(entry_pulse);
      })
      .collect();
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
