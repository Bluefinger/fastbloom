use std::ops::{AddAssign, BitAndAssign, BitOrAssign};
use wide::u64x4;

const MIN_TARGET_BITS: u64 = 8;

#[inline]
pub(crate) fn hashes_for_bits(target_bits_per_u64_per_item: u64) -> f64 {
    f64::ln(-(((target_bits_per_u64_per_item as f64) / 64.0f64) - 1.0f64))
        / f64::ln(63.0f64 / 64.0f64)
}

#[inline]
pub(crate) fn work(bits: u64) -> u64 {
    match bits {
        8 => 3,
        9 => 6,
        10 => 5,
        11 => 6,
        12 => 4,
        13 => 6,
        14 => 5,
        15 => 6,
        16 => 2,
        17 => 6,
        18 => 5,
        19 => 6,
        20 => 4,
        21 => 6,
        22 => 5,
        23 => 6,
        24 => 3,
        25 => 6,
        26 => 5,
        27 => 6,
        28 => 4,
        29 => 6,
        30 => 5,
        31 => 6,
        32 => 1,
        _ => 100000,
    }
}

impl Signature for u64 {
    #[inline]
    fn h1(h1: &mut u64, _: u64) -> Self {
        *h1
    }
    #[inline]
    fn h2(h2: u64) -> Self {
        h2
    }
    #[inline]
    fn matches(data: &[u64], x: Self) -> bool {
        (data[0] & x) == x
    }
    #[inline]
    fn set(data: &mut [u64], x: Self) {
        data[0] |= x;
    }

    /// "Double hashing" produces a new hash efficiently from two orignal hashes.
    ///
    /// Modified from <https://www.eecs.harvard.edu/~michaelm/postscripts/rsa2008.pdf>.
    #[inline]
    fn next_hash(h1: &mut Self, h2: Self) -> Self {
        *h1 = h1.wrapping_add(h2).rotate_left(5);
        *h1
    }
}

impl Signature for u64x4 {
    #[inline]
    fn h1(h1: &mut u64, h2: u64) -> Self {
        [
            u64::next_hash(h1, h2),
            u64::next_hash(h1, h2),
            u64::next_hash(h1, h2),
            u64::next_hash(h1, h2),
        ]
        .into()
    }
    #[inline]
    fn h2(h2: u64) -> Self {
        Self::splat(h2.wrapping_mul(4))
    }
    #[inline]
    fn matches(data: &[u64], x: Self) -> bool {
        let t = unsafe { std::mem::transmute::<&[u64], &[Self]>(data) };
        (t[0] & x) == x
    }
    #[inline]
    fn set(data: &mut [u64], x: Self) {
        let t = unsafe { std::mem::transmute::<&mut [u64], &mut [Self]>(data) };
        t[0] |= x;
    }
}

pub(crate) trait Signature: Sized + AddAssign + Copy + BitAndAssign + BitOrAssign {
    fn h1(h1: &mut u64, h2: u64) -> Self;
    fn h2(h2: u64) -> Self;
    fn matches(data: &[u64], x: Self) -> bool;
    fn set(data: &mut [u64], x: Self);
    #[inline]
    fn next_hash(h1: &mut Self, h2: Self) -> Self {
        *h1 += h2;
        *h1
    }

    /// Returns a `u64` hash with approx `num_bits` number of bits set.
    ///
    /// Using the raw hash is very fast way of setting ~32 random bits
    /// For example, using the intersection of two raw hashes is very fast way of setting ~16 random bits.
    /// etc
    ///
    /// If the bloom filter has a higher number of hashes to be performed per item,
    /// we can use "signatures" to quickly get many index bits, and use traditional
    /// index setting for the remainder of hashes.
    #[inline]
    fn signature(h1: &mut Self, h2: Self, num_bits: u64) -> Self {
        let mut d = Self::next_hash(h1, h2);
        match num_bits {
            8 => {
                d &= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            9 => {
                d &= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            10 => {
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            11 => {
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            12 => {
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            13 => {
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            14 => {
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            15 => {
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            16 => {
                d &= Self::next_hash(h1, h2);
            }
            17 => {
                d &= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            18 => {
                d &= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            19 => {
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            20 => {
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            21 => {
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            22 => {
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            23 => {
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            24 => {
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            25 => {
                d &= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            26 => {
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            27 => {
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            28 => {
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            29 => {
                d &= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            30 => {
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            31 => {
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d |= Self::next_hash(h1, h2);
                d &= Self::next_hash(h1, h2);
            }
            _ => {}
        }
        d
    }
}

pub(crate) fn optimize_hashing(total_num_hashes: f64, block_size: usize) -> (u64, Option<u64>) {
    let num_u64s_per_block = (block_size as u64 / 64) as f64;
    let mut num_hashes = if block_size == 512 {
        total_num_hashes.round() as u64
    } else {
        total_num_hashes.floor() as u64
    };
    let mut num_rounds = None;

    // We will not accept rounds too low the variance is too high, and bits may be 0, which is bad for false positives.
    // TODO: a more precise formula for this
    let min_target_bits = match block_size {
        512 => MIN_TARGET_BITS,
        _ => 16,
    };

    for target_bits_per_u64_per_item in min_target_bits..=32 {
        let hashes_covered = hashes_for_bits(target_bits_per_u64_per_item);
        let remaining = (total_num_hashes - (hashes_covered * num_u64s_per_block)).round();
        if remaining < 0.0 {
            continue; // signature has too many bits
        }
        let hashing_work = remaining as u64;
        let work_for_target_bits = work(target_bits_per_u64_per_item);
        let cur_work = num_hashes + num_rounds.unwrap_or(0);
        if (hashing_work + work_for_target_bits) < cur_work {
            num_rounds = Some(target_bits_per_u64_per_item);
            num_hashes = hashing_work;
        }
    }
    (num_hashes, num_rounds)
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::rngs::StdRng;
    use rand::Rng;
    use rand::SeedableRng;

    #[test]
    fn test_num_bits() {
        let mut rng = StdRng::seed_from_u64(42);
        for target_bits in MIN_TARGET_BITS..=32 {
            let trials = 10_000;
            let mut total_bits = 0;
            for _ in 0..trials {
                let mut h1 = rng.gen();
                let h2 = rng.gen();
                let h = u64::signature(&mut h1, h2, target_bits);
                total_bits += h.count_ones();
            }
            assert_eq!(
                ((total_bits as f64) / (trials as f64)).round() as u64,
                target_bits
            )
        }
    }

    #[test]
    fn hash_creation() {
        for block_size in [64, 128, 256, 512] {
            for num_hashes in 0..5000 {
                let (hashes, num_rounds) = optimize_hashing(num_hashes as f64, block_size);
                assert!(num_rounds.unwrap_or(0) <= 64);
                match num_rounds {
                    None => assert_eq!(num_hashes, hashes, "None"),
                    Some(x) => {
                        let hashes_for_rounds =
                            (hashes_for_bits(x) * (block_size / 64) as f64).round() as u64;
                        assert_eq!(hashes_for_rounds + hashes, num_hashes,
                        "\ntarget hashes: {num_hashes:}\nhashes for rounds {hashes_for_rounds:}\nrounds {x:}\nhashes: {hashes:}")
                    }
                }
            }
        }
    }

    #[test]
    fn test_work_for_unknown_bits() {
        for i in 33..=1000 {
            for j in MIN_TARGET_BITS..=32 {
                assert!(work(j) < work(i));
            }
        }
    }
}
