use hdk::map_extern::ExternResult;
use hdk::prelude::*;

/// ValidationStatus
#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub enum ValidatedBy {
   None,    // Untrusted, e.g. received remotely from another agent
   Me,      // I commited the action
   Network, // Trusted, received from DHT
}

/// Bool: True if state change just happened (real-time)
#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub enum StateChange {
   Create(bool),
   Update(bool),
   Delete(bool),
}

///
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct LinkPulse {
   pub link: Link,
   pub state: StateChange,
   pub validation: ValidatedBy,
}

impl LinkPulse {
   pub fn clear_validation(&mut self) {
      self.validation = ValidatedBy::None;
   }
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct EntryPulse {
   state: StateChange,
   validation: ValidatedBy,
   orig_ah: Option<ActionHash>,
   ah: ActionHash,
   eh: EntryHash,
   ts: Timestamp,
   author: AgentPubKey,
   def: AppEntryDef,
   bytes: AppEntryBytes,
}

impl EntryPulse {
   pub fn clear_validation(&mut self) {
      self.validation = ValidatedBy::None;
   }

   /// Can't do delete here since it does not hold the entry data
   pub fn try_from_new_record(record: Record, validation: ValidatedBy, is_new: bool) -> ExternResult<Self> {
      let state = match record.action() {
         Action::Create(_) => StateChange::Create(is_new),
         Action::Update(_) => StateChange::Update(is_new),
         _ => return Err(wasm_error!("Unhandled Action type")),
      };
      let orig_ah = match record.action() {
         Action::Update(update) => Some(update.original_action_address.clone()),
         _ => None,
      };

      let RecordEntry::Present(Entry::App(bytes)) = record.entry().to_owned() else {
         return Err(wasm_error!("Record has no entry data"));
      };
      let Some(EntryType::App(def)) = record.action().entry_type() else {
         return Err(wasm_error!("Record has no entry def"));
      };

      Ok(Self {
         orig_ah,
         ah: record.action_address().to_owned(),
         eh: record.action().entry_hash().unwrap().clone(),
         ts: record.action().timestamp(),
         author: record.action().author().clone(),
         state,
         validation,
         def: def.to_owned(),
         bytes,
      })
   }

   /// Input must be the NewEntryAction that is deleted
   pub fn try_from_delete_record(
      hashed: ActionHashed,
      entry: Entry,
      validation: ValidatedBy,
      is_new: bool,
   ) -> ExternResult<Self> {
      let action = hashed.content;
      let Action::Delete(delete) = action.clone() else {
         return Err(wasm_error!("Unhandled Action type"));
      };
      let Entry::App(bytes) = entry else {
         return Err(wasm_error!("Entry is not an App"));
      };
      let Some(EntryType::App(def)) = action.entry_type() else {
         return Err(wasm_error!("Entry has no entry def"));
      };

      Ok(Self {
         orig_ah: Some(delete.prev_action),
         ah: hashed.hash.to_owned(),
         eh: action.entry_hash().unwrap().clone(),
         ts: action.timestamp(),
         author: action.author().clone(),
         state: StateChange::Delete(is_new),
         validation,
         def: def.to_owned(),
         bytes,
      })
   }

   // ///
   // pub fn try_from_details(details: EntryDetails, is_new: bool) -> ExternResult<Self> {
   //     let state = match record.action() {
   //         Action::Create(_) => StateChange::Create(is_new),
   //         Action::Update(_) => StateChange::Update(is_new),
   //         _ => return Err(wasm_error!("Unhandled Action type")),
   //     };
   //     let RecordEntry::Present(Entry::App(bytes)) = record.entry().to_owned()
   //         else { return Err(wasm_error!("Record has no entry data")) };
   //     let Some(EntryType::App(def)) = record.action().entry_type()
   //         else { return Err(wasm_error!("Record has no entry def")) };
   //
   //     Ok(Self {
   //         ah: record.action_address().to_owned(),
   //         eh: record.action().entry_hash().unwrap().clone(),
   //         ts: record.action().timestamp(),
   //         author: record.action().author().clone(),
   //         state,
   //         def: def.to_owned(),
   //         bytes,
   //     })
   //}
}
