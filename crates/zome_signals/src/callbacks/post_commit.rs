use crate::*;
use hdk::entry::{get, must_get_action, must_get_entry};
use hdk::prelude::*;
use std::fmt::Debug;

/// Attest Entry or Link on post_commit() as well as SystemAttestation of a PostCommit
pub fn attest_post_commit<E: UnitEnum, L: LinkTypesHelper + Debug>(signed_actions: Vec<SignedActionHashed>) {
   /// Process each Action
   for sah in signed_actions {
      let ah = sah.as_hash().to_owned();
      match sah.action() {
         ///
         Action::CreateLink(create_link) => {
            /// Get LinkType
            let Ok(Some(_link_type)) = L::from_type(create_link.zome_index, create_link.link_type) else {
               error!(
                  "CreateLink should have a LinkType. Could be a Link from a different zome: {} ({}) | {:?}",
                  create_link.link_type.0, create_link.zome_index, create_link
               );
               continue;
            };
            /// Emit Link Signal
            let res = attest_link_created(ah, create_link, true);
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
         Action::DeleteLink(delete_link) => {
            let Ok(Some(record)) = get(delete_link.link_add_address.clone(), GetOptions::local()) else {
               error!("Failed to get CreateLink action");
               continue;
            };
            let Action::CreateLink(create_link) = record.action() else {
               error!("Record should be a CreateLink");
               continue;
            };
            /// Emit Link Signal
            let res = attest_link_deleted(delete_link, create_link, true);
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
         Action::Update(_) | Action::Create(_) => {
            let EntryType::App(app_entry_def) = sah.action().entry_type().unwrap() else {
               continue;
            };
            /// Emit Entry Signal
            let result = attest_new_entry(sah.clone());
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
         Action::Delete(delete) => {
            let Ok(new_sah) = must_get_action(delete.deletes_address.clone()) else {
               error!("Deleted action not found.");
               continue;
            };
            let Ok(eh) = must_get_entry(delete.deletes_entry_address.clone()) else {
               error!("Deleted entry not found.");
               continue;
            };
            let Some(EntryType::App(app_entry_def)) = new_sah.action().entry_type() else {
               error!("Deleted action should have entry_type.");
               continue;
            };
            /// Emit Entry Signal
            let result = attest_entry_deleted(new_sah.hashed.clone(), eh.content, true);
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
