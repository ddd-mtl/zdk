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
   return determine_validation_inner(receipts);
}

///
pub fn determine_validation_inner(receipts: ExternResult<Vec<ValidationReceiptSet>>) -> ValidatedBy {
   match receipts {
      Ok(receipts) => {
         let Some(record_receipts) = receipts.iter().find(|receipt_set| receipt_set.op_type == "StoreRecord") else {
            return ValidatedBy::Me;
         };
         if record_receipts.receipts_complete {
            ValidatedBy::Network
         } else {
            let maybe_valid = record_receipts
               .receipts
               .iter()
               .find(|receipt| receipt.validation_status == ValidationStatus::Valid);
            if maybe_valid.is_some() {
               ValidatedBy::Peer
            } else {
               ValidatedBy::Me
            }
         }
      },
      Err(e) => {
         error!("determine_validation() failed: {:?}", e);
         ValidatedBy::Me
      },
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   fn make_hash() -> DhtOpHash {
      DhtOpHash::from_raw_36(vec![0u8; 36])
   }
   fn make_agent() -> AgentPubKey {
      AgentPubKey::from_raw_36(vec![0u8; 36])
   }

   fn make_receipt_info(validation_status: ValidationStatus, validators: Vec<AgentPubKey>) -> ValidationReceiptInfo {
      ValidationReceiptInfo {
         validation_status,
         validators,
      }
   }

   fn make_receipt_set(
      op_type: &str,
      receipts: Vec<ValidationReceiptInfo>,
      receipts_complete: bool,
   ) -> ValidationReceiptSet {
      ValidationReceiptSet {
         op_hash: make_hash(),
         op_type: op_type.to_string(),
         receipts_complete,
         receipts,
      }
   }

   #[test]
   fn test_determine_validation_when_receipts_error() {
      assert_eq!(
         ValidatedBy::Me,
         determine_validation_inner(Err(wasm_error!(WasmErrorInner::Guest("network failure".to_string()))))
      );
   }

   #[test]
   fn test_determine_validation_when_no_store_record_receipt_set() {
      let receipts = vec![
         make_receipt_set("StoreEntry", vec![], false),
         make_receipt_set("RegisterAgentActivity", vec![], false),
      ];
      assert_eq!(determine_validation_inner(Ok(receipts)), ValidatedBy::Me);
   }

   #[test]
   fn test_determine_validation_when_receipts_empty() {
      assert_eq!(determine_validation_inner(Ok(vec![])), ValidatedBy::Me);
   }

   #[test]
   fn test_determine_validation_when_receipts_complete() {
      let receipts = vec![make_receipt_set("StoreRecord", vec![], true)];
      assert_eq!(determine_validation_inner(Ok(receipts)), ValidatedBy::Network);
   }

   // receipts_complete takes priority regardless of receipt contents
   #[test]
   fn test_determine_validation_when_receipts_complete_even_if_no_valid_receipts() {
      let receipts = vec![make_receipt_set(
         "StoreRecord",
         vec![make_receipt_info(ValidationStatus::Rejected, vec![])],
         true,
      )];
      assert_eq!(determine_validation_inner(Ok(receipts)), ValidatedBy::Network);
   }

   #[test]
   fn test_determine_validation_when_receipts_complete_and_empty_receipts() {
      let receipts = vec![make_receipt_set("StoreRecord", vec![], true)];
      assert_eq!(determine_validation_inner(Ok(receipts)), ValidatedBy::Network);
   }

   #[test]
   fn test_determine_validation_when_incomplete_but_has_valid_receipt() {
      let receipts = vec![make_receipt_set(
         "StoreRecord",
         vec![make_receipt_info(ValidationStatus::Valid, vec![])],
         false, // incomplete
      )];
      assert_eq!(determine_validation_inner(Ok(receipts)), ValidatedBy::Peer);
      let receipts = vec![make_receipt_set(
         "StoreRecord",
         vec![make_receipt_info(ValidationStatus::Valid, vec![make_agent()])],
         false, // incomplete
      )];
      assert_eq!(determine_validation_inner(Ok(receipts)), ValidatedBy::Peer);
      let receipts = vec![make_receipt_set(
         "StoreRecord",
         vec![make_receipt_info(
            ValidationStatus::Valid,
            vec![make_agent(), make_agent(), make_agent(), make_agent()],
         )],
         false, // incomplete
      )];
      assert_eq!(determine_validation_inner(Ok(receipts)), ValidatedBy::Peer);
   }

   #[test]
   fn test_determine_validation_when_mixed_receipts_and_at_least_one_valid() {
      let receipts = vec![make_receipt_set(
         "StoreRecord",
         vec![
            make_receipt_info(ValidationStatus::Rejected, vec![make_agent()]),
            make_receipt_info(ValidationStatus::Valid, vec![make_agent()]),
            make_receipt_info(ValidationStatus::Rejected, vec![make_agent()]),
         ],
         false,
      )];
      assert_eq!(determine_validation_inner(Ok(receipts)), ValidatedBy::Peer);
   }

   #[test]
   fn test_determine_validation_when_incomplete_and_no_valid_receipts() {
      let receipts = vec![make_receipt_set(
         "StoreRecord",
         vec![make_receipt_info(ValidationStatus::Rejected, vec![make_agent()])],
         false,
      )];
      assert_eq!(determine_validation_inner(Ok(receipts)), ValidatedBy::Me);
   }

   #[test]
   fn test_determine_validation_when_incomplete_and_receipts_empty() {
      let receipts = vec![make_receipt_set("StoreRecord", vec![], false)];
      assert_eq!(determine_validation_inner(Ok(receipts)), ValidatedBy::Me);
   }

   #[test]
   fn test_determine_validation_when_store_record_present() {
      // StoreEntry has complete=true but StoreRecord (incomplete, no Valid) should win
      let receipts = vec![
         make_receipt_set(
            "StoreEntry",
            vec![make_receipt_info(ValidationStatus::Valid, vec![make_agent()])],
            true,
         ),
         make_receipt_set("StoreRecord", vec![], false),
      ];
      // StoreRecord is incomplete and has no Valid receipts → Me
      assert_eq!(determine_validation_inner(Ok(receipts)), ValidatedBy::Me);
   }

   // Two StoreRecord sets; find() stops at the first one
   #[test]
   fn test_uses_first_store_record_set_found() {
      let receipts = vec![
         make_receipt_set("StoreRecord", vec![], false), // first → Me
         make_receipt_set(
            "StoreRecord",
            vec![make_receipt_info(ValidationStatus::Valid, vec![])],
            true,
         ),
      ];
      assert_eq!(determine_validation_inner(Ok(receipts)), ValidatedBy::Me);
   }
}
