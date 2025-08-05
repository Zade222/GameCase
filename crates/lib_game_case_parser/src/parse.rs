use std::io::{Read};

use ebml_iterable::{TagIterator};

use crate::gc_ebml_spec::GCEbmlSpec;
use crate::lib_error_handling::LibError;

pub fn process_ebml_data<R: Read>(
    source: R
) -> Result<bool, LibError> {
    let mut tag_iterator: TagIterator<_, GCEbmlSpec> = 
        TagIterator::new(source, &[]);

    Ok(true)
}