use chrono::{DateTime, Utc};
use hdi::prelude::AnyLinkableHash;
use hdk::prelude::{ExternResult};

pub trait IndexableHash {
    ///Time that entry type this trait is implemented on should be indexed under
    fn entry_time(&self) -> DateTime<Utc>;
    fn hash(&self) -> ExternResult<AnyLinkableHash>;
}
