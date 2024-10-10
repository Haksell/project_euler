mod duplicates;
mod problem001;
mod problem002;
mod problem003;
mod problem004;
mod problem005;
mod problem006;
mod problem007;
mod problem008;
mod problem009;
mod problem010;
mod problem011;
mod problem012;
mod problem013;
mod problem014;
mod problem015;
mod problem016;
mod problem017;
mod problem018;
mod problem019;
mod problem020;
mod problem021;
mod problem022;
mod problem025;
mod problem067;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PROBLEMS: HashMap<usize, fn() -> String> = HashMap::from([
        (1, problem001::subject as fn() -> String),
        (2, problem002::subject as fn() -> String),
        (3, problem003::subject as fn() -> String),
        (4, problem004::subject as fn() -> String),
        (5, problem005::subject as fn() -> String),
        (6, problem006::subject as fn() -> String),
        (7, problem007::subject as fn() -> String),
        (8, problem008::subject as fn() -> String),
        (9, problem009::subject as fn() -> String),
        (10, problem010::subject as fn() -> String),
        (11, problem011::subject as fn() -> String),
        (12, problem012::subject as fn() -> String),
        (13, problem013::subject as fn() -> String),
        (14, problem014::subject as fn() -> String),
        (15, problem015::subject as fn() -> String),
        (16, problem016::subject as fn() -> String),
        (17, problem017::subject as fn() -> String),
        (18, problem018::subject as fn() -> String),
        (19, problem019::subject as fn() -> String),
        (20, problem020::subject as fn() -> String),
        (21, problem021::subject as fn() -> String),
        (22, problem022::subject as fn() -> String),
        (25, problem025::subject as fn() -> String),
        (67, problem067::subject as fn() -> String),
    ]);
}
