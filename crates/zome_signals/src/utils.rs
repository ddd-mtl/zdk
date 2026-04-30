use crate::ValidatedBy;
use hdk::prelude::*;

/// same as get_variant() from zome_utils; but we don't want any dep
pub(crate) fn get_variant_from_index<T: UnitEnum>(entry_index: EntryDefIndex) -> ExternResult<T::Unit> {
   let mut i = 0;
   for variant in T::unit_iter() {
      if i == entry_index.0 {
         return Ok(variant);
      }
      i += 1;
   }
   Err(wasm_error!(format!("Unknown EntryDefIndex: {}", entry_index.0)))
}

/// Panic hook for debugging crashes in zomes.
pub(crate) fn zome_panic_hook(info: &std::panic::PanicHookInfo) {
   let mut msg = "\n\nPanic during zome call ".to_owned();
   msg.push_str(&dump_context());
   msg.push_str("\n\n");
   msg.push_str(&info.to_string());
   error!("{}\n\n", &msg);
}

/// Return zome context as String
fn dump_context() -> String {
   let mut msg = String::new();
   if let Ok(zome_info) = zome_info() {
      let maybe_call_info = call_info();
      if let Ok(call_info) = maybe_call_info {
         let provenance = format!("{}", &call_info.provenance)[5..13].to_string();
         msg.push_str(&format!(
            "'{}::{}()' by agent {} ",
            zome_info.name, call_info.function_name, provenance
         ));
      }
   }
   if let Ok(agent_info) = agent_info() {
      let snip = format!("{}", &agent_info.agent_initial_pubkey)[5..13].to_string();
      msg.push_str(&format!("in chain of agent {snip}"));
   }
   msg
}

///
pub fn determine_record_validation(record: Record, me: &AgentPubKey) -> ValidatedBy {
   if record.action().author() != me {
      return ValidatedBy::None;
   }
   return determine_validation(record.action_address());
}

///
pub fn determine_validation(ah: &ActionHash) -> ValidatedBy {
   let receipts = get_validation_receipts(GetValidationReceiptsInput::new(ah.clone()));
   match receipts {
      Ok(receipts) => {
         let maybe_record_receipts: Option<&ValidationReceiptSet> =
            receipts.iter().find(|receipt_set| receipt_set.op_type == "StoreRecord");
         if let Some(record_receipts) = maybe_record_receipts {
            if record_receipts.receipts_complete {
               ValidatedBy::Network
            } else {
               let count = record_receipts
                  .receipts
                  .iter()
                  .filter(|receipt| receipt.validation_status == ValidationStatus::Valid)
                  .count();
               if count == 0 { ValidatedBy::Me } else { ValidatedBy::Peer }
            }
         } else {
            ValidatedBy::Me
         }
      },
      Err(e) => {
         error!("determineValidation() failed: {:?}", e);
         ValidatedBy::Me
      },
   }
}
