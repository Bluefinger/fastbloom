# fastbloom
[![Crates.io](https://img.shields.io/crates/v/fastbloom.svg)](https://crates.io/crates/fastbloom)
[![docs.rs](https://docs.rs/bloomfilter/badge.svg)](https://docs.rs/fastbloom)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/tomtomwombat/fastbloom/blob/main/LICENSE-MIT)
[![License: APACHE](https://img.shields.io/badge/License-Apache-blue.svg)](https://github.com/tomtomwombat/fastbloom/blob/main/LICENSE-Apache)
<a href="https://codecov.io/gh/tomtomwombat/fastbloom">
    <img src="https://codecov.io/gh/tomtomwombat/fastbloom/branch/main/graph/badge.svg">
</a>

The fastest bloom filter in Rust. No accuracy compromises. Compatible with any hasher.


### Usage

```toml
# Cargo.toml
[dependencies]
fastbloom = "0.1.0"
```

```rust
use fastbloom::BloomFilter;

let num_bits = 1024;

let filter = BloomFilter::builder(num_bits).items(["42", "🦀"]);
assert!(filter.contains("42"));
assert!(filter.contains("🦀"));
```

```rust
use fastbloom::BloomFilter;
use ahash::RandomState;

let num_bits = 1024;

let filter = BloomFilter::builder(num_bits)
    .hasher(RandomState::default())
    .items(["42", "🦀"]);
```

### Background
Bloom filters are a space efficient approximate membership set data structure. False positives from `contains` are possible, but false negatives are not, i.e. `contains` for all items in the set is guaranteed to return true, while `contains` for all items not in the set probably return false.

Blocked bloom filters are supported by an underlying bit vector, chunked into 512, 256, 128, or 64 bit "blocks", to track item membership. To insert, a number of bits, based on the item's hash, are set in one of the underlying bit vector's block. To check membership, a number of bits, based on the item's hash, are checked in one of the underlying bit vector's block. [See more on blocked bloom filters](https://web.archive.org/web/20070623102632/http://algo2.iti.uni-karlsruhe.de/singler/publications/cacheefficientbloomfilters-wea2007.pdf).

Once constructed, neither the bloom filter's underlying memory usage nor number of bits per item change.


### Implementation

`fastbloom` is **several times faster** than existing bloomfilters and scales very well with number of hashes per item. In all cases, `fastbloom` maintains competitive false positive rates. `fastbloom` is blazingly fast because it uses L1 cache friendly blocks and efficiently derives many index bits from only one hash per value.

### Runtime Performance

#### SipHash
Runtime comparison to other bloom filter crates (all using SipHash):
> ![member](https://github.com/tomtomwombat/fastbloom/assets/45644087/6d131859-414e-47e2-958e-5675827d2138)
> ![non-member](https://github.com/tomtomwombat/fastbloom/assets/45644087/82cc2fe3-3808-462a-8105-7f88830b28d9)

#### Any Hash Goes
The fastbloom-rs crate, (similarily named), uses xxhash, which faster than SipHash, so it is not fair to compare above. However, we can configure `fastbloom` to use similarly fast hash, ahash, and compare.
> ![member-fastbloom](https://github.com/tomtomwombat/fastbloom/assets/45644087/ed84e9b6-7aab-4109-aea8-462ab97e835f)
> ![non-member-fastbloom](https://github.com/tomtomwombat/fastbloom/assets/45644087/316a691a-0e29-4de2-9f1f-cd34e7662956)


### False Positive Performance

`fastbloom` does not compromise accuracy. Below is a comparison of false positive rates with other bloom filter crates:
> ![bloom-fp](https://github.com/tomtomwombat/fastbloom/assets/45644087/f5c3af19-7467-4023-b46c-3f89b3d9d824)


## References
- [Bloom Filter](https://brilliant.org/wiki/bloom-filter/)
- [Cache-, Hash- and Space-Efficient Bloom Filters](https://web.archive.org/web/20070623102632/http://algo2.iti.uni-karlsruhe.de/singler/publications/cacheefficientbloomfilters-wea2007.pdf)
- [Less hashing, same performance: Building a better Bloom filter](https://www.eecs.harvard.edu/~michaelm/postscripts/rsa2008.pdf)
- [A fast alternative to the modulo reduction](https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/)

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.