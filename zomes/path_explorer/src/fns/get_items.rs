use hdk::prelude::*;
use zome_utils::*;

/// LinkTypes to get
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyLinkFilter(pub Vec<(u8, Vec<u8>)>);

impl LinkTypeFilterExt for MyLinkFilter {
  fn try_into_filter(self) -> Result<LinkTypeFilter, WasmError> {
    let zomes: Vec<ZomeIndex> = self.0.iter().map(|(a, _b)| a.clone().into()).collect();
    let mut res: Vec<(ZomeIndex, Vec<LinkType>)> = Vec::new();
    let mut type_count = 0;
    for pair in self.0 {
      let lts: Vec<LinkType> = pair.1.iter().map(|a| a.clone().into()).collect();
      type_count += lts.len();
      res.push((pair.0.into(), lts))
    }
    if type_count == 0 {
      return Ok(LinkTypeFilter::Dependencies(zomes));
    }
    Ok(LinkTypeFilter::Types(res))
  }
}

///
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetItemsInput {
  pub anchor: String, // We don't need a typedAnchor here since we care only about the ItemLink type and not the Anchor type
  pub link_filter: MyLinkFilter, //TODO: Change type ro LinkTypeFilter once holochain-client-js defines it.
  pub link_tag: Option<LinkTag>,
}


///
#[hdk_extern]
pub fn get_items(input: GetItemsInput) -> ExternResult<Vec<ItemLink>> {
  let path = Path::from(&input.anchor);
  let res = get_itemlinks(path, input.link_filter.try_into_filter()?, input.link_tag.map(|a| LinkTag::from(a.clone())))
    ?;
  Ok(res)
}

