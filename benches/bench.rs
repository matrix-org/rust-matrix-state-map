#![feature(test)]

extern crate state_map;
extern crate test;

use state_map::StateMap;
use test::Bencher;

#[bench]
fn bench_get_well_known(b: &mut Bencher) {
    let state_map: StateMap<_> = vec![
        (("m.room.power_levels", ""), 1),
        (("fooooo", ""), 2),
        (("bar", "example"), 3),
    ]
    .into_iter()
    .collect();

    b.iter(|| state_map.get("m.room.power_levels", ""));
}

#[bench]
fn bench_get_member(b: &mut Bencher) {
    let state_map: StateMap<_> = vec![
        (("m.room.power_levels", ""), 1),
        (("fooooo", ""), 2),
        (("bar", "example"), 3),
        (("m.room.member", "example"), 4),
    ]
    .into_iter()
    .collect();

    b.iter(|| state_map.get("m.room.member", "example"));
}

#[bench]
fn bench_get_other(b: &mut Bencher) {
    let state_map: StateMap<_> = vec![
        (("m.room.power_levels", ""), 1),
        (("fooooo", ""), 2),
        (("bar", "example"), 3),
    ]
    .into_iter()
    .collect();

    b.iter(|| state_map.get("fooooo", ""));
}

#[bench]
fn bench_get_missing(b: &mut Bencher) {
    let state_map: StateMap<_> = vec![
        (("m.room.power_levels", ""), 1),
        (("fooooo", ""), 2),
        (("bar", "example"), 3),
    ]
    .into_iter()
    .collect();

    b.iter(|| state_map.get("missing", ""));
}

#[bench]
fn bench_get_explicit_member(b: &mut Bencher) {
    let state_map: StateMap<_> = vec![
        (("m.room.power_levels", ""), 1),
        (("fooooo", ""), 2),
        (("bar", "example"), 3),
        (("m.room.member", "example"), 4),
    ]
    .into_iter()
    .collect();

    b.iter(|| state_map.get_membership("example"));
}

#[bench]
fn bench_insert_well_known(b: &mut Bencher) {
    let mut state_map: StateMap<_> = vec![
        (("m.room.power_levels", ""), 1),
        (("fooooo", ""), 2),
        (("bar", "example"), 3),
    ]
    .into_iter()
    .collect();

    b.iter(|| state_map.insert("m.room.create", "", 4));
}

#[bench]
fn bench_insert_other(b: &mut Bencher) {
    let mut state_map: StateMap<_> = vec![
        (("m.room.power_levels", ""), 1),
        (("fooooo", ""), 2),
        (("bar", "example"), 3),
    ]
    .into_iter()
    .collect();

    b.iter(|| state_map.insert("random", "", 4));
}

#[bench]
fn bench_insert_member(b: &mut Bencher) {
    let mut state_map: StateMap<_> = vec![
        (("m.room.power_levels", ""), 1),
        (("fooooo", ""), 2),
        (("bar", "example"), 3),
    ]
    .into_iter()
    .collect();

    b.iter(|| state_map.insert("m.room.member", "", 4));
}
