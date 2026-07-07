pub mod p0001;
pub mod p0002;
pub mod p0003;
pub mod p0004;
pub mod p0005;
pub mod p0006;
pub mod p0007;
pub mod p0008;
pub mod p0009;
pub mod p0010;

const SOLVED_PROBLEMS: &[u32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

pub fn solved_problem_ids() -> &'static [u32] {
    SOLVED_PROBLEMS
}

pub fn solve(id: u32) -> Option<String> {
    match id {
        1 => Some(p0001::solve()),
        2 => Some(p0002::solve()),
        3 => Some(p0003::solve()),
        4 => Some(p0004::solve()),
        5 => Some(p0005::solve()),
        6 => Some(p0006::solve()),
        7 => Some(p0007::solve()),
        8 => Some(p0008::solve()),
        9 => Some(p0009::solve()),
        10 => Some(p0010::solve()),
        _ => None,
    }
}
