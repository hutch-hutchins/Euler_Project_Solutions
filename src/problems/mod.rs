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
pub mod p0011;
pub mod p0012;
pub mod p0013;
pub mod p0014;
pub mod p0015;
pub mod p0016;
pub mod p0017;
pub mod p0018;
pub mod p0019;
pub mod p0020;

const SOLVED_PROBLEMS: &[u32] = &[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
];

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
        11 => Some(p0011::solve()),
        12 => Some(p0012::solve()),
        13 => Some(p0013::solve()),
        14 => Some(p0014::solve()),
        15 => Some(p0015::solve()),
        16 => Some(p0016::solve()),
        17 => Some(p0017::solve()),
        18 => Some(p0018::solve()),
        19 => Some(p0019::solve()),
        20 => Some(p0020::solve()),
        _ => None,
    }
}
