use hdk::prelude::holo_hash::hash_type;
use hdk::prelude::*;
use zome_core::get_input_types::GetLhInput;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashInfo {
   link_type: String,
   entry_type: String,
   author: String,
   maybe_entry_def: Option<AppEntryDef>,
}

///
#[hdk_extern]
pub fn inspect_linkable_hash(input: GetLhInput) -> ExternResult<HashInfo> {
   let info = match input.lh.hash_type() {
      hash_type::AnyLinkable::External => HashInfo {
         link_type: "External".to_string(),
         entry_type: "External".to_string(),
         author: "Unknown".to_string(),
         maybe_entry_def: None,
      },
      hash_type::AnyLinkable::Action => {
         return inspect_ah(ActionHash::try_from(input.lh).unwrap(), input.strategy.into());
      },
      hash_type::AnyLinkable::Entry => {
         return inspect_eh(EntryHash::try_from(input.lh).unwrap(), input.strategy.into());
      },
   };
   Ok(info)
}

///
pub fn inspect_eh(eh: EntryHash, options: GetOptions) -> ExternResult<HashInfo> {
   let maybe = get(eh, options)?;
   let Some(record) = maybe else {
      return Ok(HashInfo {
         link_type: "Entry".to_string(),
         entry_type: "Unknown".to_string(),
         author: "Unknown".to_string(),
         maybe_entry_def: None,
      });
   };
   let action = record.signed_action.action();
   let action_type = action.action_type();
   let entry_type = action.entry_type().unwrap();
   let mut maybe_entry_def = None;
   if let EntryType::App(def) = entry_type.to_owned() {
      maybe_entry_def = Some(def);
   }
   let author = action.author();
   Ok(HashInfo {
      link_type: format!("Entry | {}", action_type.to_string()),
      entry_type: entry_type.to_string(),
      author: author.to_string(),
      maybe_entry_def,
   })
}

///
pub fn inspect_ah(ah: ActionHash, options: GetOptions) -> ExternResult<HashInfo> {
   let maybe = get(ah, options)?;
   let Some(record) = maybe else {
      return Ok(HashInfo {
         link_type: "Action".to_string(),
         entry_type: "Unknown".to_string(),
         author: "Unknown".to_string(),
         maybe_entry_def: None,
      });
   };
   let action = record.signed_action.action();
   let action_type = action.action_type();
   let entry_type = action.entry_type().unwrap();
   let mut maybe_entry_def = None;
   if let EntryType::App(def) = entry_type.to_owned() {
      maybe_entry_def = Some(def);
   }
   let author = action.author();
   Ok(HashInfo {
      link_type: format!("Action | {}", action_type.to_string()),
      entry_type: entry_type.to_string(),
      author: author.to_string(),
      maybe_entry_def,
   })
}
