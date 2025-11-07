use hdk::prelude::*;
use zome_signals::*;

///
#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    std::panic::set_hook(Box::new(zome_panic_hook));
    let _ = create_signal_cap_grant()?;
    Ok(InitCallbackResult::Pass)
}


///
#[hdk_extern(infallible)]
pub fn post_commit(signed_actions: Vec<SignedActionHashed>) {
    std::panic::set_hook(Box::new(zome_panic_hook));
    attest_post_commit::<EntryTypes, LinkTypes>(signed_actions);
}

