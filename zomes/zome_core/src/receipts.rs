use hdk::prelude::*;

#[hdk_extern]
pub fn get_my_receipts(ah: ActionHash) -> ExternResult<Vec<MyValidationReceiptSet>> {
   let receipts = get_validation_receipts(GetValidationReceiptsInput::new(ah))?;
   //debug!("get_my_receipts: {:?}", receipts);
   Ok(receipts
      .into_iter()
      .map(|set| MyValidationReceiptSet {
         op_hash: set.op_hash,
         op_type: set.op_type,
         receipts_complete: set.receipts_complete,
         receipts: set
            .receipts
            .iter()
            .map(|info| MyValidationReceiptInfo {
               validation_status: info.validation_status as u32,
               validators: info.validators.clone(),
            })
            .collect(),
      })
      .collect())
}

/// Copy of `ValidationReceiptSet` so we can have TypeScript bindings with zits.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyValidationReceiptSet {
   /// The op hash that this receipt is for.
   pub op_hash: DhtOpHash,

   /// The type of the op that was validated.
   ///
   /// Note that the original type is discarded here because DhtOpType is part of `holochain_types`
   /// and moving it would be a breaking change. For now this is just informational.
   pub op_type: String,

   /// Whether this op has received the required number of receipts.
   pub receipts_complete: bool,

   /// The validation receipts for this op.
   pub receipts: Vec<MyValidationReceiptInfo>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyValidationReceiptInfo {
   /// the result of the validation.
   pub validation_status: u32,

   /// the remote validators who signed the receipt.
   pub validators: Vec<AgentPubKey>,
}
