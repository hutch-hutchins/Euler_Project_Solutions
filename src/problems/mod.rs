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
pub mod p0021;
pub mod p0022;
pub mod p0023;
pub mod p0024;
pub mod p0025;
pub mod p0026;
pub mod p0027;
pub mod p0028;
pub mod p0029;
pub mod p0030;
pub mod p0031;
pub mod p0032;
pub mod p0033;
pub mod p0034;
pub mod p0035;
pub mod p0036;
pub mod p0037;
pub mod p0038;
pub mod p0039;
pub mod p0040;
pub mod p0041;
pub mod p0042;
pub mod p0043;
pub mod p0044;
pub mod p0045;
pub mod p0046;
pub mod p0047;
pub mod p0048;
pub mod p0049;
pub mod p0050;
pub mod p0051;
pub mod p0052;
pub mod p0053;
pub mod p0054;
pub mod p0055;
pub mod p0056;
pub mod p0057;
pub mod p0058;
pub mod p0059;
pub mod p0060;

const SOLVED_PROBLEMS: &[u32] = &[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
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
        21 => Some(p0021::solve()),
        22 => Some(p0022::solve()),
        23 => Some(p0023::solve()),
        24 => Some(p0024::solve()),
        25 => Some(p0025::solve()),
        26 => Some(p0026::solve()),
        27 => Some(p0027::solve()),
        28 => Some(p0028::solve()),
        29 => Some(p0029::solve()),
        30 => Some(p0030::solve()),
        31 => Some(p0031::solve()),
        32 => Some(p0032::solve()),
        33 => Some(p0033::solve()),
        34 => Some(p0034::solve()),
        35 => Some(p0035::solve()),
        36 => Some(p0036::solve()),
        37 => Some(p0037::solve()),
        38 => Some(p0038::solve()),
        39 => Some(p0039::solve()),
        40 => Some(p0040::solve()),
        41 => Some(p0041::solve()),
        42 => Some(p0042::solve()),
        43 => Some(p0043::solve()),
        44 => Some(p0044::solve()),
        45 => Some(p0045::solve()),
        46 => Some(p0046::solve()),
        47 => Some(p0047::solve()),
        48 => Some(p0048::solve()),
        49 => Some(p0049::solve()),
        50 => Some(p0050::solve()),
        51 => Some(p0051::solve()),
        52 => Some(p0052::solve()),
        53 => Some(p0053::solve()),
        54 => Some(p0054::solve()),
        55 => Some(p0055::solve()),
        56 => Some(p0056::solve()),
        57 => Some(p0057::solve()),
        58 => Some(p0058::solve()),
        59 => Some(p0059::solve()),
        60 => Some(p0060::solve()),
        _ => None,
    }
}
