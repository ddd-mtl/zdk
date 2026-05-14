use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use hdi::hash_path::path::{Component, DELIMITER};
use hdk::prelude::holo_hash::{HashType, holo_hash_decode_unchecked, holo_hash_encode};
use hdk::prelude::*;

/// Convert Path to Anchor
pub fn path2anchor(path: &Path) -> Result<String, SerializedBytesError> {
   let mut res = String::new();
   let comps: &Vec<Component> = path.as_ref();
   for comp in comps {
      res.push_str(String::try_from(comp)?.as_str());
      res.push_str(DELIMITER);
   }
   Ok(res)
}

///
pub fn comp2hash<T: HashType>(comp: &Component) -> ExternResult<HoloHash<T>> {
   let hash_str = String::try_from(comp).map_err(|e| wasm_error!(SerializedBytesError::Deserialize(e.to_string())))?;
   let raw_hash = holo_hash_decode_unchecked(&hash_str)
      .map_err(|e| wasm_error!(SerializedBytesError::Deserialize(e.to_string())))?;
   let hash = HoloHash::<T>::try_from_raw_39(raw_hash)
      .map_err(|e| wasm_error!(SerializedBytesError::Deserialize(e.to_string())))?;
   Ok(hash)
}

///
pub fn hash2comp<T: HashType>(hash: HoloHash<T>) -> Component {
   let str = holo_hash_encode(hash.get_raw_39());
   str.into()
}

/// For some unknown reason we get a Bad Checksum error with appletIds when using comp2hash(),
/// so we are doing the decoding manually without the checksum check here.
pub fn comp2appletHash(comp: &Component) -> ExternResult<EntryHash> {
   let hash_str = String::try_from(comp).map_err(|e| wasm_error!(SerializedBytesError::Deserialize(e.to_string())))?;
   let str = &hash_str[1..]; // remove the starting 'u' char added during string::try_from()

   let raw_hash = URL_SAFE_NO_PAD
      .decode(str)
      .map_err(|e| wasm_error!(SerializedBytesError::Deserialize(e.to_string())))?;
   let eh = EntryHash::try_from_raw_39(raw_hash)
      .map_err(|e| wasm_error!(SerializedBytesError::Deserialize(e.to_string())))?;
   Ok(eh)
}

/// Convert a Component stored in a LinkTag to a String
/// TODO: Check if same as get_component_from_link_tag()
pub fn compTag2str(tag: &LinkTag) -> Result<String, SerializedBytesError> {
   if tag.0.len() <= 2 {
      return Err(SerializedBytesError::Deserialize("LinkTag not a Component".to_string()));
   }
   let vec = tag.0[2..].to_vec();
   let comp = Component::from(vec);
   let res = String::try_from(&comp)?;
   Ok(res)
}

/// Convert a Component stored in a LinkTag to a Component
pub fn compTag2tag(tag: &LinkTag) -> Component {
   let tag2 = tag.0.clone().split_off(2);
   let comp: Component = tag2.clone().into();
   comp
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn comp2hash_converts_string() {
      let str = "uhCEkAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIgST39";
      let comp = Component::from(str);
      let decoded_hash = comp2hash::<hash_type::Entry>(&comp).expect("component should decode to hash");
      let comp = hash2comp(decoded_hash.clone());
      let decoded_str = String::try_from(&comp).expect("component should decode to string");
      assert_eq!(decoded_str, str);
   }

   #[test]
   fn comp2hash_converts_component_back_to_original_hash() {
      let hash = EntryHash::from_raw_32_and_type(vec![0; 32], hash_type::Entry);
      let comp = hash2comp(hash.clone());
      let decoded_hash = comp2hash::<hash_type::Entry>(&comp).expect("component should decode to hash");
      assert_eq!(decoded_hash, hash);
   }

   #[test]
   fn comp2hash_converts_component_back_to_original_hash_2() {
      let hash = EntryHash::from_raw_32_and_type(vec![42; 32], hash_type::Entry);
      let comp = hash2comp(hash.clone());
      let decoded_hash = comp2hash::<hash_type::Entry>(&comp).expect("component should decode to hash");
      assert_eq!(decoded_hash, hash);
   }

   #[test]
   fn comp2applethash_converts_component_back_to_original_hash() {
      let hash = EntryHash::from_raw_32_and_type(vec![42; 32], hash_type::Entry);
      let comp = hash2comp(hash.clone());
      let decoded_hash = comp2appletHash(&comp).expect("component should decode to hash");
      assert_eq!(decoded_hash, hash);
   }

   #[test]
   fn comp2hash_returns_error_for_invalid_component() {
      let comp: Component = "not-a-valid-holo-hash".into();
      let result = comp2hash::<hash_type::Entry>(&comp);
      assert!(result.is_err());
   }

   #[test]
   fn path2anchor_test() {
      let hash = EntryHash::from_raw_32_and_type(vec![0; 32], hash_type::Entry);
      let comp = hash2comp(hash.clone());
      let hash2 = EntryHash::from_raw_32_and_type(vec![2; 32], hash_type::Entry);
      let comp2 = hash2comp(hash2.clone());

      let path = Path::from(vec![comp.clone(), comp2.clone()]);

      let anchor = path2anchor(&path).expect("path2anchor should work");
      let res = anchor.split(DELIMITER).collect::<Vec<&str>>();
      //println!("comp: {:?}", comp);
      //println!("res: {:?}", res);
      assert_eq!(res.len(), 3);
      //assert_eq!(res[0].as_bytes(), comp.as_ref());
      //assert_eq!(res[1].as_bytes(), comp2.as_ref());

      let decoded_hash = comp2hash::<hash_type::Entry>(&comp).expect("component should decode to hash");
      let decoded_hash2 = comp2hash::<hash_type::Entry>(&comp2).expect("component should decode to hash");

      assert_eq!(decoded_hash, hash);
      assert_eq!(decoded_hash2, hash2);
   }

   // TODO: Figure out how to test compTag2str() & compTag2tag()
   // #[test]
   // fn compTag2str_test() {
   //    let str = "hello-tester";
   //    let vec = str[2..].as_bytes().to_vec();
   //    let comp: Component = vec.into();
   //
   //    let tag = LinkTag::new(comp);
   //    println!("tag: {:?}", tag);
   //    let res = compTag2str(&tag).expect("compTag2str should work");
   //    println!("res: {:?}", res);
   //    assert_eq!(res, "hello-tester");
   // }
   //
   // #[test]
   // fn compTag2str_test2() {
   //    let tag = LinkTag::new("".as_bytes().to_vec());
   //    let res = compTag2str(&tag).expect("compTag2str should work");
   //    assert_eq!(res, "");
   // }
   //
   // #[test]
   // fn compTag2tag_test() {
   //    let str = "uhCEkAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIgST39";
   //    let tag = LinkTag::new(str.as_bytes().to_vec());
   //
   //    let comp = compTag2tag(&tag);
   //    let decoded_hash = comp2hash::<hash_type::Entry>(&comp).expect("component should decode to hash");
   //    let comp = hash2comp(decoded_hash.clone());
   //    let decoded_str = String::try_from(&comp).expect("component should decode to string");
   //    assert_eq!(decoded_str, str);
   // }
}
