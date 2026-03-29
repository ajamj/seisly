pub mod mmap;
pub mod parser;
pub mod reader;
pub mod writer;

pub use reader::{IoError, SegyReader};
pub use writer::SegyWriter;
