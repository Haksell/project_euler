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
        assert_eq!(solve(), 0);
    }
}
"""

MOD_FILENAME = "src/problems/mod.rs"

assert len(sys.argv) == 2
num = int(sys.argv[1])
assert num > 0

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
        + [f"        map.insert({num}, problem{num:03}::subject as fn() -> String);"]
    )
)

open(MOD_FILENAME, "w").write(mods + "\n" + top + inserts + "\n" + bottom)
open(f"src/problems/problem{num:03}.rs", "w").write(TEMPLATE)
