use hdk::prelude::*;
//use zome_utils::*;
use crate::*;

/// Remember to call create_cap_grant() at init() for this zome function
#[hdk_extern]
fn recv_remote_signal(pulse: ExternIO) -> ExternResult<()> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   let mut pulse: ZomeSignalProtocol = pulse
      .decode()
      .map_err(|e| wasm_error!(SerializedBytesError::Deserialize(e.to_string())))?;
   // Received data is not attested, so clear validation.
   pulse.clear_validation();
   //
   let signal = ZomeSignal {
      from: call_info()?.provenance,
      pulses: vec![pulse],
   };
   return emit_signal(&signal);
}
