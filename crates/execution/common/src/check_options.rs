use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CheckOptions: u8 {
        // NLJ
        const NestedLoopJoin = 0b00000001;
        const TopN = 0b00000010;
    }
}
