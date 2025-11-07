mod init;
mod zfns;
mod paths;

pub(crate) use paths::*;
use hdk::prelude::*;

/// Zome functions required by lit-happ framework
#[hdk_extern]
fn get_zome_info(_:()) -> ExternResult<ZomeInfo> {
  return zome_info();
}
#[hdk_extern]
fn get_dna_info(_:()) -> ExternResult<DnaInfo> {
  return dna_info();
}
