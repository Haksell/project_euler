mod problem001;
mod problem002;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PROBLEM_MAP: HashMap<usize, fn() -> String> = {
        let mut map = HashMap::new();
        map.insert(1, problem001::subject as fn() -> String);
        map.insert(2, problem002::subject as fn() -> String);
        map
    };
}
