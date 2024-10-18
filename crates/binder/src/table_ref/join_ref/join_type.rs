
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum JoinType {
    Invalid,
    Left,
    Right,
    Inner,
    Outer
}
