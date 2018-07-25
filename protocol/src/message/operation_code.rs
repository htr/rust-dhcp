//! DHCP message operation code module.

use std::fmt;

/// DHCP opcode.
#[derive(Clone, Copy)]
pub enum OperationCode {
    Undefined,
    BootRequest,
    BootReply,
}

impl From<u8> for OperationCode {
    fn from(value: u8) -> Self {
        use self::OperationCode::*;
        match value {
            1 => BootRequest,
            2 => BootReply,
            _ => Undefined,
        }
    }
}

impl fmt::Display for OperationCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::OperationCode::*;
        match self {
            Undefined => write!(f, "UNDEFINED"),
            BootRequest => write!(f, "BOOTREQUEST"),
            BootReply => write!(f, "BOOTREPLY"),
        }
    }
}