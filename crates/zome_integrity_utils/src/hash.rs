use holo_hash::*;

pub type AnyHashB64 = String;

///
pub fn any2hash<T: HashType>(any: &AnyHashB64) -> Result<HoloHash<T>, HoloHashError> {
   // let hash_str = String::try_from(comp)
   //   .map_err(|e| SerializedBytesError::Deserialize(e.to_string()))?;
   let raw_hash = holo_hash_decode_unchecked(&any)?;
   let hash = HoloHash::<T>::try_from_raw_39(raw_hash)?;
   Ok(hash)
}

///
pub fn hash2any<T: HashType>(hash: HoloHash<T>) -> AnyHashB64 {
   let str = holo_hash_encode(hash.get_raw_39());
   str.into()
}
