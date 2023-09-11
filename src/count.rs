use std::path::Path;

use cargo_geiger_serde::CounterBlock;
use geiger::{find_unsafe_in_file, IncludeTests};
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Error, Debug)]
pub enum FindUnsafeError {
    #[error("walkdir error")]
    WalkDir(#[from] walkdir::Error),
    #[error("geiger error")]
    Geiger(#[from] geiger::ScanFileError),
}

pub fn find_unsafe_recursively(path: &Path) -> Result<CounterBlock, FindUnsafeError> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| {
            entry
                .map(|entry| {
                    (entry.file_type().is_file()
                        && match entry.path().extension() {
                            Some(extension) => extension == "rs",
                            None => false,
                        })
                    .then_some(entry.into_path())
                })
                .map_err(FindUnsafeError::from)
                .transpose()
        })
        .map(|path| {
            path.and_then(|path| {
                find_unsafe_in_file(&path, IncludeTests::Yes)
                    .map(|metrics| metrics.counters)
                    .map_err(FindUnsafeError::from)
            })
        })
        .try_fold(CounterBlock::default(), |sum, x| x.map(|x| sum + x))
}
