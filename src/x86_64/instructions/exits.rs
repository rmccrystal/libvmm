use super::vmcs::*;

pub struct DebugExceptionExit {
    pub exit_qual: u64,
}

impl DebugExceptionExit {
    pub fn new() -> Self {
        Self {
            exit_qual: VMCSField64ReadOnly::EXIT_QUALIFICATION.read(),
        }
    }

    pub fn is_b(&self, index: usize) -> bool {
        debug_assert!(index < 4);
        ((self.exit_qual >> index) & 1) == 1
    }

    pub fn is_bd(&self) -> bool {
        ((self.exit_qual >> 13) & 1) == 1
    }

    pub fn is_bs(&self) -> bool {
        ((self.exit_qual >> 14) & 1) == 1
    }
}

pub enum TSExitSource {
    CALL,
    IRET,
    JMP,
    IDT,
}

pub struct TSExit {
    pub exit_qual: u64,
}

impl TSExit {
    pub fn new() -> Self {
        Self {
            exit_qual: VMCSField64ReadOnly::EXIT_QUALIFICATION.read(),
        }
    }

    pub fn selector(&self) -> u16 {
        self.exit_qual as u16
    }

    pub fn source(&self) -> TSExitSource {
        match (self.exit_qual >> 30) & 0x3 {
            0 => TSExitSource::CALL,
            1 => TSExitSource::IRET,
            2 => TSExitSource::JMP,
            3 => TSExitSource::IDT,
            _ => panic!("Invalid"),
        }
    }
}

pub enum CRAccessType {
    MovToCR,
    MovFromCR,
    CLTS,
    LMSW,
}

pub struct CRAccessExit {
    pub exit_qual: u64,
}

impl CRAccessExit {
    pub fn new() -> Self {
        Self {
            exit_qual: VMCSField64ReadOnly::EXIT_QUALIFICATION.read(),
        }
    }

    pub fn cr(&self) -> usize {
        (self.exit_qual & 0xf) as usize
    }

    pub fn access_type(&self) -> CRAccessType {
        match (self.exit_qual >> 4) & 0x3 {
            0 => CRAccessType::MovToCR,
            1 => CRAccessType::MovFromCR,
            2 => CRAccessType::CLTS,
            3 => CRAccessType::LMSW,
            _ => panic!("Invalid"),
        }
    }

    pub fn is_lmsw_register(&self) -> bool {
        ((self.exit_qual >> 6) & 1) == 0
    }

    pub fn reg(&self) -> usize {
        ((self.exit_qual >> 8) & 0xf) as usize
    }

    pub fn lmsw_source(&self) -> usize {
        ((self.exit_qual >> 16) & 0xff) as usize
    }
}
