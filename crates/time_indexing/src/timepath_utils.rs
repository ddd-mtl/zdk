//use std::array::TryFromSliceError;
use chrono::{DateTime, Datelike, NaiveDate, Timelike, Utc};
use hdi::hash_path::path::{Component, TypedPath};
use hdk::prelude::*;
use zome_path::*;

///
pub fn get_component_from_link_tag(link: &Link) -> Result<Component, SerializedBytesError> {
   SerializedBytes::from(UnsafeBytes::from(link.tag.clone().into_inner())).try_into()
}

///
pub fn get_previous_hour_timestamp(time: Timestamp) -> Result<Timestamp, TimestampError> {
   time - std::time::Duration::from_secs(60 * 60)
}

///
pub fn get_timepath_leaf_value(path: &Path) -> ExternResult<i32> {
   let component = path.leaf().unwrap();
   return convert_component_to_i32(component);
}

// ///
// pub fn convert_component_to_i32(component: &Component) -> ExternResult<i32> {
//   let bytes: [u8; 4] = component
//     .as_ref()
//     .try_into()
//     .map_err(|e: TryFromSliceError| wasm_error!(e))
//     ?;
//   Ok(i32::from_be_bytes(bytes))
// }

///
pub fn convert_component_to_i32(component: &Component) -> ExternResult<i32> {
   //debug!("convert_component_to_i32() {:?}", component);
   let Ok(str) = String::try_from(component) else {
      return Err(wasm_error!(WasmErrorInner::Guest(
         "Failed to convert Component to string".to_string()
      )));
   };
   //let str = std::str::from_utf8(component.as_ref()).unwrap();
   let Ok(number) = str.parse::<i32>() else {
      return Err(wasm_error!(WasmErrorInner::Guest("Component is not i32".to_string())));
   };
   Ok(number)
}

/// Convert timestamp to timepath
pub fn ts2timepath(time: Timestamp) -> Path {
   let (secs, ns) = time.as_seconds_and_nanos();
   //let dtc = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(secs, ns).unwrap(), Utc);
   let dtc = DateTime::from_timestamp(secs, ns).unwrap();
   let mut components: Vec<Component> = Vec::new();

   components.push((dtc.year() as i32).to_string().into());
   components.push((dtc.month() as i32).to_string().into());
   components.push((dtc.day() as i32).to_string().into());
   components.push((dtc.hour() as i32).to_string().into());

   //let path = Path::from( components);
   components.into()
}

/// Convert timestamp to typed timepath
pub fn get_time_path(tp: TypedPath, time: Timestamp) -> ExternResult<TypedPath> {
   let (secs, ns) = time.as_seconds_and_nanos();
   let dtc = DateTime::from_timestamp(secs, ns).unwrap();
   let mut components: Vec<_> = tp.path.into();

   components.push((dtc.year() as i32).to_string().into());
   components.push((dtc.month() as i32).to_string().into());
   components.push((dtc.day() as i32).to_string().into());
   components.push((dtc.hour() as i32).to_string().into());

   let tp = TypedPath::new(tp.link_type, components.into());

   // let ts2 = convert_time_path_to_timestamp(tp.path.clone())?;
   // debug!("get_time_path() {} -> {} == {} ? | ", ts.as_seconds_and_nanos().0, time.as_seconds_and_nanos().0, ts2.as_seconds_and_nanos().0);

   Ok(tp)
}

///
pub fn timepath2anchor(tp: &TypedPath) -> String {
   let maybe_time_path = trim_to_timepath(&tp.path);
   if let Ok(time_path) = maybe_time_path {
      //debug!("timepath2str() FAILED for {:?}: {:?}", path2anchor(tp), maybe_time_path.err().unwrap());
      return path2anchor(&time_path).unwrap();
   };
   return path2anchor(&tp.path).unwrap();
}

///
pub fn ts2anchor(ts: Timestamp) -> String {
   let path = ts2timepath(ts);
   return path2anchor(&path).unwrap();
}

/// Possible input:
///  - 2023
///  - 2023.4.13.12
///  - all.global.2023.4.13.12
///  - all.global.2023.4
pub fn trim_to_timepath(path: &Path) -> ExternResult<Path> {
   let components = path.as_ref();

   let mut time_comps: Vec<Component> = Vec::new();
   for comp in components {
      if let Ok(_) = convert_component_to_i32(comp) {
         time_comps.push(comp.clone());
      }
   }
   if time_comps.len() > 4 {
      return Err(wasm_error!(WasmErrorInner::Guest(
         "Not a valid timepath. Too many number components found".to_string()
      )));
   }
   if time_comps.is_empty() {
      return Err(wasm_error!(WasmErrorInner::Guest(
         "Not a valid timepath. No time component found".to_string()
      )));
   }
   Ok(Path::from(time_comps))
}

///
pub fn convert_timepath_to_timestamp(path: Path) -> ExternResult<Timestamp> {
   //debug!("convert_timepath_to_timestamp() {}", path2anchor(&path).unwrap_or("<failed>".to_string()));
   let time_comps: Vec<_> = trim_to_timepath(&path)?.into();

   let len = time_comps.len();

   let year = convert_component_to_i32(&time_comps[0])?;
   let month = if len > 1 {
      convert_component_to_i32(&time_comps[1])?
   } else {
      1
   };
   let day = if len > 2 {
      convert_component_to_i32(&time_comps[2])?
   } else {
      1
   };
   let hour = if len > 3 {
      convert_component_to_i32(&time_comps[3])?
   } else {
      0
   };

   //debug!("convert_timepath_to_timestamp() {}-{}-{} {}", year, month, day, hour);

   let dtc: DateTime<Utc> = NaiveDate::from_ymd_opt(year, month as u32, day as u32)
      .unwrap()
      .and_hms_opt(hour as u32, 0, 0)
      .unwrap()
      .and_utc();

   let ts = Timestamp::from_micros(dtc.timestamp_micros());
   Ok(ts)
}

#[cfg(test)]
mod tests {
   use super::*;
   use chrono::DateTime;

   fn timestamp_from_rfc3339(value: &str) -> Timestamp {
      let dt = DateTime::parse_from_rfc3339(value).unwrap();
      Timestamp::from_micros(dt.timestamp_micros())
   }

   #[test]
   fn convert_component_to_i32_returns_number_for_numeric_component() {
      let component: Component = "42".into();
      let result = convert_component_to_i32(&component).unwrap();
      assert_eq!(result, 42);

      let component: Component = "0".into();
      let result = convert_component_to_i32(&component).unwrap();
      assert_eq!(result, 0);

      let component: Component = "-42".into();
      let result = convert_component_to_i32(&component).unwrap();
      assert_eq!(result, -42);

      let component: Component = "00420".into();
      let result = convert_component_to_i32(&component).unwrap();
      assert_eq!(result, 420);
   }

   #[test]
   fn convert_component_to_i32_returns_error_for_non_numeric_component() {
      let component: Component = "not-a-number".into();
      let result = convert_component_to_i32(&component);
      assert!(result.is_err());

      let component: Component = "".into();
      let result = convert_component_to_i32(&component);
      assert!(result.is_err());

      let component: Component = "42-".into();
      let result = convert_component_to_i32(&component);
      assert!(result.is_err());

      let component: Component = "--42".into();
      let result = convert_component_to_i32(&component);
      assert!(result.is_err());
   }

   #[test]
   fn ts2timepath_converts_timestamp_to_year_month_day_hour_path() {
      let timestamp = timestamp_from_rfc3339("2023-04-13T12:34:56Z");
      let path = ts2timepath(timestamp);
      assert_eq!(path2anchor(&path).unwrap(), "2023.4.13.12");
   }

   #[test]
   fn trim_to_timepath_keeps_only_numeric_components() {
      let path: Path = vec![
         Component::from("all"),
         Component::from("global"),
         Component::from("2023"),
         Component::from("4"),
         Component::from("13"),
         Component::from("12"),
      ]
      .into();

      let trimmed = trim_to_timepath(&path).unwrap();
      assert_eq!(path2anchor(&trimmed).unwrap(), "2023.4.13.12");
   }

   #[test]
   fn trim_to_timepath_returns_error_when_no_numeric_components_exist() {
      let path: Path = vec![Component::from("all"), Component::from("global")].into();
      let result = trim_to_timepath(&path);
      assert!(result.is_err());
   }

   #[test]
   fn trim_to_timepath_returns_error_when_too_many_numeric_components_exist() {
      let path: Path = vec![
         Component::from("2023"),
         Component::from("4"),
         Component::from("13"),
         Component::from("12"),
         Component::from("30"),
      ]
      .into();
      let result = trim_to_timepath(&path);
      assert!(result.is_err());
   }

   #[test]
   fn convert_timepath_to_timestamp_defaults_missing_month_day_and_hour() {
      let path: Path = vec![Component::from("2023")].into();
      let timestamp = convert_timepath_to_timestamp(path).unwrap();
      assert_eq!(timestamp, timestamp_from_rfc3339("2023-01-01T00:00:00Z"));
   }

   #[test]
   fn convert_timepath_to_timestamp_converts_full_timepath() {
      let path: Path = vec![
         Component::from("2023"),
         Component::from("4"),
         Component::from("13"),
         Component::from("12"),
      ]
      .into();
      let timestamp = convert_timepath_to_timestamp(path).unwrap();
      assert_eq!(timestamp, timestamp_from_rfc3339("2023-04-13T12:00:00Z"));
   }

   #[test]
   fn get_previous_hour_timestamp_subtracts_one_hour() {
      let timestamp = timestamp_from_rfc3339("2023-04-13T12:00:00Z");
      let previous = get_previous_hour_timestamp(timestamp).unwrap();
      assert_eq!(previous, timestamp_from_rfc3339("2023-04-13T11:00:00Z"));
   }

   #[test]
   fn get_previous_hour_timestamp_subtracts_one_hour_2() {
      let timestamp = timestamp_from_rfc3339("2023-04-01T00:00:00Z");
      let previous = get_previous_hour_timestamp(timestamp).unwrap();
      assert_eq!(previous, timestamp_from_rfc3339("2023-03-31T23:00:00Z"));
   }

   #[test]
   fn get_previous_hour_timestamp_subtracts_one_hour_3() {
      let timestamp = timestamp_from_rfc3339("2023-04-13T12:12:34Z");
      let previous = get_previous_hour_timestamp(timestamp).unwrap();
      assert_eq!(previous, timestamp_from_rfc3339("2023-04-13T11:12:34Z"));
   }
}
