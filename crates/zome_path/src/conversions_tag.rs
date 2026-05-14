//use std::convert::Infallible;
use hdk::prelude::holo_hash::{HashType, holo_hash_decode_unchecked, holo_hash_encode};
use hdk::prelude::*;

/// Convert String to LinkTag
pub fn str2tag(tag_str: &str) -> LinkTag {
   return LinkTag::new(tag_str.as_bytes().to_vec());
}

/// Convert LinkTag to String
pub fn tag2str(tag: &LinkTag) -> ExternResult<String> {
   let vec = &tag.0;
   let Ok(str) = std::str::from_utf8(vec) else {
      return Err(wasm_error!(WasmErrorInner::Guest(
         "Failed to parse utf8 string from link tag".to_string()
      )));
   };
   Ok(str.to_string())
}

///
pub fn hash2tag<T: HashType>(hash: HoloHash<T>) -> LinkTag {
   let str = holo_hash_encode(hash.get_raw_39());
   return str2tag(&str);
}

///
pub fn tag2hash<T: HashType>(tag: &LinkTag) -> ExternResult<HoloHash<T>> {
   let hash_str = tag2str(&tag)?;
   // if hash_str == "" {
   //
   // }
   let raw_hash = holo_hash_decode_unchecked(&hash_str)
      .map_err(|e| wasm_error!(SerializedBytesError::Deserialize(e.to_string())))?;
   let hash = HoloHash::<T>::try_from_raw_39(raw_hash)
      .map_err(|e| wasm_error!(SerializedBytesError::Deserialize(e.to_string())))?;
   Ok(hash)
}

/// Convert the i64 timestamp stored in the tag as Vec<u8>
pub fn tag2Ts(tag: LinkTag) -> Timestamp {
   let bytes: [u8; 8] = tag.0.try_into().unwrap();
   let ts = i64::from_le_bytes(bytes);
   return Timestamp::from_micros(ts);
}

///
pub fn ts2Tag(ts: Timestamp) -> LinkTag {
   LinkTag::new(ts.0.to_le_bytes().to_owned())
}

///
pub fn obj2Tag<T: serde::Serialize + std::fmt::Debug + Clone>(obj: T) -> ExternResult<LinkTag> {
   let data = encode(&obj).map_err(|e| wasm_error!(SerializedBytesError::Serialize(e.to_string())))?;
   Ok(LinkTag::new(data))
}

// ///
// pub fn tag2Obj<'a, T: Deserialize<'a> + std::fmt::Debug + Clone>(tag: LinkTag) -> ExternResult<T> {
//    let data: T = decode(&tag.into_inner())
//      .map_err(|e| wasm_error!(SerializedBytesError::Deserialize(e.to_string())))?;
//   Ok(data)
// }

// ///
// pub fn obj2Tag<T: Into<hdk::prelude::SerializedBytes>>(obj: T) -> ExternResult<LinkTag> {
//   let sb = SerializedBytes::try_from(obj)?;
//   Ok(LinkTag::new(sb.bytes().to_owned()))
// }
//
//
// ///
// pub fn tag2Obj<T: From<hdk::prelude::SerializedBytes>>(tag: LinkTag) -> ExternResult<T> {
//   let res = SerializedBytes::from(UnsafeBytes::from(tag.0)).try_into()
//                                                            .map_err(|e: Infallible| wasm_error!(SerializedBytesError::Deserialize(e.to_string())))?;
//   Ok(res)
// }

#[cfg(test)]
mod tests {
   use super::*;
   use serde::{Deserialize, Serialize};

   #[test]
   fn str2tag_stores_string_bytes_in_link_tag() {
      let tag = str2tag("hello-link-tag");
      assert_eq!(tag.0, b"hello-link-tag".to_vec());
   }

   #[test]
   fn tag2str_converts_link_tag_back_to_string() {
      let tag = LinkTag::new(b"hello-link-tag".to_vec());
      let result = tag2str(&tag).expect("valid utf8 tag should convert to string");
      assert_eq!(result, "hello-link-tag");
   }

   #[test]
   fn tag2str_returns_error_for_invalid_utf8() {
      let tag = LinkTag::new(vec![0xff, 0xfe, 0xfd]);
      let result = tag2str(&tag);
      assert!(result.is_err());
   }

   #[test]
   fn hash2tag_and_tag2hash_round_trip_entry_hash() {
      let hash = EntryHash::from_raw_32_and_type(vec![42; 32], hash_type::Entry);
      let tag = hash2tag(hash.clone());
      let decoded_hash = tag2hash::<hash_type::Entry>(&tag).expect("tag should decode back to hash");
      assert_eq!(decoded_hash, hash);
   }

   #[test]
   fn tag2hash_returns_error_for_invalid_hash_tag() {
      let tag = str2tag("not-a-valid-holo-hash");
      let result = tag2hash::<hash_type::Entry>(&tag);
      assert!(result.is_err());
   }

   #[test]
   fn ts2tag_and_tag2ts_round_trip_timestamp() {
      let timestamp = Timestamp::from_micros(1_713_456_789_123_456);
      let tag = ts2Tag(timestamp);
      let decoded_timestamp = tag2Ts(tag);
      assert_eq!(decoded_timestamp, timestamp);
   }

   #[test]
   fn ts2tag_stores_timestamp_as_little_endian_i64_bytes() {
      let timestamp = Timestamp::from_micros(42);
      let tag = ts2Tag(timestamp);
      assert_eq!(tag.0, 42_i64.to_le_bytes().to_vec());
   }

   #[test]
   #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
   fn tag2ts_panics_when_tag_is_not_eight_bytes() {
      let tag = LinkTag::new(vec![1, 2, 3]);
      let _ = tag2Ts(tag);
   }

   #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
   struct TestObject {
      name: String,
      count: u32,
      active: bool,
   }

   #[test]
   fn obj2tag_serializes_object_into_link_tag() {
      let object = TestObject {
         name: "test".to_string(),
         count: 7,
         active: true,
      };
      let tag = obj2Tag(object.clone()).expect("object should serialize into link tag");
      let decoded: TestObject = decode(&tag.0).expect("serialized tag should decode back to object");
      assert_eq!(decoded, object);
   }
}
