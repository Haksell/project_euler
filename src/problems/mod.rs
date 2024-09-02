mod problem001;
mod problem002;
mod problem003;
mod problem006;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PROBLEMS: HashMap<usize, fn() -> String> = {
        let mut map = HashMap::new();
        map.insert(1, problem001::subject as fn() -> String);
        map.insert(2, problem002::subject as fn() -> String);
        map.insert(3, problem003::subject as fn() -> String);
        map.insert(6, problem006::subject as fn() -> String);
        map
    };
}
