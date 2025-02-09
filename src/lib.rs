//! Low-level date algorithms for libraries
//!
//! This library aims to provide the **highest performance algorithms** for date
//! manipulation in an unopinionated way. It is meant to be used by the various
//! date and time libraries which can then provide ergonomic and opinionated
//! interfaces for their users.
//!
//! # Usage
//!
//! The primary contribution of this crate for date libraries are the
//! conversions between a day number from Unix epoch (January 1st, 1970) and a
//! Gregorian date:
//!
//! ```
//! use datealgo::{rd_to_date, date_to_rd};
//!
//! assert_eq!(date_to_rd((1970, 1, 1)), 0);
//! assert_eq!(date_to_rd((2023, 5, 12)), 19489);
//! assert_eq!(rd_to_date(19489), (2023, 5, 12));
//! ```
//!
//! For convenience, there is also converters to and from Unix timestamps:
//!
//! ```
//! use datealgo::{secs_to_datetime, datetime_to_secs};
//!
//! assert_eq!(datetime_to_secs((1970, 1, 1, 0, 0, 0)), 0);
//! assert_eq!(datetime_to_secs((2023, 5, 20, 9, 24, 38)), 1684574678);
//! assert_eq!(secs_to_datetime(1684574678), (2023, 5, 20, 9, 24, 38));
//! ```
//!
//! If the `std` feature is enabled, there are also converters to and from
//! `SystemTime`:
//!
//! ```
//! use datealgo::{systemtime_to_datetime, datetime_to_systemtime};
//! use std::time::{Duration, UNIX_EPOCH};
//!
//! assert_eq!(systemtime_to_datetime(UNIX_EPOCH), Some((1970, 1, 1, 0, 0, 0, 0)));
//! assert_eq!(systemtime_to_datetime(UNIX_EPOCH + Duration::from_secs(1684574678)), Some((2023, 5, 20, 9, 24, 38, 0)));
//! assert_eq!(datetime_to_systemtime((2023, 5, 20, 9, 24, 38, 0)), UNIX_EPOCH.checked_add(Duration::from_secs(1684574678)));
//! ```
//!
//! # Features
//!
//! The crate works in `no_std` environments and has no allocations. Most of the
//! functions also work in constant contexts.
//!
//! - `std` (default): Include `SystemTime` conversions
//!
//! # Background
//!
//! There are many date and time libraries for Rust for varying use cases as the
//! standard library doesn't include any utilities for dealing with dates. Most
//! of these libraries contain their own copies of date algorithms, most
//! prominently the conversion from days since an epoch to a Gregorian calendar
//! date (year, month, day). These algorithms have been sourced from various
//! places with various licenses, often translated either by machine or by hand
//! from C algorithms found in different libc variants. The algorithms are
//! usually somewhat optimized for performance, but fall short of fastest
//! algorithms available.
//!
//! # Notes
//!
//! The library does not expose any kind of `Date` or `DateTime` structures, but
//! simply tuples for the necessary values. Bounds checking is done via
//! `debug_assert` only, which means the methods are guaranteed to not panic in
//! release builds. Callers are required to do their own bounds checking.
//! Datatypes are selected as the smallest that will fit the value.
//!
//! Currently the library implements algorithms for the [Proleptic Gregorian
//! Calendar](https://en.wikipedia.org/wiki/Proleptic_Gregorian_calendar) which
//! is our current calendar extended backwards indefinitely. The Gregorian
//! calendar defines the average year to be 365.2425 days long by defining every
//! fourth year to be a leap year, unless the year is divisible by 100 and not
//! by 400.
//!
//! The algorithms do not account for leap seconds, as is customary for [Unix
//! time](https://en.wikipedia.org/wiki/Unix_time). Every day is exactly 86400
//! seconds in length, and the calculated times do not adjust for leap seconds
//! between timestamps.
//!
//! We define [Rata Die](https://en.wikipedia.org/wiki/Rata_Die) to be integral
//! day numbers counted from 1st of January, 1970, which is the Unix epoch. We
//! use the abbreviation `rd` to concisely refer to such values. This differs
//! from the epoch originally chosen by Howard Jacobson, but is more convenient
//! for usage.
//!
//! The Rata Die values are represented as `i32` for performance reasons. The
//! needed calculations reduce that to roughly an effective `i30` integer range,
//! which means a usable range of roughly -1,460,000 to 1,460,000 years.
//!
//! # Benchmarks
//!
//! Results on Intel(R) Core(TM) i9-10900K CPU @ 3.70GHz:
//!
//! | Function | [datealgo](https://github.com/nakedible/datealgo-rs) | [hinnant](https://howardhinnant.github.io/date_algorithms.html) | [httpdate](https://github.com/pyfisch/httpdate) | [humantime](https://github.com/tailhook/humantime) | [time](https://github.com/time-rs/time) | [chrono](https://github.com/chronotope/chrono) |
//! | ---------------------- | ------------- | --------- | --------- | --------- | --------- | --------- |
//! | date_to_rd | **2.1 ns** | 3.3 ns | 3.3 ns | 3.6 ns | 15.1 ns | 6.5 ns |
//! | rd_to_date | **3.2 ns** | 7.6 ns | 13.5 ns | 13.5 ns | 24.3 ns | 8 ns |
//! | datetime_to_systemtime | **5.1 ns** | | 8.8 ns | 9 ns | 31.3 ns | 22.8 ns |
//! | systemtime_to_datetime | **17.8 ns** | | 28.4 ns | 30.9 ns | 44.1 ns | 98.4 ns |
//!
//! Reliable and reproducible microbenchmarks are extremely hard to obtain with
//! modern processors. And even then, they are of limited use as the surrounding
//! code will dictate a lot about the performance. These benchmarks are not
//! meant to be authoritative, but rather illustrate the likely relative speed
//! differences of the algorithms. Your mileage will vary, so always benchmark
//! the real use case.
//!
//! # Acknowledgements
//!
//! This crate is based publicly available algorithms from many sources, but
//! most notably Cassio Neri. He has directly contributed to this crate with
//! novel algoritms and has provided valuable feedback and suggestions.
//!
//! - [Cassio Neri and Lorenz
//!   Schneider](https://onlinelibrary.wiley.com/doi/full/10.1002/spe.3172):
//!   While searching for best method for date conversion, I stumbled upon a
//!   research paper which explains a novel way to optimize the performance.
//!   These algorithms have been implemented here based on the published
//!   article. This wouldn't be the best performing date conversion library
//!   without their work.
//! - [Howard Hinnant](https://howardhinnant.github.io/date_algorithms.html):
//!   While searching for "perpetual calendar" algorithms, and having already
//!   started my library, I stumbled upon a very similar idea by Howard Hinnant.
//!   It remains one of the cleanest and simplest algorithms while still
//!   retaining excellent performance.
//! - [Rich
//!   Felker](https://git.musl-libc.org/cgit/musl/tree/src/time/__secs_to_tm.c):
//!   The original musl `__time_to_tm` function has spread far and wide and been
//!   translated to many languages, and is still the basis of many of the
//!   standalone implementations littered among the libraries.
//! - [Many authors of newlib
//!   `gmtime_r.c`](https://sourceware.org/git/?p=newlib-cygwin.git;a=blob;f=newlib/libc/time/gmtime_r.c;hb=HEAD):
//!   The newlib implementation has evolved significantly over time and has now
//!   been updated based on the work by Howard Hinnant.
#![forbid(unsafe_code)]
#![allow(clippy::absurd_extreme_comparisons, clippy::manual_range_contains)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Adjustment from Unix epoch to make calculations use positive integers
///
/// Unit is eras, which is defined to be 400 years, as that is the period of the
/// proleptic Gregorian calendar. Selected to place unix epoch roughly in the
/// center of the value space, can be arbitrary within type limits.
const ERA_OFFSET: i32 = 3670;
/// Every era has 146097 days
const DAYS_IN_ERA: i32 = 146097;
/// Every era has 400 years
const YEARS_IN_ERA: i32 = 400;
/// Number of days from 0000-03-01 to Unix epoch 1970-01-01
const DAYS_TO_UNIX_EPOCH: i32 = 719468;
/// Offset to be added to given day values
const DAY_OFFSET: i32 = ERA_OFFSET * DAYS_IN_ERA + DAYS_TO_UNIX_EPOCH;
/// Offset to be added to given year values
const YEAR_OFFSET: i32 = ERA_OFFSET * YEARS_IN_ERA;
/// Seconds in a single 24 hour calendar day
const SECS_IN_DAY: i64 = 86400;
/// Offset to be added to given second values
const SECS_OFFSET: i64 = DAY_OFFSET as i64 * SECS_IN_DAY;

/// Minimum supported year for conversion
///
/// Years earlier than this are not supported and will likely produce incorrect
/// results.
pub const YEAR_MIN: i32 = -1467999;

/// Maximum supported year for conversion
///
/// Years later than this are not supported and will likely produce incorrect
/// results.
pub const YEAR_MAX: i32 = 1471744;

/// Minimum Rata Die for conversion
///
/// Rata die days earlier than this are not supported and will likely produce incorrect
/// results.
pub const RD_MIN: i32 = date_to_rd((YEAR_MIN, 1, 1));

/// Maximum Rata Die for conversion
///
/// Rata die days later than this are not supported and will likely produce incorrect
/// results.
pub const RD_MAX: i32 = date_to_rd((YEAR_MAX, 12, 31));

/// Minimum Rata Die in seconds for conversion
///
/// Rata die seconds earlier than this are not supported and will likely produce incorrect
/// results.
pub const RD_SECONDS_MIN: i64 = RD_MIN as i64 * SECS_IN_DAY;

/// Maximum Rata die in seconds for conversion
///
/// Rata die seconds later than this are not supported and will likely produce incorrect
/// results.
pub const RD_SECONDS_MAX: i64 = RD_MAX as i64 * SECS_IN_DAY + SECS_IN_DAY - 1;

/// Convenience constants, mostly for input validation
///
/// The use of these constants is strictly optional, as this is a low level
/// library and the values are wholly unremarkable.
pub mod consts {
    /// Minimum value for week
    pub const WEEK_MIN: u8 = 1;
    /// Maximum value for week
    pub const WEEK_MAX: u8 = 53;
    /// Minimum value for month
    pub const MONTH_MIN: u8 = 1;
    /// Maximum value for month
    pub const MONTH_MAX: u8 = 12;
    /// Minimum value for day of month
    pub const DAY_MIN: u8 = 1;
    /// Maximum value for day of month
    pub const DAY_MAX: u8 = 31;
    /// Minimum value for day of week
    pub const WEEKDAY_MIN: u8 = 1;
    /// Maximum value for day of week
    pub const WEEKDAY_MAX: u8 = 7;
    /// Minimum value for hours
    pub const HOUR_MIN: u8 = 0;
    /// Maximum value for hours
    pub const HOUR_MAX: u8 = 23;
    /// Minimum value for minutes
    pub const MINUTE_MIN: u8 = 0;
    /// Maximum value for minutes
    pub const MINUTE_MAX: u8 = 59;
    /// Minimum value for seconds
    pub const SECOND_MIN: u8 = 0;
    /// Maximum value for seconds
    pub const SECOND_MAX: u8 = 59;
    /// Minimum value for nanoseconds
    pub const NANOSECOND_MIN: u32 = 0;
    /// Maximum value for nanoseconds
    pub const NANOSECOND_MAX: u32 = 999_999_999;

    /// January month value
    pub const JANUARY: u8 = 1;
    /// February month value
    pub const FEBRUARY: u8 = 2;
    /// March month value
    pub const MARCH: u8 = 3;
    /// April month value
    pub const APRIL: u8 = 4;
    /// May month value
    pub const MAY: u8 = 5;
    /// June month value
    pub const JUNE: u8 = 6;
    /// July month value
    pub const JULY: u8 = 7;
    /// August month value
    pub const AUGUST: u8 = 8;
    /// September month value
    pub const SEPTEMBER: u8 = 9;
    /// October month value
    pub const OCTOBER: u8 = 10;
    /// November month value
    pub const NOVEMBER: u8 = 11;
    /// December month value
    pub const DECEMBER: u8 = 12;

    /// Monday day of week value
    pub const MONDAY: u8 = 1;
    /// Tuesday day of week value
    pub const TUESDAY: u8 = 2;
    /// Wednesday day of week value
    pub const WEDNESDAY: u8 = 3;
    /// Thursday day of week value
    pub const THURSDAY: u8 = 4;
    /// Friday day of week value
    pub const FRIDAY: u8 = 5;
    /// Saturday day of week value
    pub const SATURDAY: u8 = 6;
    /// Sunday day of week value
    pub const SUNDAY: u8 = 7;
}

// OPTIMIZATION NOTES:
// - addition and substraction is the same speed regardless of signed or unsigned
// - addition and substraction is the same speed for u32 and u64
// - multiplication and especially division is slower for u64 than u32
// - division is slower for signed than unsigned
// - if the addition of two i32 is positive and fits in u32, wrapping (default)
//   semantics give us the correct results even if the sum is larger than i32::MAX

/// Convert Rata Die to Gregorian date
///
/// Given a day counting from Unix epoch (January 1st, 1970) returns a `(year,
/// month, day)` tuple.
///
/// # Panics
///
/// Argument must be between [RD_MIN] and [RD_MAX] inclusive. Bounds are checked
/// using `debug_assert` only, so that the checks are not present in release
/// builds, similar to integer overflow checks.
///
/// # Examples
///
/// ```
/// use datealgo::rd_to_date;
///
/// assert_eq!(rd_to_date(-719528), (0, 1, 1));
/// assert_eq!(rd_to_date(0), (1970, 1, 1));
/// assert_eq!(rd_to_date(19489), (2023, 5, 12));
/// assert_eq!(rd_to_date(2932896), (9999, 12, 31));
/// assert_eq!(rd_to_date(46761996), (129999, 12, 31));
/// assert_eq!(rd_to_date(-48200687), (-129999, 1, 1));
/// ```
///
/// # Algorithm
///
/// Algorithm currently used is the Neri-Schneider algorithm using Euclidean
/// Affine Functions:
///
/// > Neri C, Schneider L. "*Euclidean affine functions and their application to
/// > calendar algorithms*". Softw Pract Exper. 2022;1-34. doi:
/// > [10.1002/spe.3172](https://onlinelibrary.wiley.com/doi/full/10.1002/spe.3172).
#[inline]
pub const fn rd_to_date(n: i32) -> (i32, u8, u8) {
    debug_assert!(n >= RD_MIN && n <= RD_MAX, "given rata die is out of range");
    let n = (n + DAY_OFFSET) as u32;
    // century
    let n = 4 * n + 3;
    let c = n / 146097;
    let r = n % 146097;
    // year
    let n = r | 3;
    let p = 2939745 * n as u64;
    let z = (p / 2u64.pow(32)) as u32;
    let n = (p % 2u64.pow(32)) as u32 / 2939745 / 4;
    let j = n >= 306;
    let y = 100 * c + z + j as u32;
    // month and day
    let n = 2141 * n + 197913;
    let m = n / 2u32.pow(16);
    let d = n % 2u32.pow(16) / 2141;
    // map
    let y = (y as i32) - YEAR_OFFSET;
    let m = if j { m - 12 } else { m };
    let d = d + 1;
    (y, m as u8, d as u8)
}

/// Convert a Gregorian date to its Computational calendar's counterpart.
#[inline]
const fn date_to_internal(y: i32, m: u8, d: u8) -> (u32, u32, u32, u32) {
    debug_assert!(y >= YEAR_MIN && y <= YEAR_MAX, "given year is out of range");
    debug_assert!(m >= consts::MONTH_MIN && m <= consts::MONTH_MAX, "given month is out of range");
    debug_assert!(d >= consts::DAY_MIN && d <= days_in_month(y, m), "given day is out of range");
    let y = (y + YEAR_OFFSET) as u32;
    let jf = (m < 3) as u32;
    // year
    let y = y - jf;
    // century
    let c = y / 100;
    // month
    let m = m as u32 + 12 * jf;
    // day
    let d = d as u32; // in Neri-Schneider's paper this is d - 1.
    (c, y, m, d)
}

/// Convert Gregorian date to Rata Die
///
/// Given a `(year, month, day)` tuple returns the days since Unix epoch
/// (January 1st, 1970). Dates before the epoch produce negative values.
///
/// # Panics
///
/// Year must be between [YEAR_MIN] and [YEAR_MAX]. Month must be between `1`
/// and `12`. Day must be between `1` and the number of days in the month in
/// question. Bounds are checked using `debug_assert` only, so that the checks
/// are not present in release builds, similar to integer overflow checks.
///
/// # Examples
///
/// ```
/// use datealgo::date_to_rd;
///
/// assert_eq!(date_to_rd((2023, 5, 12)), 19489);
/// assert_eq!(date_to_rd((1970, 1, 1)), 0);
/// assert_eq!(date_to_rd((0, 1, 1)), -719528);
/// assert_eq!(date_to_rd((9999, 12, 31)), 2932896);
/// assert_eq!(date_to_rd((129999, 12, 31)), 46761996);
/// assert_eq!(date_to_rd((-129999, 1, 1)), -48200687);
/// ```
///
/// # Algorithm
///
/// Algorithm currently used is the Neri-Schneider algorithm using Euclidean
/// Affine Functions:
///
/// > Neri C, Schneider L. "*Euclidean affine functions and their application to
/// > calendar algorithms*". Softw Pract Exper. 2022;1-34. doi:
/// > [10.1002/spe.3172](https://onlinelibrary.wiley.com/doi/full/10.1002/spe.3172).
#[inline]
pub const fn date_to_rd((y, m, d): (i32, u8, u8)) -> i32 {
    let (c, y, m, d) = date_to_internal(y, m, d);
    let d = d - 1;
    // year
    let y = 1461 * y / 4 - c + c / 4;
    // month
    let m = (979 * m - 2919) / 32;
    // result
    let n = y + m + d;
    (n as i32) - DAY_OFFSET
}

/// Convert Rata Die to day of week
///
/// Given a day counting from Unix epoch (January 1st, 1970) returns the day of
/// week. Day of week is given as `u32` number between 1 and 7, with `1` meaning
/// Monday and `7` meaning Sunday.
///
/// # Panics
///
/// Argument must be between [RD_MIN] and [RD_MAX] inclusive. Bounds are checked
/// using `debug_assert` only, so that the checks are not present in release
/// builds, similar to integer overflow checks.
///
/// # Examples
///
/// ```
/// use datealgo::{date_to_rd, rd_to_weekday};
///
/// assert_eq!(rd_to_weekday(date_to_rd((2023, 5, 12))), 5);
/// assert_eq!(rd_to_weekday(date_to_rd((1970, 1, 1))), 4);
/// assert_eq!(rd_to_weekday(date_to_rd((2023, 1, 1))), 7);
/// ```
///
/// If you wish to instead have a value from `0` to `6` with `0` signifying a
/// Sunday, it is easiest to just add `% 7`:
///
/// ```
/// use datealgo::{date_to_rd, rd_to_weekday};
///
/// assert_eq!(rd_to_weekday(date_to_rd((2023, 1, 1))) % 7, 0);
/// assert_eq!(rd_to_weekday(date_to_rd((2023, 5, 12))) % 7, 5);
/// ```
///
/// # Algorithm
///
/// Novel algorithm contributed by Cassio Neri:
///
/// > In essence, the algorithm calculates `(n + offset) % 7 + 1` where `offset`
/// > is such that `m := n + offset >= 0` and for `n = 0` it yields `4` (since
/// > January 1st, 1970 was a Thursday). However, it uses a faster way to
/// > evaluate `m % 7 + 1` based on the binary representation of the reciprocal
/// > of `7`, namely, `C := (0.001_001_001...)_2`. The following table presents
/// > the binary values of `m % 7 + 1` and `p := (m + 1) * C` for `m = 0`, `2`,
/// > `...`:
/// >
/// > | `m` | `m % 7 + 1` | `(m + 1) * C`          |
/// > | --- | ----------- | ---------------------- |
/// > | `0` | `(001)_2`   | `(0.001_001_001...)_2` |
/// > | `1` | `(010)_2`   | `(0.010_010_010...)_2` |
/// > | `2` | `(011)_2`   | `(0.011_011_011...)_2` |
/// > | `3` | `(100)_2`   | `(0.100_100_100...)_2` |
/// > | `4` | `(101)_2`   | `(0.101_101_101...)_2` |
/// > | `5` | `(110)_2`   | `(0.110_110_110...)_2` |
/// > | `6` | `(111)_2`   | `(0.111_111_111...)_2` |
/// > | `7` | `(001)_2`   | `(1.001_001_001...)_2` |
/// > | ... | ...         | ...                    |
/// >
/// > Notice that the bits of `(m + 1) * C` after the dot repeat indefinitely in
/// > groups of `3`.  Furthermore, the repeating group matches `m % 7 + 1`.
/// >
/// > Based on the above, the algorithm multiplies `m` by `2^64 / 7` and
/// > extracts the `3` highest bits of the product by shifiting `61` bits to the
/// > right. However, since `2^64 / 7` must be truncated, the result is an
/// > approximation that works provided that `m` is not too large but, still,
/// > large enough for our purposes.
#[inline]
pub const fn rd_to_weekday(n: i32) -> u8 {
    debug_assert!(n >= RD_MIN && n <= RD_MAX, "given rata die is out of range");
    const P64_OVER_SEVEN: u64 = ((1 << 63) / 7) << 1; // = (1 << 64) / 7
    ((((n - RD_MIN) as u64 + 1).wrapping_mul(P64_OVER_SEVEN)) >> 61) as u8
}

/// Convert Gregorian date to day of week
///
/// Given a `(year, month, day)` tuple returns the day of week. Day of week is
/// given as `u32` number between 1 and 7, with `1` meaning Monday and `7`
/// meaning Sunday.
///
/// # Panics
///
/// Year must be between [YEAR_MIN] and [YEAR_MAX]. Month must be between `1`
/// and `12`. Day must be between `1` and the number of days in the month in
/// question. Bounds are checked using `debug_assert` only, so that the checks
/// are not present in release builds, similar to integer overflow checks.
///
/// # Examples
///
/// ```
/// use datealgo::{date_to_weekday};
///
/// assert_eq!(date_to_weekday((2023, 5, 12)), 5);
/// assert_eq!(date_to_weekday((1970, 1, 1)), 4);
/// assert_eq!(date_to_weekday((2023, 1, 1)), 7);
/// ```
///
/// If you wish to instead have a value from `0` to `6` with `0` signifying a
/// Sunday, it is easiest to just add `% 7`:
///
/// ```
/// use datealgo::{date_to_weekday};
///
/// assert_eq!(date_to_weekday((2023, 1, 1)) % 7, 0);
/// assert_eq!(date_to_weekday((2023, 5, 12)) % 7, 5);
/// ```
///
/// # Algorithm
///
/// Simple adaptation of `date_to_rd` to modulus 7 arithmetics.
///
#[inline]
pub const fn date_to_weekday((y, m, d): (i32, u8, u8)) -> u8 {
    let (c, y, m, d) = date_to_internal(y, m, d);
    // year
    let y = 5 * y / 4 - c + c / 4;
    // month
    let m = (979 * m - 2855) / 32;
    // result
    let n = y + m + d;
    const P32_OVER_SEVEN: u32 = ((1 << 31) / 7) << 1; // = (1 << 32) / 7
    ((n.wrapping_mul(P32_OVER_SEVEN)) >> 29) as u8
}

/// Calculate next Gregorian date given a Gregorian date
///
/// Given a `(year, month, day)` tuple returns the `(year, month, day)` tuple
/// for the following Gregorian date.
///
/// # Panics
///
/// Year must be between [YEAR_MIN] and [YEAR_MAX]. Month must be between `1`
/// and `12`. Day must be between `1` and the number of days in the month in
/// question and the next date must not be after [YEAR_MAX]. Bounds are checked
/// using `debug_assert` only, so that the checks are not present in release
/// builds, similar to integer overflow checks.
///
/// # Examples
///
/// ```
/// use datealgo::{next_date};
///
/// assert_eq!(next_date((2023, 5, 12)), (2023, 5, 13));
/// assert_eq!(next_date((1970, 1, 1)), (1970, 1, 2));
/// assert_eq!(next_date((2023, 1, 31)), (2023, 2, 1));
/// assert_eq!(next_date((2023, 12, 31)), (2024, 1, 1));
/// ```
///
/// # Algorithm
///
/// Simple incrementation with manual overflow checking and carry. Relatively
/// speedy, but not fully optimized.
#[inline]
pub const fn next_date((y, m, d): (i32, u8, u8)) -> (i32, u8, u8) {
    debug_assert!(y >= YEAR_MIN && y <= YEAR_MAX, "given year is out of range");
    debug_assert!(m >= consts::MONTH_MIN && m <= consts::MONTH_MAX, "given month is out of range");
    debug_assert!(d >= consts::DAY_MIN && d <= days_in_month(y, m), "given day is out of range");
    debug_assert!(
        y != YEAR_MAX || m != consts::MONTH_MAX || d != consts::DAY_MAX,
        "next date is out of range"
    );
    if d < 28 || d < days_in_month(y, m) {
        (y, m, d + 1)
    } else if m < 12 {
        (y, m + 1, 1)
    } else {
        (y + 1, 1, 1)
    }
}

/// Calculate previous Gregorian date given a Gregorian date
///
/// Given a `(year, month, day)` tuple returns the `(year, month, day)` tuple
/// for the preceding Gregorian date.
///
/// # Panics
///
/// Year must be between [YEAR_MIN] and [YEAR_MAX]. Month must be between `1`
/// and `12`. Day must be between `1`, the number of days in the month in
/// question and the previous date must not be before [YEAR_MIN]. Bounds are
/// checked using `debug_assert` only, so that the checks are not present in
/// release builds, similar to integer overflow checks.
///
/// # Examples
///
/// ```
/// use datealgo::{prev_date};
///
/// assert_eq!(prev_date((2023, 5, 12)), (2023, 5, 11));
/// assert_eq!(prev_date((1970, 1, 1)), (1969, 12, 31));
/// assert_eq!(prev_date((2023, 2, 1)), (2023, 1, 31));
/// assert_eq!(prev_date((2024, 1, 1)), (2023, 12, 31));
/// ```
///
/// # Algorithm
///
/// Simple decrementation with manual underflow checking and carry. Relatively
/// speedy, but not fully optimized.
#[inline]
pub const fn prev_date((y, m, d): (i32, u8, u8)) -> (i32, u8, u8) {
    debug_assert!(y >= YEAR_MIN && y <= YEAR_MAX, "given year is out of range");
    debug_assert!(m >= consts::MONTH_MIN && m <= consts::MONTH_MAX, "given month is out of range");
    debug_assert!(d >= consts::DAY_MIN && d <= days_in_month(y, m), "given day is out of range");
    debug_assert!(
        y != YEAR_MIN || m != consts::MONTH_MIN || d != consts::DAY_MIN,
        "previous date is out of range"
    );
    if d > 1 {
        (y, m, d - 1)
    } else if m > 1 {
        (y, m - 1, days_in_month(y, m - 1))
    } else {
        (y - 1, 12, 31)
    }
}

/// Split total seconds to days, hours, minutes and seconds
///
/// Given seconds counting from Unix epoch (January 1st, 1970) returns a `(days,
/// hours, minutes, seconds)` tuple.
///
/// # Panics
///
/// Argument must be between [RD_SECONDS_MIN] and [RD_SECONDS_MAX] inclusive.
/// Bounds are checked using `debug_assert` only, so that the checks are not
/// present in release builds, similar to integer overflow checks.
///
/// # Examples
///
/// ```
/// use datealgo::{secs_to_dhms, date_to_rd};
///
/// assert_eq!(secs_to_dhms(0), (0, 0, 0, 0));
/// assert_eq!(secs_to_dhms(86400), (1, 0, 0, 0));
/// assert_eq!(secs_to_dhms(86399), (0, 23, 59, 59));
/// assert_eq!(secs_to_dhms(-1), (-1, 23, 59, 59));
/// assert_eq!(secs_to_dhms(1684574678), (date_to_rd((2023, 5, 20)), 9, 24, 38));
/// ```
///
/// # Algorithm
///
/// See examples 14 and 15 of:
///
/// > Neri C, Schneider L. "*Euclidean affine functions and their application to
/// > calendar algorithms*". Softw Pract Exper. 2022;1-34. doi:
/// > [10.1002/spe.3172](https://onlinelibrary.wiley.com/doi/full/10.1002/spe.3172).
#[inline]
pub const fn secs_to_dhms(secs: i64) -> (i32, u8, u8, u8) {
    debug_assert!(
        secs >= RD_SECONDS_MIN && secs <= RD_SECONDS_MAX,
        "given seconds value is out of range"
    );
    // Algorithm is based on the following identities valid for all n in [0, 97612919[.
    //
    // n / 60 = 71582789 * n / 2^32,
    // n % 60 = 71582789 * n % 2^32 / 71582789.
    //
    // `SECS_IN_DAY` obviously fits within these bounds
    let secs = if secs > RD_SECONDS_MAX { 0 } else { secs }; // allows compiler to optimize more
    let secs = (secs + SECS_OFFSET) as u64;
    let days = (secs / SECS_IN_DAY as u64) as u32;
    let secs = secs % SECS_IN_DAY as u64; // secs in [0, SECS_IN_DAY[ => secs in [0, 97612919[

    let prd = 71582789 * secs;
    let mins = prd >> 32; // secs / 60
    let ss = (prd as u32) / 71582789; // secs % 60

    let prd = 71582789 * mins;
    let hh = prd >> 32; // mins / 60
    let mm = (prd as u32) / 71582789; // mins % 60

    let days = (days as i32) - DAY_OFFSET;
    (days, hh as u8, mm as u8, ss as u8)
}

/// Combine days, hours, minutes and seconds to total seconds
///
/// Given a `(days, hours, minutes, seconds)` tuple from Unix epoch (January
/// 1st, 1970) returns the total seconds.
///
/// # Panics
///
/// Days must be between [RD_MIN] and [RD_MAX] inclusive. Hours must be between
/// `0` and `23`. Minutes must be between `0` and `59`. Seconds must be between
/// `0` and `59`. Bounds are checked using `debug_assert` only, so that the
/// checks are not present in release builds, similar to integer overflow
/// checks.
///
/// # Examples
///
/// ```
/// use datealgo::{dhms_to_secs, date_to_rd};
///
/// assert_eq!(dhms_to_secs((0, 0, 0, 0)), 0);
/// assert_eq!(dhms_to_secs((1, 0, 0, 0)), 86400);
/// assert_eq!(dhms_to_secs((0, 23, 59, 59)), 86399);
/// assert_eq!(dhms_to_secs((-1, 0, 0, 0)), -86400);
/// assert_eq!(dhms_to_secs((-1, 0, 0, 1)), -86399);
/// assert_eq!(dhms_to_secs((date_to_rd((2023, 5, 20)), 9, 24, 38)), 1684574678)
/// ```
///
/// # Algorithm
///
/// Algorithm is simple multiplication, method provided only as convenience.
#[inline]
pub const fn dhms_to_secs((d, h, m, s): (i32, u8, u8, u8)) -> i64 {
    debug_assert!(d >= RD_MIN && d <= RD_MAX, "given rata die is out of range");
    debug_assert!(h >= consts::HOUR_MIN && h <= consts::HOUR_MAX, "given hour is out of range");
    debug_assert!(m >= consts::MINUTE_MIN && m <= consts::MINUTE_MAX, "given minute is out of range");
    debug_assert!(s >= consts::SECOND_MIN && s <= consts::SECOND_MAX, "given second is out of range");
    if d >= RD_MIN && d <= RD_MAX {
        d as i64 * SECS_IN_DAY + h as i64 * 3600 + m as i64 * 60 + s as i64
    } else {
        0
    }
}

/// Convert total seconds to year, month, day, hours, minutes and seconds
///
/// Given seconds counting from Unix epoch (January 1st, 1970) returns a `(year,
/// month, day, hours, minutes, seconds)` tuple.
///
/// # Panics
///
/// Argument must be between [RD_SECONDS_MIN] and [RD_SECONDS_MAX] inclusive.
/// Bounds are checked using `debug_assert` only, so that the checks are not
/// present in release builds, similar to integer overflow checks.
///
/// # Examples
///
/// ```
/// use datealgo::secs_to_datetime;
///
/// assert_eq!(secs_to_datetime(0), (1970, 1, 1, 0, 0, 0));
/// assert_eq!(secs_to_datetime(86400), (1970, 1, 2, 0, 0, 0));
/// assert_eq!(secs_to_datetime(86399), (1970, 1, 1, 23, 59, 59));
/// assert_eq!(secs_to_datetime(-1), (1969, 12, 31, 23, 59, 59));
/// assert_eq!(secs_to_datetime(1684574678), (2023, 5, 20, 9, 24, 38));
/// ```
///
/// # Algorithm
///
/// Combination of existing functions for convenience only.
#[inline]
pub const fn secs_to_datetime(secs: i64) -> (i32, u8, u8, u8, u8, u8) {
    let (days, hh, mm, ss) = secs_to_dhms(secs);
    let (y, m, s) = rd_to_date(days);
    (y, m, s, hh, mm, ss)
}

/// Convert year, month, day, hours, minutes and seconds to total seconds
///
/// Given a `(year, month, day, hours, minutes, seconds)` tuple from Unix epoch
/// (January 1st, 1970) returns the total seconds.
///
/// # Panics
///
/// Year must be between [YEAR_MIN] and [YEAR_MAX]. Month must be between `1`
/// and `12`. Day must be between `1` and the number of days in the month in
/// question. Hours must be between `0` and `23`. Minutes must be between `0`
/// and `59`. Seconds must be between `0` and `59`. Bounds are checked using
/// `debug_assert` only, so that the checks are not present in release builds,
/// similar to integer overflow checks.
///
/// # Examples
///
/// ```
/// use datealgo::datetime_to_secs;
///
/// assert_eq!(datetime_to_secs((1970, 1, 1, 0, 0, 0)), 0);
/// assert_eq!(datetime_to_secs((1970, 1, 2, 0, 0, 0)), 86400);
/// assert_eq!(datetime_to_secs((1970, 1, 1, 23, 59, 59)), 86399);
/// assert_eq!(datetime_to_secs((1969, 12, 31, 0, 0, 0)), -86400);
/// assert_eq!(datetime_to_secs((1969, 12, 31, 0, 0, 1)), -86399);
/// assert_eq!(datetime_to_secs((2023, 5, 20, 9, 24, 38)), 1684574678)
/// ```
///
/// # Algorithm
///
/// Algorithm is simple multiplication, method provided only as convenience.
#[inline]
pub const fn datetime_to_secs((y, m, d, hh, mm, ss): (i32, u8, u8, u8, u8, u8)) -> i64 {
    let days = date_to_rd((y, m, d));
    dhms_to_secs((days, hh, mm, ss))
}

/// Determine if the given year is a leap year
///
/// # Panics
///
/// Year must be between [YEAR_MIN] and [YEAR_MAX] inclusive. Bounds are checked
/// using `debug_assert` only, so that the checks are not present in release
/// builds, similar to integer overflow checks.
///
/// # Examples
///
/// ```
/// use datealgo::is_leap_year;
///
/// assert_eq!(is_leap_year(2023), false);
/// assert_eq!(is_leap_year(2024), true);
/// assert_eq!(is_leap_year(2100), false);
/// assert_eq!(is_leap_year(2400), true);
/// ```
///
/// # Algorithm
///
/// Algorithm is Neri-Schneider from C++now 2023 conference:
/// > <https://github.com/boostcon/cppnow_presentations_2023/blob/main/cppnow_slides/Speeding_Date_Implementing_Fast_Calendar_Algorithms.pdf>
#[inline]
pub const fn is_leap_year(y: i32) -> bool {
    debug_assert!(y >= YEAR_MIN && y <= YEAR_MAX, "given year is out of range");
    // Using `%` instead of `&` causes compiler to emit branches instead. This
    // is faster in a tight loop due to good branch prediction, but probably
    // slower in a real program so we use `&`. Also `% 25` is functionally
    // equivalent to `% 100` here, but a little cheaper to compute. If branches
    // were to be emitted, using `% 100` would be most likely faster due to
    // better branch prediction.
    if (y % 25) != 0 {
        y & 3 == 0
    } else {
        y & 15 == 0
    }
}

/// Determine the number of days in the given month in the given year
///
/// # Panics
///
/// Year must be between [YEAR_MIN] and [YEAR_MAX]. Month must be between `1`
/// and `12`. Bounds are checked using `debug_assert` only, so that the checks
/// are not present in release builds, similar to integer overflow checks.
///
/// # Example
///
/// ```
/// use datealgo::days_in_month;
///
/// assert_eq!(days_in_month(2023, 1), 31);
/// assert_eq!(days_in_month(2023, 2), 28);
/// assert_eq!(days_in_month(2023, 4), 30);
/// assert_eq!(days_in_month(2024, 1), 31);
/// assert_eq!(days_in_month(2024, 2), 29);
/// assert_eq!(days_in_month(2024, 4), 30);
/// ```
///
/// # Algorithm
///
/// Algorithm is Neri-Schneider from C++now 2023 conference:
/// > <https://github.com/boostcon/cppnow_presentations_2023/blob/main/cppnow_slides/Speeding_Date_Implementing_Fast_Calendar_Algorithms.pdf>
#[inline]
pub const fn days_in_month(y: i32, m: u8) -> u8 {
    debug_assert!(m >= consts::MONTH_MIN && m <= consts::MONTH_MAX, "given month is out of range");
    if m != 2 {
        30 | (m ^ (m >> 3))
    } else if is_leap_year(y) {
        29
    } else {
        28
    }
}

/// Convert Rata Die to [ISO week date](https://en.wikipedia.org/wiki/ISO_week_date)
///
/// Given a day counting from Unix epoch (January 1st, 1970) returns a `(year,
/// week, day of week)` tuple. Week is the ISO week number, with the first week
/// of the year being the week containing the first Thursday of the year. Day of
/// week is between 1 and 7, with `1` meaning Monday and `7` meaning Sunday.
///
/// Compared to Gregorian date, the first one to three days of the year might
/// belong to a week in the previous year, and the last one to three days of the
/// year might belong to a week in the next year. Also some years have 53 weeks
/// instead of 52.
///
/// # Panics
///
/// Argument must be between [RD_MIN] and [RD_MAX] inclusive. Bounds are checked
/// using `debug_assert` only, so that the checks are not present in release
/// builds, similar to integer overflow checks.
///
/// # Examples
///
/// ```
/// use datealgo::{rd_to_isoweekdate, date_to_rd};
///
/// assert_eq!(rd_to_isoweekdate(date_to_rd((2023, 5, 12))), (2023, 19, 5));
/// assert_eq!(rd_to_isoweekdate(date_to_rd((1970, 1, 1))), (1970, 1, 4));
/// assert_eq!(rd_to_isoweekdate(date_to_rd((2023, 1, 1))), (2022, 52, 7));
/// assert_eq!(rd_to_isoweekdate(date_to_rd((1979, 12, 31))), (1980, 1, 1));
/// assert_eq!(rd_to_isoweekdate(date_to_rd((1981, 12, 31))), (1981, 53, 4));
/// assert_eq!(rd_to_isoweekdate(date_to_rd((1982, 1, 1))), (1981, 53, 5));
/// ```
///
/// # Algorithm
///
/// Algorithm is hand crafted and not significantly optimized.
#[inline]
pub const fn rd_to_isoweekdate(rd: i32) -> (i32, u8, u8) {
    debug_assert!(rd >= RD_MIN && rd <= RD_MAX, "given rata die is out of range");
    let wd = rd_to_weekday(rd);
    let rdt = rd + (4 - wd as i32) % 7;
    let (y, _, _) = rd_to_date(rdt);
    let ys = date_to_rd((y, 1, 1));
    let w = (rdt - ys) / 7 + 1;
    (y, w as u8, wd)
}

/// Convert [ISO week date](https://en.wikipedia.org/wiki/ISO_week_date) to Rata Die
///
/// Given a `(year, week, day of week)` tuple returns the days since Unix epoch
/// (January 1st, 1970). Week is the ISO week number, with the first week of the
/// year being the week containing the first Thursday of the year. Day of week
/// is between 1 and 7, with `1` meaning Monday and `7` meaning Sunday. Dates
/// before the epoch produce negative values.
///
/// Compared to Gregorian date, the first one to three days of the year might
/// belong to a week in the previous year, and the last one to three days of the
/// year might belong to a week in the next year. Also some years have 53 weeks
/// instead of 52.
///
/// # Panics
///
/// Year must be between [YEAR_MIN] and [YEAR_MAX]. Week must be between `1` and
/// the number of ISO weeks in the given year (52 or 53). Day must be between
/// `1` and `7`. Bounds are checked using `debug_assert` only, so that the
/// checks are not present in release builds, similar to integer overflow
/// checks.
///
/// # Examples
///
/// ```
/// use datealgo::{isoweekdate_to_rd, date_to_rd};
///
/// assert_eq!(isoweekdate_to_rd((2023, 19, 5)), date_to_rd((2023, 5, 12)));
/// assert_eq!(isoweekdate_to_rd((1970, 1, 4)), date_to_rd((1970, 1, 1)));
/// assert_eq!(isoweekdate_to_rd((2022, 52, 7)), date_to_rd((2023, 1, 1)));
/// assert_eq!(isoweekdate_to_rd((1980, 1, 1)), date_to_rd((1979, 12, 31)));
/// assert_eq!(isoweekdate_to_rd((1981, 53, 4)), date_to_rd((1981, 12, 31)));
/// assert_eq!(isoweekdate_to_rd((1981, 53, 5)), date_to_rd((1982, 1, 1)));
/// ```
///
/// # Algorithm
///
/// Algorithm is hand crafted and not significantly optimized.
#[inline]
pub const fn isoweekdate_to_rd((y, w, d): (i32, u8, u8)) -> i32 {
    debug_assert!(y >= YEAR_MIN && y <= YEAR_MAX, "given year is out of range");
    debug_assert!(w >= consts::WEEK_MIN && w <= isoweeks_in_year(y), "given week is out of range");
    debug_assert!(
        d >= consts::WEEKDAY_MIN && d <= consts::WEEKDAY_MAX,
        "given weekday is out of range"
    );
    debug_assert!(
        y != YEAR_MAX || w != consts::WEEK_MAX || d <= consts::THURSDAY,
        "given weekday is out of range (for last week of range)"
    );
    let rd4 = date_to_rd((y, 1, 4));
    let wd4 = rd_to_weekday(rd4);
    let ys = rd4 - (wd4 - 1) as i32;
    ys + (w as i32 - 1) * 7 + (d as i32 - 1)
}

/// Convert Gregorian date to [ISO week date](https://en.wikipedia.org/wiki/ISO_week_date)
///
/// Given a `(year, month, day)` tuple returns a `(year, week, day of week)`
/// tuple. Week is the ISO week number, with the first week of the year being
/// the week containing the first Thursday of the year. Day of week is between
/// 1 and 7, with `1` meaning Monday and `7` meaning Sunday.
///
/// Compared to Gregorian date, the first one to three days of the year might
/// belong to a week in the previous year, and the last one to three days of the
/// year might belong to a week in the next year. Also some years have 53 weeks
/// instead of 52.
///
/// # Panics
///
/// Year must be between [YEAR_MIN] and [YEAR_MAX]. Month must be between `1`
/// and `12`. Day must be between `1` and the number of days in the month in
/// question. Bounds are checked using `debug_assert` only, so that the checks
/// are not present in release builds, similar to integer overflow checks.
///
/// # Examples
///
/// ```
/// use datealgo::{date_to_isoweekdate};
///
/// assert_eq!(date_to_isoweekdate((2023, 5, 12)), (2023, 19, 5));
/// assert_eq!(date_to_isoweekdate((1970, 1, 1)), (1970, 1, 4));
/// assert_eq!(date_to_isoweekdate((2023, 1, 1)), (2022, 52, 7));
/// assert_eq!(date_to_isoweekdate((1979, 12, 31)), (1980, 1, 1));
/// assert_eq!(date_to_isoweekdate((1981, 12, 31)), (1981, 53, 4));
/// assert_eq!(date_to_isoweekdate((1982, 1, 1)), (1981, 53, 5));
/// ```
///
/// # Algorithm
///
/// Simply converts date to rata die and then rata die to ISO week date.
#[inline]
pub const fn date_to_isoweekdate((y, m, d): (i32, u8, u8)) -> (i32, u8, u8) {
    let rd = date_to_rd((y, m, d));
    rd_to_isoweekdate(rd)
}

/// Convert [ISO week date](https://en.wikipedia.org/wiki/ISO_week_date) to Gregorian date
///
/// Given a `(year, week, day of week)` tuple returns a `(year, month, day)`
/// tuple. Week is the ISO week number, with the first week of the year being
/// the week containing the first Thursday of the year. Day of week is between
/// 1 and 7, with `1` meaning Monday and `7` meaning Sunday.
///
/// Compared to Gregorian date, the first one to three days of the year might
/// belong to a week in the previous year, and the last one to three days of the
/// year might belong to a week in the next year. Also some years have 53 weeks
/// instead of 52.
///
/// # Panics
///
/// Year must be between [YEAR_MIN] and [YEAR_MAX]. Week must be between `1` and
/// the number of ISO weeks in the given year (52 or 53). Day must be between
/// `1` and `7`. Bounds are checked using `debug_assert` only, so that the
/// checks are not present in release builds, similar to integer overflow
/// checks.
///
/// # Examples
///
/// ```
/// use datealgo::{isoweekdate_to_date};
///
/// assert_eq!(isoweekdate_to_date((2023, 19, 5)), (2023, 5, 12));
/// assert_eq!(isoweekdate_to_date((1970, 1, 4)), (1970, 1, 1));
/// assert_eq!(isoweekdate_to_date((2022, 52, 7)), (2023, 1, 1));
/// assert_eq!(isoweekdate_to_date((1980, 1, 1)), (1979, 12, 31));
/// assert_eq!(isoweekdate_to_date((1981, 53, 4)), (1981, 12, 31));
/// assert_eq!(isoweekdate_to_date((1981, 53, 5)), (1982, 1, 1));
/// ```
///
/// # Algorithm
///
/// Simply converts ISO week date to rata die and then rata die to date.
#[inline]
pub const fn isoweekdate_to_date((y, w, d): (i32, u8, u8)) -> (i32, u8, u8) {
    let rd = isoweekdate_to_rd((y, w, d));
    rd_to_date(rd)
}

/// Determine the number of [ISO weeks](https://en.wikipedia.org/wiki/ISO_week_date) in the given year
///
/// According to the ISO standard a year has 52 weeks, unless the first week of
/// the year starts on a Thursday or the year is a leap year and the first week
/// of the year starts on a Wednesday, in which case the year has 53 weeks.
///
/// # Panics
///
/// Year must be between [YEAR_MIN] and [YEAR_MAX]. Bounds are checked using
/// `debug_assert` only, so that the checks are not present in release builds,
/// similar to integer overflow checks.
///
/// # Examples
///
/// ```
/// use datealgo::isoweeks_in_year;
///
/// assert_eq!(isoweeks_in_year(2023), 52);
/// assert_eq!(isoweeks_in_year(2024), 52);
/// assert_eq!(isoweeks_in_year(2025), 52);
/// assert_eq!(isoweeks_in_year(2026), 53);
/// assert_eq!(isoweeks_in_year(2027), 52);
/// ```
///
/// # Algorithm
///
/// Algorithm is hand crafted and not significantly optimized.
#[inline]
pub const fn isoweeks_in_year(y: i32) -> u8 {
    debug_assert!(y >= YEAR_MIN && y <= YEAR_MAX, "given year is out of range");
    let wd = date_to_weekday((y, 1, 1));
    let l = is_leap_year(y);
    match wd {
        consts::THURSDAY => 53,
        consts::WEDNESDAY if l => 53,
        _ => 52,
    }
}

/// Convert [`std::time::SystemTime`] to seconds and nanoseconds
///
/// Given [`std::time::SystemTime`] returns an `Option` of `(seconds,
/// nanoseconds)` tuple from Unix epoch (January 1st, 1970).
///
/// # Errors
///
/// Returns `None` if the time is before [RD_SECONDS_MIN] or after
/// [RD_SECONDS_MAX].
///
/// # Examples
///
/// ```
/// use datealgo::systemtime_to_secs;
/// use std::time::{Duration, UNIX_EPOCH};
///
/// assert_eq!(systemtime_to_secs(UNIX_EPOCH), Some((0, 0)));
/// assert_eq!(systemtime_to_secs(UNIX_EPOCH + Duration::new(1, 0)), Some((1, 0)));
/// assert_eq!(systemtime_to_secs(UNIX_EPOCH + Duration::new(0, 1)), Some((0, 1)));
/// assert_eq!(systemtime_to_secs(UNIX_EPOCH - Duration::new(1, 0)), Some((-1, 0)));
/// assert_eq!(systemtime_to_secs(UNIX_EPOCH - Duration::new(0, 1)), Some((-1, 999_999_999)));
/// ```
///
/// # Algorithm
///
/// Uses `.duration_since(UNIX_EPOCH)` and handles both positive and negative
/// result.
#[cfg(feature = "std")]
#[inline]
pub fn systemtime_to_secs(st: SystemTime) -> Option<(i64, u32)> {
    match st.duration_since(UNIX_EPOCH) {
        Ok(dur) => {
            let secs = dur.as_secs();
            let nsecs = dur.subsec_nanos();
            if secs > RD_SECONDS_MAX as u64 {
                return None;
            }
            Some((secs as i64, nsecs))
        }
        Err(err) => {
            let dur = err.duration();
            let mut secs = dur.as_secs();
            let mut nsecs = dur.subsec_nanos();
            if nsecs > 0 {
                secs += 1;
                nsecs = 1_000_000_000 - nsecs;
            }
            if secs > -RD_SECONDS_MIN as u64 {
                return None;
            }
            Some((-(secs as i64), nsecs))
        }
    }
}

/// Convert seconds and nanoseconds to [`std::time::SystemTime`]
///
/// Given a tuple of seconds and nanoseconds counting from Unix epoch (January
/// 1st, 1970) returns Option of [`std::time::SystemTime`].
///
/// # Errors
///
/// Returns `None` if given datetime cannot be represented as `SystemTime`.
///
/// # Panics
///
/// Seconds must be between [RD_SECONDS_MIN] and [RD_SECONDS_MAX] inclusive.
/// Nanoseconds must between `0` and `999_999_999`. Bounds are checked using
/// `debug_assert` only, so that the checks are not present in release builds,
/// similar to integer overflow checks.
///
/// # Examples
///
/// ```
/// use datealgo::secs_to_systemtime;
/// use std::time::{Duration, UNIX_EPOCH};
///
/// assert_eq!(secs_to_systemtime((0, 0)), Some(UNIX_EPOCH));
/// assert_eq!(secs_to_systemtime((0, 1)), UNIX_EPOCH.checked_add(Duration::new(0, 1)));
/// assert_eq!(secs_to_systemtime((1, 0)), UNIX_EPOCH.checked_add(Duration::new(1, 0)));
/// assert_eq!(secs_to_systemtime((-1, 999_999_999)), UNIX_EPOCH.checked_sub(Duration::new(0, 1)));
/// assert_eq!(secs_to_systemtime((-1, 0)), UNIX_EPOCH.checked_sub(Duration::new(1, 0)));
/// assert_eq!(secs_to_systemtime((-2, 999_999_999)), UNIX_EPOCH.checked_sub(Duration::new(1, 1)));
/// ```
///
/// # Algorithm
///
/// Combination of existing functions for convenience only.
#[cfg(feature = "std")]
#[inline]
pub fn secs_to_systemtime((secs, nsecs): (i64, u32)) -> Option<SystemTime> {
    debug_assert!(secs >= RD_SECONDS_MIN && secs <= RD_SECONDS_MAX, "given seconds is out of range");
    debug_assert!(
        nsecs >= consts::NANOSECOND_MIN && nsecs <= consts::NANOSECOND_MAX,
        "given nanoseconds is out of range"
    );
    if secs >= 0 {
        UNIX_EPOCH.checked_add(Duration::new(secs as u64, nsecs))
    } else if nsecs > 0 {
        UNIX_EPOCH.checked_sub(Duration::new((-secs - 1) as u64, 1_000_000_000 - nsecs))
    } else {
        UNIX_EPOCH.checked_sub(Duration::from_secs(-secs as u64))
    }
}

/// Convert [`std::time::SystemTime`] to year, month, day, hours, minutes,
/// seconds and nanoseconds
///
/// Given [`std::time::SystemTime`] returns an Option of `(year, month, day,
/// hours, minutes, seconds, nanoseconds)` tuple.
///
/// # Errors
///
/// Returns `None` if the time is before [RD_SECONDS_MIN] or after
/// [RD_SECONDS_MAX].
///
/// # Examples
///
/// ```
/// use datealgo::systemtime_to_datetime;
/// use std::time::{Duration, UNIX_EPOCH};
///
/// assert_eq!(systemtime_to_datetime(UNIX_EPOCH), Some((1970, 1, 1, 0, 0, 0, 0)));
/// assert_eq!(systemtime_to_datetime(UNIX_EPOCH + Duration::from_secs(1684574678)), Some((2023, 5, 20, 9, 24, 38, 0)));
/// assert_eq!(systemtime_to_datetime(UNIX_EPOCH - Duration::from_secs(1)), Some((1969, 12, 31, 23, 59, 59, 0)));
/// assert_eq!(systemtime_to_datetime(UNIX_EPOCH - Duration::new(0, 1)), Some((1969, 12, 31, 23, 59, 59, 999_999_999)));
/// ```
///
/// # Algorithm
///
/// Combination of existing functions for convenience only.
#[cfg(feature = "std")]
#[inline]
pub fn systemtime_to_datetime(st: SystemTime) -> Option<(i32, u8, u8, u8, u8, u8, u32)> {
    let (secs, nsecs) = systemtime_to_secs(st)?;
    let (days, hh, mm, ss) = secs_to_dhms(secs);
    let (year, month, day) = rd_to_date(days);
    Some((year, month, day, hh, mm, ss, nsecs))
}

/// Convert year, month, day, hours, minutes, seconds and nanoseconds to
/// [`std::time::SystemTime`]
///
/// Given a `(year, month, day, hours, minutes, seconds, nanoseconds)` tuple
/// from Unix epoch (January 1st, 1970) returns Option of
/// [`std::time::SystemTime`].
///
/// # Errors
///
/// Returns `None` if given datetime cannot be represented as `SystemTime`.
///
/// # Panics
///
/// Year must be between [YEAR_MIN] and [YEAR_MAX]. Month must be between `1`
/// and `12`. Day must be between `1` and the number of days in the month in
/// question. Hours must be between `0` and `23`. Minutes must be between `0`
/// and `59`. Seconds must be between `0` and `59`. Nanoseconds must be between
/// `0` and `999_999_999`. Bounds are checked using `debug_assert` only, so that
/// the checks are not present in release builds, similar to integer overflow
/// checks.
///
/// # Examples
///
/// ```
/// use datealgo::datetime_to_systemtime;
/// use std::time::{Duration, UNIX_EPOCH};
///
/// assert_eq!(datetime_to_systemtime((1970, 1, 1, 0, 0, 0, 0)), Some(UNIX_EPOCH));
/// assert_eq!(datetime_to_systemtime((1970, 1, 1, 0, 0, 1, 0)), UNIX_EPOCH.checked_add(Duration::new(1, 0)));
/// assert_eq!(datetime_to_systemtime((2023, 5, 20, 9, 24, 38, 0)), UNIX_EPOCH.checked_add(Duration::from_secs(1684574678)));
/// ```
///
/// # Algorithm
///
/// Combination of existing functions for convenience only.
#[cfg(feature = "std")]
#[inline]
pub fn datetime_to_systemtime((y, m, d, hh, mm, ss, nsec): (i32, u8, u8, u8, u8, u8, u32)) -> Option<SystemTime> {
    let days = date_to_rd((y, m, d));
    let secs = dhms_to_secs((days, hh, mm, ss));
    secs_to_systemtime((secs, nsec))
}

#[cfg(feature = "asmdump")]
pub mod asm {
    //! Non-inline wrappers for functions for dumping assembly with
    //! cargo-show-asm
    #[cfg(feature = "std")]
    use std::time::SystemTime;

    #[inline(never)]
    pub const fn rd_to_date(n: i32) -> (i32, u8, u8) {
        super::rd_to_date(n)
    }
    #[inline(never)]
    pub const fn date_to_rd((y, m, d): (i32, u8, u8)) -> i32 {
        super::date_to_rd((y, m, d))
    }
    #[inline(never)]
    pub const fn rd_to_weekday(n: i32) -> u8 {
        super::rd_to_weekday(n)
    }
    #[inline(never)]
    pub const fn date_to_weekday((y, m, d): (i32, u8, u8)) -> u8 {
        super::date_to_weekday((y, m, d))
    }
    #[inline(never)]
    pub const fn next_date((y, m, d): (i32, u8, u8)) -> (i32, u8, u8) {
        super::next_date((y, m, d))
    }
    #[inline(never)]
    pub const fn prev_date((y, m, d): (i32, u8, u8)) -> (i32, u8, u8) {
        super::prev_date((y, m, d))
    }
    #[inline(never)]
    pub const fn secs_to_dhms(secs: i64) -> (i32, u8, u8, u8) {
        super::secs_to_dhms(secs)
    }
    #[inline(never)]
    pub const fn dhms_to_secs((d, h, m, s): (i32, u8, u8, u8)) -> i64 {
        super::dhms_to_secs((d, h, m, s))
    }
    #[inline(never)]
    pub const fn secs_to_datetime(secs: i64) -> (i32, u8, u8, u8, u8, u8) {
        super::secs_to_datetime(secs)
    }
    #[inline(never)]
    pub const fn datetime_to_secs((y, m, d, hh, mm, ss): (i32, u8, u8, u8, u8, u8)) -> i64 {
        super::datetime_to_secs((y, m, d, hh, mm, ss))
    }
    #[inline(never)]
    pub const fn is_leap_year(y: i32) -> bool {
        super::is_leap_year(y)
    }
    #[inline(never)]
    pub const fn days_in_month(y: i32, m: u8) -> u8 {
        super::days_in_month(y, m)
    }
    #[inline(never)]
    pub const fn rd_to_isoweekdate(rd: i32) -> (i32, u8, u8) {
        super::rd_to_isoweekdate(rd)
    }
    #[inline(never)]
    pub const fn isoweekdate_to_rd((y, w, d): (i32, u8, u8)) -> i32 {
        super::isoweekdate_to_rd((y, w, d))
    }
    #[inline(never)]
    pub const fn date_to_isoweekdate((y, m, d): (i32, u8, u8)) -> (i32, u8, u8) {
        super::date_to_isoweekdate((y, m, d))
    }
    #[inline(never)]
    pub const fn isoweekdate_to_date((y, w, d): (i32, u8, u8)) -> (i32, u8, u8) {
        super::isoweekdate_to_date((y, w, d))
    }
    #[inline(never)]
    pub const fn isoweeks_in_year(y: i32) -> u8 {
        super::isoweeks_in_year(y)
    }
    #[cfg(feature = "std")]
    #[inline(never)]
    pub fn systemtime_to_secs(st: SystemTime) -> Option<(i64, u32)> {
        super::systemtime_to_secs(st)
    }
    #[cfg(feature = "std")]
    #[inline(never)]
    pub fn secs_to_systemtime((secs, nsecs): (i64, u32)) -> Option<SystemTime> {
        super::secs_to_systemtime((secs, nsecs))
    }
    #[cfg(feature = "std")]
    #[inline(never)]
    pub fn systemtime_to_datetime(st: SystemTime) -> Option<(i32, u8, u8, u8, u8, u8, u32)> {
        super::systemtime_to_datetime(st)
    }
    #[cfg(feature = "std")]
    #[inline(never)]
    pub fn datetime_to_systemtime((y, m, d, hh, mm, ss, nsec): (i32, u8, u8, u8, u8, u8, u32)) -> Option<SystemTime> {
        super::datetime_to_systemtime((y, m, d, hh, mm, ss, nsec))
    }
}
