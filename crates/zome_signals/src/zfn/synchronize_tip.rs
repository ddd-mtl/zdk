use crate::*;
use hdk::hdi::hdk_extern;
use hdk::info::agent_info;
use hdk::map_extern::ExternResult;
use hdk::prelude::*;

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SynchronizeTipInput {
   pub tip: TipProtocol,
   pub recipient: AgentPubKey,
   pub zome_name: String,
}

/// To be used by app clients for making sure a tip has been received (by using `call_remote()`).
#[hdk_extern]
pub fn synchronize_tip(input: SynchronizeTipInput) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   /// Pre-conditions: Don't call yourself (otherwise could get concurrency issues)
   if input.recipient == agent_info()?.agent_initial_pubkey {
      return Ok(());
   }
   /// Prepare payload
   let pulse = ZomeSignalProtocol::Tip(input.tip.clone());
   /// call
   let res = call_remote(
      input.recipient,
      ZomeName::from(input.zome_name),
      "recv_remote_signal".into(),
      None,
      ExternIO::encode(pulse).unwrap(),
   );
   /// Check call response
   match res {
      Err(e) => {
         error!("recv_remote_signal() failed during synchronize_tip(): {:?}", e);
         return Err(wasm_error!("recv_remote_signal() failed during synchronize_tip()"));
      },
      Ok(ZomeCallResponse::Ok(_)) => Ok(()),
      Ok(ZomeCallResponse::AuthenticationFailed(_sign, apk)) => Err(wasm_error!("recv_remote_signal() call failed: AuthenticationFailed - {:?}", apk)),
      Ok(ZomeCallResponse::Unauthorized(auth, _, _, fn_name)) => Err(wasm_error!("recv_remote_signal() call failed: Unauthorized call to {}(): {:?}", fn_name, auth)),
      Ok(ZomeCallResponse::NetworkError(e)) => Err(wasm_error!("recv_remote_signal() call failed: NetworkError: {:?}", e)),
      Ok(ZomeCallResponse::CountersigningSession(e)) => Err(wasm_error!("recv_remote_signal() call failed: CountersigningSession: {:?}", e)),
   }
}
