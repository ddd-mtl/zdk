use crate::*;
use hdk::prelude::*;

///
#[derive(Serialize, Deserialize, Debug)]
pub struct ZomeSignal {
   pub from: AgentPubKey,
   pub pulses: Vec<ZomeSignalProtocol>,
}

///
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub enum ZomeSignalProtocol {
   System(SystemAttestation),
   Entry(EntryPulse),
   Link(LinkPulse),
   Tip(TipProtocol),
}

impl ZomeSignalProtocol {
   // Reset ValidationBy to None
   pub fn clear_validation(&mut self) {
      match self {
         ZomeSignalProtocol::Entry(e) => {
            e.clear_validation();
         },
         ZomeSignalProtocol::Link(l) => {
            l.clear_validation();
         },
         ZomeSignalProtocol::Tip(t) => match t {
            TipProtocol::Entry(e) => {
               e.clear_validation();
            },
            TipProtocol::Link(l) => {
               l.clear_validation();
            },
            _ => (),
         },
         _ => (),
      };
   }
}

/// App-agnostic attestation
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
#[serde(tag = "type")]
pub enum SystemAttestation {
   PostCommitLink {
      link_type: u8,
      is_delete: bool,
      succeeded: bool,
   },
   PostCommitEntry {
      app_entry_type: String,
      is_delete: bool,
      succeeded: bool,
   },
   SelfCallStart {
      zome_name: String,
      fn_name: String,
   },
   SelfCallEnd {
      zome_name: String,
      fn_name: String,
      succeeded: bool,
   },
}

/// Protocol used by UI ONLY to send data to other agents
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub enum TipProtocol {
   Ping(AgentPubKey),
   Pong(AgentPubKey),
   Entry(EntryPulse),
   Link(LinkPulse),
   AppCustom(SerializedBytes), // App specific data and structure
   AppValue((String, String)), // Generic key-Value pair
}
