use crate::*;
use hdi::prelude::*;

///
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
   match op {
      Op::CreateRecord(_) => Ok(ValidateCallbackResult::Valid),
      Op::CreateEntry { .. } => Ok(ValidateCallbackResult::Valid),
      Op::CreateLink(registered_create_link) => {
         let (create, signature) = registered_create_link.create_link.into_inner();
         /// `zome_index` and `link_type` are Copy, so the borrow on `create` ends here
         let (zome_index, zome_type) = match &create.content.data {
            ActionData::CreateLink(create_data) => (create_data.zome_index, create_data.link_type),
            _ => {
               return Ok(ValidateCallbackResult::Invalid(
                  "Action data is not a CreateLink".to_string(),
               ))
            },
         };
         let link_type = AgentDirectoryLinkType::try_from(ScopedLinkType { zome_index, zome_type })?;
         if link_type == AgentDirectoryLinkType::Agent {
            return validate_agent_link(create, signature);
         } else {
            Ok(ValidateCallbackResult::Invalid("Unknown link type".to_string()))
         }
      },
      Op::DeleteLink(_) => Ok(ValidateCallbackResult::Invalid(
         "Deleting links isn't allowed".to_string(),
      )),
      Op::Update { .. } => Ok(ValidateCallbackResult::Invalid(
         "Updating entries isn't allowed".to_string(),
      )),
      Op::Delete { .. } => Ok(ValidateCallbackResult::Invalid(
         "Deleting entries isn't allowed".to_string(),
      )),
      Op::AgentActivity { .. } => Ok(ValidateCallbackResult::Valid),
   }
}

/// Checks Agent Link is created by self
pub fn validate_agent_link(
   create_link: HoloHashed<Action>,
   signature: Signature,
) -> ExternResult<ValidateCallbackResult> {
   //debug!("validate_agent_link(): {:?}", create_link);
   /// Retrieve Path::Component from LinkTag
   let ActionData::CreateLink(create_data) = &create_link.content.data else {
      return Ok(ValidateCallbackResult::Invalid(
         "Action data is not a CreateLink".to_string(),
      ));
   };
   let tag_bytes = create_data.tag.clone().into_inner();
   let unsafe_bytes = UnsafeBytes::from(tag_bytes.clone());
   let ser_bytes = SerializedBytes::from(unsafe_bytes);
   let maybe_component = Component::try_from(ser_bytes);
   let Ok(component) = maybe_component else {
      return Ok(ValidateCallbackResult::Invalid(
         "Failed to convert LinkTag to Component".to_string(),
      ));
   };
   /// Retrieve AgentPubKey from Component
   let maybe_agent_key = AgentPubKey::try_from_raw_39(component.as_ref().to_vec());
   //debug!("validate_agent_link(): agent_key = {:?}", maybe_agent_key);
   /// Check key in LinkTag matches author and action signature
   let Ok(agent_key) = maybe_agent_key else {
      /// TODO: Path root is also of type Agent but does not have the LinkTag, so skip for now.
      // return Ok(ValidateCallbackResult::Invalid("Failed to convert Component to AgentPubKey".to_string()))
      return Ok(ValidateCallbackResult::Valid);
   };
   if &agent_key != create_link.content.author() {
      return Ok(ValidateCallbackResult::Invalid(
         "Link Author and Tag don't match".to_string(),
      ));
   }
   let success = verify_signature(agent_key, signature, create_link.content)?;
   Ok(if !success {
      ValidateCallbackResult::Invalid("Failed to verify signature".to_string())
   } else {
      ValidateCallbackResult::Valid
   })
}
