use std::path::Path;

use time;

mod types;
mod vars;
#[macro_use] mod load;  // macro use so the macro can be tested
mod link;
pub mod fmt;

#[cfg(test)]
mod tests;

// export for other modules to use
pub use core::vars::find_repo;
pub use core::types::{
    LoadResult, LoadError,
    Artifacts, Artifact, ArtType, ArtName, Loc,
    Settings};
pub use core::load::{parse_names, load_toml};

/// do all core loading operations defined in SPC-core-load-parts
/// includes loading and validating raw data, resolving and applying
/// variables, and linking artifacts
/// LOC-core-load-path
pub fn load_path(path: &Path) -> LoadResult<(Artifacts, Settings)>{
    let start = time::get_time();
    info!("loading path: {}", path.to_string_lossy().as_ref());
    let (mut artifacts, settings) = try!(load::load_path_raw(path));
    try!(vars::resolve_locs(&mut artifacts));
    // TODO: LOC-core-load-parts-2:<load and validate global variables>
    // LOC-core-load-parts-4:<auto-creation of missing prefix artifacts>
    link::create_parents(&mut artifacts);
    link::link_parents(&mut artifacts);
    // [TST-core-partof-vaidate]
    try!(link::validate_partof(&artifacts));
    // LOC-core-load-parts-5:<linking of artifacts>
    link::link_parts(&mut artifacts);
    link::set_completed(&mut artifacts);
    link::set_tested(&mut artifacts);
    let total = time::get_time() - start;
    info!("Done loading: {} artifacts loaded successfullly in {:.3} seconds",
          artifacts.len(), total.num_milliseconds() as f64 * 1e-3);
    Ok((artifacts, settings))
}

