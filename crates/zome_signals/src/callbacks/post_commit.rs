use crate::*;
use hdk::entry::{get, must_get_action, must_get_entry};
use hdk::prelude::*;
use std::fmt::Debug;

/// Attest Entry or Link on post_commit() as well as SystemAttestation of a PostCommit
pub fn attest_post_commit<E: UnitEnum, L: LinkTypesHelper + Debug>(signed_actions: Vec<SignedActionHashed>)
where
   <L as LinkTypesHelper>::Error: Debug,
{
   /// Process each Action
   for sah in signed_actions {
      let ah = sah.as_hash().to_owned();
      match &sah.action().data {
         ///
         ActionData::CreateLink(create_link) => {
            /// Get LinkType
            match L::from_type(create_link.zome_index, create_link.link_type) {
               Ok(Some(_link_type)) => (),
               Ok(None) => {
                  error!(
                     "CreateLink should have a LinkType. Could be a Link from a different zome: {} ({}) | {:?}",
                     create_link.link_type.0, create_link.zome_index, create_link
                  );
                  continue;
               },
               Err(e) => {
                  error!(
                     "Getting LinkType from CreateLink failed. Could be a Link from a different zome: {} ({}) | {:?} || error: {:?}",
                     create_link.link_type.0, create_link.zome_index, create_link, e,
                  );
                  continue;
               },
            };
            /// Emit Link Signal
            let res = attest_link_created(ah, sah.action(), ValidatedBy::Me, true);
            if let Err(e) = &res {
               error!("Emitting CreateLink signal failed: {:?}", e);
            }
            let _ = emit_system_signal(SystemAttestation::PostCommitLink {
               link_type: create_link.link_type.0,
               is_delete: false,
               succeeded: res.is_ok(),
            });
         },
         ///
         ActionData::DeleteLink(delete_link) => {
            let Ok(Some(record)) = get(delete_link.link_add_address.clone(), GetOptions::local()) else {
               error!("Failed to get CreateLink action");
               continue;
            };
            let ActionData::CreateLink(create_link) = &record.action().data else {
               error!("Record should be a CreateLink");
               continue;
            };
            /// Emit Link Signal
            let res = attest_link_deleted(sah.action(), record.action(), ValidatedBy::Me, true);
            if let Err(e) = &res {
               error!("Emitting DeleteLink signal failed: {:?}", e);
            }
            let _ = emit_system_signal(SystemAttestation::PostCommitLink {
               link_type: create_link.link_type.0,
               is_delete: true,
               succeeded: res.is_ok(),
            });
         },
         /// NewEntryAction
         ActionData::Update(_) | ActionData::Create(_) => {
            let EntryType::App(app_entry_def) = sah.action().entry_type().unwrap() else {
               continue;
            };
            /// Emit Entry Signal
            let result = attest_new_entry(sah.clone(), ValidatedBy::Me);
            /// Emit System Signal
            let type_variant = get_variant_from_index::<E>(app_entry_def.entry_index).unwrap();
            let variant_name = format!("{:?}", type_variant);
            let _ = emit_system_signal(SystemAttestation::PostCommitEntry {
               app_entry_type: variant_name,
               is_delete: false,
               succeeded: result.is_ok(),
            });
            ///
            if let Err(e) = result {
               error!("<< post_commit() failed: {:?}", e);
            }
         },
         /// DeleteAction
         ActionData::Delete(delete) => {
            let Ok(new_sah) = must_get_action(delete.deletes_address.clone()) else {
               error!("Deleted action not found.");
               continue;
            };
            let Ok(create_sah) = must_get_action(delete.deletes_address.clone()) else {
               error!("Deleted entry not found.");
               continue;
            };
            let Ok(create_entry) = must_get_entry(delete.deletes_entry_address.clone()) else {
               error!("Deleted entry not found.");
               continue;
            };
            let Some(EntryType::App(app_entry_def)) = new_sah.action().entry_type() else {
               error!("Deleted action should have entry_type.");
               continue;
            };
            let create_record = Record::new(create_sah.clone(), RecordEntry::Present(create_entry.content));
            /// Emit Entry Signal
            let result = attest_entry_deleted(sah.hashed.clone(), create_record, ValidatedBy::Me, true);
            /// Emit System Signal
            let type_variant = get_variant_from_index::<E>(app_entry_def.entry_index).unwrap();
            let variant_name = format!("{:?}", type_variant);
            let _ = emit_system_signal(SystemAttestation::PostCommitEntry {
               app_entry_type: variant_name,
               is_delete: true,
               succeeded: result.is_ok(),
            });
            ///
            if let Err(e) = result {
               error!("<< attest_post_commit() failed: {:?}", e);
            }
         },
         ///
         _ => (),
      }
   }
}
