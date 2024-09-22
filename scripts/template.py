import os
import re
import sys

TEMPLATE = """\
pub fn subject() -> String {
    solve().to_string()
}

fn solve() -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 42);
    }
}
"""

MOD_FILENAME = "src/problems/mod.rs"

assert len(sys.argv) == 2
num = int(sys.argv[1])
assert num > 0

problem_filename = f"src/problems/problem{num:03}.rs"

mods, top, inserts, bottom = re.fullmatch(
    r"((?:mod problem\d{3};\n)+)(.*?)((?: +map\.insert[^\n]*;\n)+)(.*)",
    open(MOD_FILENAME).read(),
    re.DOTALL | re.MULTILINE,
).groups()

assert f"{num:03}" not in mods

mods = "\n".join(sorted(mods.splitlines() + [f"mod problem{num:03};"]))
inserts = "\n".join(
    sorted(
        inserts.splitlines()
        + [f"        map.insert({num}, problem{num:03}::subject as fn() -> String);"],
        key=lambda s: s.split("problem")[1],
    )
)

open(MOD_FILENAME, "w").write(mods + "\n" + top + inserts + "\n" + bottom)
open(problem_filename, "w").write(TEMPLATE)
os.system(f"code {problem_filename}")
