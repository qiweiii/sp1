use crate::runtime::Instruction;

use super::memory::MemoryRecordEnum;

/// A standard format for describing CPU operations that need to be proven.
#[derive(Debug, Copy, Clone)]
pub struct CpuEvent {
    /// The current shard.
    pub shard: u32,

    /// The current clock.
    pub clk: u32,

    /// The current program counter.
    pub pc: u32,

    /// The current instruction.
    pub instruction: Instruction,

    /// The first operand.
    pub a: u32,

    /// The memory access record for the first operand.
    pub a_record: Option<MemoryRecordEnum>,

    /// The second operand.
    pub b: u32,

    /// The memory access record for the second operand.
    pub b_record: Option<MemoryRecordEnum>,

    /// The third operand.
    pub c: u32,

    /// The memory access record for the third operand.
    pub c_record: Option<MemoryRecordEnum>,

    /// The memory value we potentially may access.
    pub memory: Option<u32>,

    /// The memory access record for the memory value.
    pub memory_record: Option<MemoryRecordEnum>,
}
