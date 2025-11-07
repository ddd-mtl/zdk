use hdk::prelude::*;
use hdk::prelude::holo_hash::hash_type;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashInfo {
  link_type: String,
  info: String,
  author: String,
  maybe_entry_def: Option<AppEntryDef>,
}


///
#[hdk_extern]
pub fn inspect_link(any: AnyLinkableHash) -> ExternResult<HashInfo> {
  let info = match any.hash_type() {
    hash_type::AnyLinkable::External => {
      HashInfo {
        link_type: "External".to_string(),
        info: "External".to_string(),
        author: "unknown".to_string(),
        maybe_entry_def: None,
      }
    },
    hash_type::AnyLinkable::Action => return inspect_ah(ActionHash::try_from(any).unwrap()),
    hash_type::AnyLinkable::Entry => return inspect_eh(EntryHash::try_from(any).unwrap()),
  };
  Ok(info)
}


///
pub fn inspect_eh(eh: EntryHash) -> ExternResult<HashInfo> {
  let maybe = get(eh, GetOptions::network())?;
  let Some(record) = maybe
    else { return Ok(HashInfo {
      link_type: "Entry".to_string(),
      info: "unknown".to_string(),
      author: "unknown".to_string(),
      maybe_entry_def: None,
    })
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
    info: entry_type.to_string(),
    author: author.to_string(),
    maybe_entry_def,
  })
}


///
pub fn inspect_ah(ah: ActionHash) -> ExternResult<HashInfo> {
  let maybe = get(ah, GetOptions::network())?;
  let Some(record) = maybe
    else { return Ok(HashInfo {
      link_type: "Action".to_string(),
      info: "unknown".to_string(),
      author: "unknown".to_string(),
      maybe_entry_def: None,
    })
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
    info: entry_type.to_string(),
    author: author.to_string(),
    maybe_entry_def,
  })
}
