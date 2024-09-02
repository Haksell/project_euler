mod problem001;
mod problem002;
mod problem003;
mod problem004;
mod problem005;
mod problem006;
mod problem007;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PROBLEMS: HashMap<usize, fn() -> String> = {
        let mut map = HashMap::new();
        map.insert(1, problem001::subject as fn() -> String);
        map.insert(2, problem002::subject as fn() -> String);
        map.insert(3, problem003::subject as fn() -> String);
        map.insert(4, problem004::subject as fn() -> String);
        map.insert(5, problem005::subject as fn() -> String);
        map.insert(6, problem006::subject as fn() -> String);
        map.insert(7, problem007::subject as fn() -> String);
        map
    };
}
