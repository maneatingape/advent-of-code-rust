//! # Infinite Elves and Infinite Houses
//!
//! ## Part One
//!
//! The amount of presents that each house receives is 10 times the
//! [divisor function](https://en.wikipedia.org/wiki/Divisor_function) `σ`.
//! For example the divisors of 6 are 1, 2, 3 and 6, so house 6 receives
//! 10 + 20 + 30 + 60 = 120 presents. The answer will be a
//! [highly abundant number](https://en.wikipedia.org/wiki/Highly_abundant_number).
//!
//! If `n` has the prime factorization `n = p₁^a₁ × p₂^a₂ × ... × pₖ^aₖ` then the sum of divisors is
//! `σ(n) = [(p₁^(a₁+1) - 1)/(p₁ - 1)] × [(p₂^(a₂+1) - 1)/(p₂ - 1)] × ... × [(pₖ^(aₖ+1) - 1)/(pₖ - 1)]`
//! or more compactly `σ(n) = ∏ᵢ₌₁ᵏ [(pᵢ^(aᵢ+1) - 1)/(pᵢ - 1)]`
//!
//! For example `n = 12 = 2² × 3¹`
//!
//! * `σ(12) = [(2³ - 1)/(2 - 1)] × [(3² - 1)/(3 - 1)]`
//! * `[(8 - 1)/1] × [(9 - 1)/2] = 7 × 4 = 28`
//!
//! It is easy enough to pre-generate a list of highly-abundant numbers, or
//! even just grab one from [OEIS A002093](https://oeis.org/A002093/b002093.txt).
//! Inspecting that list shows that between house numbers `540_540` and `1_201_200`,
//! there are only 28 candidates.  In turn, it is easy to precompute their
//! sum of divisors, turning this into a LUT (lookup table) on the target,
//! sufficient to cover the range of all known puzzle inputs (30-40 million).
//!
//! ## Part Two
//!
//! As with part one, an offline brute force search showed that for all house numbers between
//! `540_540` and `1_201_200`, there are only 46 record-holders; just use another LUT.
use crate::util::parse::*;

struct Mapping {
    sum: u32,
    house: u32,
}

pub fn parse(input: &str) -> u32 {
    input.unsigned()
}

pub fn part1(input: &u32) -> u32 {
    #[expect(clippy::decimal_literal_representation)]
    const MAPPING: [Mapping; 28] = [
        Mapping { sum: 22_579_200, house: 540_540 },
        Mapping { sum: 24_373_440, house: 554_400 },
        Mapping { sum: 24_624_000, house: 582_120 },
        Mapping { sum: 25_206_720, house: 589_680 },
        Mapping { sum: 25_296_000, house: 604_800 },
        Mapping { sum: 25_727_520, house: 609_840 },
        Mapping { sum: 26_208_000, house: 622_440 },
        Mapping { sum: 26_956_800, house: 637_560 },
        Mapping { sum: 28_435_680, house: 655_200 },
        Mapping { sum: 29_260_800, house: 665_280 },
        Mapping { sum: 29_760_000, house: 718_200 },
        Mapping { sum: 32_497_920, house: 720_720 },
        Mapping { sum: 33_611_760, house: 776_160 },
        Mapping { sum: 34_137_600, house: 786_240 },
        Mapping { sum: 36_902_400, house: 831_600 },
        Mapping { sum: 38_263_680, house: 887_040 },
        Mapping { sum: 38_304_000, house: 914_760 },
        Mapping { sum: 39_213_720, house: 917_280 },
        Mapping { sum: 41_783_040, house: 942_480 },
        Mapping { sum: 43_052_800, house: 982_800 },
        Mapping { sum: 43_908_480, house: 997_920 },
        Mapping { sum: 44_640_960, house: 1_048_320 },
        Mapping { sum: 46_425_600, house: 1_053_360 },
        Mapping { sum: 48_384_000, house: 1_081_080 },
        Mapping { sum: 49_133_760, house: 1_108_800 },
        Mapping { sum: 50_889_600, house: 1_164_240 },
        Mapping { sum: 51_226_560, house: 1_179_360 },
        Mapping { sum: 51_663_360, house: 1_201_200 },
    ];
    lookup(&MAPPING, *input)
}

pub fn part2(input: &u32) -> u32 {
    #[expect(clippy::decimal_literal_representation)]
    const MAPPING: [Mapping; 46] = [
        Mapping { sum: 22_103_774, house: 540_540 },
        Mapping { sum: 22_113_630, house: 544_320 },
        Mapping { sum: 23_818_179, house: 554_400 },
        Mapping { sum: 23_841_125, house: 574_560 },
        Mapping { sum: 23_909_886, house: 579_600 },
        Mapping { sum: 24_259_015, house: 582_120 },
        Mapping { sum: 24_668_490, house: 589_680 },
        Mapping { sum: 24_969_868, house: 604_800 },
        Mapping { sum: 25_587_870, house: 609_840 },
        Mapping { sum: 25_755_345, house: 622_440 },
        Mapping { sum: 25_941_795, house: 635_040 },
        Mapping { sum: 26_623_905, house: 637_560 },
        Mapping { sum: 27_800_157, house: 655_200 },
        Mapping { sum: 28_413_770, house: 665_280 },
        Mapping { sum: 28_899_255, house: 693_000 },
        Mapping { sum: 29_002_446, house: 705_600 },
        Mapping { sum: 29_370_187, house: 718_200 },
        Mapping { sum: 31_358_250, house: 720_720 },
        Mapping { sum: 31_476_060, house: 766_080 },
        Mapping { sum: 31_811_010, house: 771_120 },
        Mapping { sum: 33_007_425, house: 776_160 },
        Mapping { sum: 33_161_590, house: 786_240 },
        Mapping { sum: 33_297_495, house: 803_880 },
        Mapping { sum: 33_717_915, house: 819_000 },
        Mapping { sum: 35_780_206, house: 831_600 },
        Mapping { sum: 35_856_513, house: 856_800 },
        Mapping { sum: 35_960_155, house: 876_960 },
        Mapping { sum: 36_191_925, house: 884_520 },
        Mapping { sum: 37_523_640, house: 887_040 },
        Mapping { sum: 37_915_955, house: 914_760 },
        Mapping { sum: 38_520_735, house: 917_280 },
        Mapping { sum: 40_459_650, house: 942_480 },
        Mapping { sum: 40_676_757, house: 970_200 },
        Mapping { sum: 41_762_798, house: 982_800 },
        Mapping { sum: 42_620_655, house: 997_920 },
        Mapping { sum: 42_768_110, house: 1_028_160 },
        Mapping { sum: 42_774_270, house: 1_043_280 },
        Mapping { sum: 43_788_360, house: 1_048_320 },
        Mapping { sum: 45_111_990, house: 1_053_360 },
        Mapping { sum: 46_486_825, house: 1_081_080 },
        Mapping { sum: 47_636_358, house: 1_108_800 },
        Mapping { sum: 47_682_250, house: 1_149_120 },
        Mapping { sum: 48_218_247, house: 1_159_200 },
        Mapping { sum: 49_585_250, house: 1_164_240 },
        Mapping { sum: 49_742_385, house: 1_179_360 },
        Mapping { sum: 50_193_682, house: 1_201_200 },
    ];
    lookup(&MAPPING, *input)
}

fn lookup(data: &[Mapping], target: u32) -> u32 {
    let idx = data.partition_point(|entry| entry.sum < target);
    data[idx].house
}
