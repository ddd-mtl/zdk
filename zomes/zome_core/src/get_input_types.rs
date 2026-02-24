use hdk::prelude::*;

///
#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAhInput {
   pub ah: ActionHash,
   pub strategy: GetStrategy,
}

///
#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetManyAhInput {
   pub ahs: Vec<ActionHash>,
   pub strategy: GetStrategy,
}

///
#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetEhInput {
   pub eh: EntryHash,
   pub strategy: GetStrategy,
}

///
#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetManyEhInput {
   pub ehs: Vec<EntryHash>,
   pub strategy: GetStrategy,
}

///
#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLhInput {
   pub lh: AnyLinkableHash,
   pub strategy: GetStrategy,
}

///
#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetManyLhInput {
   pub lhs: Vec<AnyLinkableHash>,
   pub strategy: GetStrategy,
}

///
#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAgentInput {
   pub agent: AgentPubKey,
   pub strategy: GetStrategy,
}

///
#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetManyAgentInput {
   pub agents: Vec<AgentPubKey>,
   pub strategy: GetStrategy,
}

///
#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDhInput {
   pub dh: AnyDhtHash,
   pub strategy: GetStrategy,
}

///
#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetManyDhInput {
   pub dhs: Vec<AnyDhtHash>,
   pub strategy: GetStrategy,
}
