use std::path::PathBuf;

pub type ModulePath = PathBuf;

/// A module path is followed by `:`.
///
/// `foo:bar` should translate to `foo/bar.grape`
pub fn from(s: &str) -> ModulePath {
  PathBuf::from(s.replace(':', "/") + ".grape")
}
