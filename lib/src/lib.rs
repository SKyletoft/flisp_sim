pub mod addressing;
pub mod error;
pub mod instructions;
pub mod processor;

use addressing::*;
pub use error::FlispError;
use error::Result;
pub use instructions::Instruction;
pub use processor::Flisp;
