use std::ffi::OsStr;

use arcstr::ArcStr;
use dashmap::Entry;
use sugar_path::SugarPath;

use crate::{concat_string, dashmap::FxDashMap};

pub fn make_unique_name(name: &ArcStr, used_name_counts: &FxDashMap<ArcStr, u32>) -> ArcStr {
  let mut candidate = name.clone();
  let extension = name
    .as_path()
    .extension()
    .and_then(OsStr::to_str)
    .map(|e| concat_string!(".", e))
    .unwrap_or_default();
  let file_name = &name[..name.len() - extension.len()];
  loop {
    match used_name_counts.entry(candidate.clone()) {
      Entry::Occupied(mut occ) => {
        // This name is already used
        let next_count = *occ.get();
        occ.insert(next_count + 1);
        candidate = ArcStr::from(concat_string!(
          file_name,
          itoa::Buffer::new().format(next_count),
          extension
        ));
      }
      Entry::Vacant(vac) => {
        // This is the first time we see this name
        let name = vac.key().clone();
        vac.insert(2);
        break name;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let used_name_counts = FxDashMap::default();

    let unique_name = make_unique_name(&ArcStr::from("foo.js"), &used_name_counts);
    assert_eq!(unique_name.as_str(), "foo.js");

    let unique_name = make_unique_name(&ArcStr::from("foo.js"), &used_name_counts);
    assert_eq!(unique_name.as_str(), "foo2.js");

    let unique_name = make_unique_name(&ArcStr::from("foo2.js"), &used_name_counts);
    assert_eq!(unique_name.as_str(), "foo22.js");
  }

  #[test]
  fn test2() {
    let used_name_counts = FxDashMap::default();

    let unique_name = make_unique_name(&ArcStr::from("foo.js"), &used_name_counts);
    assert_eq!(unique_name.as_str(), "foo.js");

    let unique_name = make_unique_name(&ArcStr::from("foo2.js"), &used_name_counts);
    assert_eq!(unique_name.as_str(), "foo2.js");

    let unique_name = make_unique_name(&ArcStr::from("foo.js"), &used_name_counts);
    assert_eq!(unique_name.as_str(), "foo3.js");
  }
}
