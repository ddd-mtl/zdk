use hdk::prelude::*;
#[allow(unused_imports)]
use agent_directory_integrity::*;
use crate::*;

/// On init callback register self to public agent directory.
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
   let res = register_self();
   if let Err(e) = res {
      let msg = format!("Failed registering agent: {:?}", e);
      error!(msg);
      return Ok(InitCallbackResult::Fail(msg));
   }
   Ok(InitCallbackResult::Pass)
}


/// Register this agent
fn register_self() -> ExternResult<()> {
   let agent_address = agent_info()?.agent_initial_pubkey;
   /// Avoid duplicate linking if already registered
   if Ok(true) == agent_to_path(&agent_address).exists() {
      return Ok(());
   }
   /// Build path registering agent
   let agent_path = agent_to_path(&agent_address);
   agent_path.ensure()?;
   /// Done
   Ok(())
}
