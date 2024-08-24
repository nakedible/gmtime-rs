//! Non-inline wrappers for functions for dumping assembly with
//! cargo-show-asm
#[cfg(feature = "std")]
use std::time::SystemTime;

#[no_mangle]
pub const fn asm_rd_to_date(n: i32) -> (i32, u8, u8) {
    datealgo::rd_to_date(n)
}
#[no_mangle]
pub const fn asm_date_to_rd((y, m, d): (i32, u8, u8)) -> i32 {
    datealgo::date_to_rd((y, m, d))
}
#[no_mangle]
pub const fn asm_rd_to_weekday(n: i32) -> u8 {
    datealgo::rd_to_weekday(n)
}
#[no_mangle]
pub const fn asm_date_to_weekday((y, m, d): (i32, u8, u8)) -> u8 {
    datealgo::date_to_weekday((y, m, d))
}
#[no_mangle]
pub const fn asm_next_date((y, m, d): (i32, u8, u8)) -> (i32, u8, u8) {
    datealgo::next_date((y, m, d))
}
#[no_mangle]
pub const fn asm_prev_date((y, m, d): (i32, u8, u8)) -> (i32, u8, u8) {
    datealgo::prev_date((y, m, d))
}
#[no_mangle]
pub const fn asm_secs_to_dhms(secs: i64) -> (i32, u8, u8, u8) {
    datealgo::secs_to_dhms(secs)
}
#[no_mangle]
pub const fn asm_dhms_to_secs((d, h, m, s): (i32, u8, u8, u8)) -> i64 {
    datealgo::dhms_to_secs((d, h, m, s))
}
#[no_mangle]
pub const fn asm_secs_to_datetime(secs: i64) -> (i32, u8, u8, u8, u8, u8) {
    datealgo::secs_to_datetime(secs)
}
#[no_mangle]
pub const fn asm_datetime_to_secs((y, m, d, hh, mm, ss): (i32, u8, u8, u8, u8, u8)) -> i64 {
    datealgo::datetime_to_secs((y, m, d, hh, mm, ss))
}
#[no_mangle]
pub const fn asm_is_leap_year(y: i32) -> bool {
    datealgo::is_leap_year(y)
}
#[no_mangle]
pub const fn asm_days_in_month(y: i32, m: u8) -> u8 {
    datealgo::days_in_month(y, m)
}
#[no_mangle]
pub const fn asm_rd_to_isoweekdate(rd: i32) -> (i32, u8, u8) {
    datealgo::rd_to_isoweekdate(rd)
}
#[no_mangle]
pub const fn asm_isoweekdate_to_rd((y, w, d): (i32, u8, u8)) -> i32 {
    datealgo::isoweekdate_to_rd((y, w, d))
}
#[no_mangle]
pub const fn asm_date_to_isoweekdate((y, m, d): (i32, u8, u8)) -> (i32, u8, u8) {
    datealgo::date_to_isoweekdate((y, m, d))
}
#[no_mangle]
pub const fn asm_isoweekdate_to_date((y, w, d): (i32, u8, u8)) -> (i32, u8, u8) {
    datealgo::isoweekdate_to_date((y, w, d))
}
#[no_mangle]
pub const fn asm_isoweeks_in_year(y: i32) -> u8 {
    datealgo::isoweeks_in_year(y)
}
#[cfg(feature = "std")]
#[no_mangle]
pub fn asm_systemtime_to_secs(st: SystemTime) -> Option<(i64, u32)> {
    datealgo::systemtime_to_secs(st)
}
#[cfg(feature = "std")]
#[no_mangle]
pub fn asm_secs_to_systemtime((secs, nsecs): (i64, u32)) -> Option<SystemTime> {
    datealgo::secs_to_systemtime((secs, nsecs))
}
#[cfg(feature = "std")]
#[no_mangle]
pub fn asm_systemtime_to_datetime(st: SystemTime) -> Option<(i32, u8, u8, u8, u8, u8, u32)> {
    datealgo::systemtime_to_datetime(st)
}
#[cfg(feature = "std")]
#[no_mangle]
pub fn asm_datetime_to_systemtime((y, m, d, hh, mm, ss, nsec): (i32, u8, u8, u8, u8, u8, u32)) -> Option<SystemTime> {
    datealgo::datetime_to_systemtime((y, m, d, hh, mm, ss, nsec))
}
