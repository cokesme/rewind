
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SnapshotError {
    Generic(String),

}

impl std::fmt::Display for SnapshotError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {:?}", self)
    }

}

pub trait Snapshot {

    fn read_gpa(&self, gpa: u64, buffer: &mut [u8]) -> Result<(), SnapshotError>;

}

