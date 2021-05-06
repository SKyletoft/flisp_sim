pub mod addressing;
pub mod error;
pub mod instructions;
pub mod processor;

use addressing::*;
use error::{FlispError, Result};
use instructions::Instruction;
use processor::Flisp;
