use crate::math::isqrt;

pub fn subject() -> String {
    solve(1000).unwrap().to_string()
}

// https://en.wikipedia.org/wiki/Pythagorean_triple#Generating_a_triple
fn solve(target: u64) -> Option<u64> {
    if target & 1 == 1 {
        return None;
    }
    for m in 2u64..=isqrt(target / 2) {
        let mm = m * m;
        for n in 1..m {
            let nn = n * n;
            let a = mm - nn;
            let b = 2 * m * n;
            let c = mm + nn;
            let plus = a + b + c;
            if plus > target {
                break;
            }
            if target % plus == 0 {
                let k = target / plus;
                return Some(a * b * c * k.pow(3));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "31875000");
    }

    #[test]
    fn test_exact() {
        assert_eq!(solve(12), Some(60)); // (3,4,5)
        assert_eq!(solve(24), Some(480)); // (6,8,10)
        assert_eq!(solve(30), Some(780)); // (5,12,13)
        assert_eq!(solve(36), Some(1620)); // (9,12,15)
        assert_eq!(solve(40), Some(2040)); // (8,15,17)
        assert_eq!(solve(48), Some(3840)); // (12,16,20)
        assert_eq!(solve(1000), Some(31875000)); // (200,375,425)
    }

    #[test]
    fn test_option() {
        let triples: HashSet<u64> = HashSet::from([
            12, 24, 30, 36, 40, 48, 56, 60, 70, 72, 80, 84, 90, 96, 108, 112, 120, 126, 132, 140,
            144, 150, 154, 156, 160, 168, 176, 180, 182, 192, 198, 200, 204, 208, 210, 216, 220,
            224, 228, 234, 240, 252, 260, 264, 270, 276, 280, 286, 288, 300, 306, 308, 312, 320,
            324, 330, 336, 340, 348, 350, 352, 360, 364, 372, 374, 378, 380, 384, 390, 392, 396,
            400, 408, 416, 418, 420, 432, 440, 442, 444, 448, 450, 456, 462, 468, 476, 480, 490,
            492, 494, 504, 510, 516, 520, 528, 532, 540, 544, 546, 552, 560, 564, 570, 572, 576,
            588, 594, 598, 600, 608, 612, 616, 624, 630, 636, 640, 644, 646, 648, 650, 660, 672,
            680, 684, 690, 696, 700, 702, 704, 708, 714, 720, 728, 732, 736, 744, 748, 750, 756,
            760, 768, 770, 780, 782, 784, 792, 798, 800, 804, 810, 816, 828, 832, 836, 840, 850,
            852, 858, 864, 870, 874, 876, 880, 882, 884, 888, 896, 900, 910, 912, 918, 920, 924,
            928, 930, 936, 948, 950, 952, 960, 966, 972, 980, 984, 986, 988, 990, 992, 996,
        ]);
        for i in 0..1000 {
            match solve(i) {
                Some(_) => assert!(triples.contains(&i)),
                None => assert!(!triples.contains(&i)),
            };
        }
    }
}
