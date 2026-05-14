use crate::ValidatedBy;
use hdk::map_extern::ExternResult;
use hdk::prelude::{
   ActionHash, AgentPubKey, GetValidationReceiptsInput, Record, ValidationReceiptSet, ValidationStatus, error,
   get_validation_receipts,
};

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
   use hdk::prelude::*;

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
