#![cfg(feature = "backend")]

mod backend;

use std::assert_eq;

use backend::get_db_connection;
use mysql::prelude::*;

const SETUP: &[&str] = &[
    "CREATE OR REPLACE FUNCTION levenshtein RETURNS integer
        SONAME 'libudf_stringops.so'",
    "CREATE OR REPLACE FUNCTION levenshtein_normalized RETURNS real
        SONAME 'libudf_stringops.so'",
];

/// `(a, b, result)`
const TESTS: &[((&str, &str), i64)] = &[
    (("abcd", "abcd"), 0),
    (("abcd", "ab"), 2),
    (("ab", "abcd"), 2),
    (("abcd", "ad"), 2),
    (("abcd", "cd"), 2),
    (("abcd", "a"), 3),
    (("abcd", "c"), 3),
    (("abcd", "accd"), 1),
    (("kitten", "sitting"), 3),
    (("sitting", "kitten"), 3),
    (("not", "to a"), 3),
    (("to be a bee", "not to bee"), 6),
];

/// `(a, b, limit, result)`
const TESTS_LIMIT: &[((&str, &str, u32), i64)] = &[
    (("abcd", "abcd", 1), 0),
    (("abcdef", "", 3), 3),
    (("", "abcdef", 3), 3),
    (("abcdef", "", 8), 6),
    (("", "abcdef", 8), 6),
    (("abcdef", "000000", 3), 3),
    (("ab", "0000", 3), 3),
];

/// `(a, b, result)`
const TESTS_NORMALIZED: &[((&str, &str), f64)] = &[
    (("abcd", "abcd"), 0.0),
    (("abcd", "ab"), 0.5),
    (("ab", "abcd"), 0.5),
    (("abcd", "ad"), 0.5),
    (("abcd", "cd"), 0.5),
    (("abcd", "a"), 0.75),
    (("abcd", "c"), 0.75),
    (("abcd", "accd"), 0.25),
    (("kitten", "sitting"), 0.42),
    (("sitting", "kitten"), 0.42),
    (("not", "to a"), 0.75),
    (("to be a bee", "not to bee"), 0.54),
];

/// `(a, b, limit, result)`
const TESTS_NORMALIZED_LIMIT: &[((&str, &str, f64), f64)] = &[
    (("abcd", "abcd", 1.0), 0.0),
    (("abcdef", "", 0.2), 0.2),
    (("", "abcdef", 0.2), 0.2),
    (("abcdef", "", 0.6), 0.6),
    (("", "abcdef", 0.6), 0.6),
    (("abcdef", "000000", 0.5), 0.5),
    (("ab", "0000", 0.5), 0.5),
];

#[test]
fn test_levenshtein() {
    let conn = &mut get_db_connection(SETUP);

    for (params, expected) in TESTS {
        let res: i64 = conn
            .exec_first("select levenshtein(?, ?)", params)
            .unwrap()
            .unwrap();

        assert_eq!(res, *expected, "params {params:?} -> {expected} failed");
    }
}

#[test]
fn test_levenshtein_limit() {
    let conn = &mut get_db_connection(SETUP);

    for (params, expected) in TESTS_LIMIT {
        let res: i64 = conn
            .exec_first("select levenshtein(?, ?, ?)", params)
            .unwrap()
            .unwrap();

        assert_eq!(res, *expected, "params {params:?} -> {expected}failed");
    }
}

#[test]
fn test_levenshtein_normalized() {
    let conn = &mut get_db_connection(SETUP);

    for (params, expected) in TESTS_NORMALIZED {
        let res: f64 = conn
            .exec_first("select levenshtein_normalized(?, ?)", params)
            .unwrap()
            .unwrap();

        assert!(
            approx_eq(res, *expected),
            "params {params:?} -> {expected} failed: {res}"
        );
    }
}

#[test]
fn test_levenshtein_normalized_limit() {
    let conn = &mut get_db_connection(SETUP);

    for (params, expected) in TESTS_NORMALIZED_LIMIT {
        let res: f64 = conn
            .exec_first("select levenshtein_normalized(?, ?, ?)", params)
            .unwrap()
            .unwrap();

        assert!(
            approx_eq(res, *expected),
            "params {params:?} -> {expected} failed {res}"
        );
    }
}

fn approx_eq(a: f64, b: f64) -> bool {
    const EPSILON: f64 = 0.01;
    (a - b).abs() < EPSILON
}
