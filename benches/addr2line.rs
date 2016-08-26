#![feature(test)]

extern crate test;

use std::process::Command;

fn add_obj_file_arg(cmd: &mut Command) -> &mut Command {
    cmd.arg("/Users/fitzgen/src/gimli/target/debug/gimli-2ffa6594baf9fafb.dSYM/Contents/Resources/DWARF/gimli-2ffa6594baf9fafb")
}

fn add_addrs_args(cmd: &mut Command) -> &mut Command {
    cmd
        .arg("0x100000f3a")
        .arg("0x100000f3c")
        .arg("0x100000f43")
        .arg("0x100000f4a")
        .arg("0x100000f50")
        .arg("0x100000f55")
        .arg("0x100000f5c")
        .arg("0x100000f5f")
        .arg("0x100000f65")
        .arg("0x100000f55")
        .arg("0x100000f5c")
        .arg("0x100000f5f")
        .arg("0x100000f65")
        .arg("0x100000f6c")
        .arg("0x100000f71")
        .arg("0x100000f75")
        .arg("0x100000f79")
        .arg("0x100000f7d")
        .arg("0x100000f89")
        .arg("0x100000f8d")
        .arg("0x100000f91")
        .arg("0x100000f98")
        .arg("0x100000f9f")
        .arg("0x100000fa4")
        .arg("0x100000fa8")
        .arg("0x100000faf")
        .arg("0x100000fbf")
        .arg("0x100000fc3")
        .arg("0x100000fc7")
        .arg("0x100000fcb")
        .arg("0x100000fd0")
        .arg("0x100000fd4")
        .arg("0x100000fd8")
        .arg("0x100000fdc")
        .arg("0x100000fd0")
        .arg("0x100000fd4")
        .arg("0x100000fd8")
        .arg("0x100000fdc")
        .arg("0x100000fe0")
        .arg("0x100000fe5")
        .arg("0x100000fe8")
        .arg("0x100000feb")
        .arg("0x100000fee")
        .arg("0x100001017")
        .arg("0x10000101b")
        .arg("0x10000101f")
        .arg("0x100001023")
        .arg("0x100001027")
        .arg("0x10000102c")
        .arg("0x100001030")
        .arg("0x100001034")
        .arg("0x100001060")
        .arg("0x100001064")
        .arg("0x100001067")
        .arg("0x10000106b")
        .arg("0x100001070")
        .arg("0x100001073")
        .arg("0x100001076")
        .arg("0x100001079")
        .arg("0x100001114")
        .arg("0x100001117")
        .arg("0x10000111b")
        .arg("0x10000111f")
        .arg("0x100001124")
        .arg("0x100001127")
        .arg("0x10000112a")
        .arg("0x10000112d")
        .arg("0x1000011ea")
        .arg("0x1000011ec")
        .arg("0x1000011f3")
        .arg("0x1000011fa")
        .arg("0x100001200")
        .arg("0x100001205")
        .arg("0x10000120c")
        .arg("0x10000120f")
        .arg("0x100001215")
        .arg("0x100001205")
        .arg("0x10000120c")
        .arg("0x10000120f")
        .arg("0x100001215")
        .arg("0x10000121c")
        .arg("0x100001221")
        .arg("0x100001225")
        .arg("0x100001229")
        .arg("0x10000122d")
        .arg("0x100001239")
        .arg("0x10000123d")
        .arg("0x100001241")
        .arg("0x100001248")
        .arg("0x10000124f")
        .arg("0x100001254")
        .arg("0x100001258")
        .arg("0x10000125f")
        .arg("0x10000126f")
        .arg("0x100001273")
        .arg("0x100001277")
        .arg("0x10000127b")
        .arg("0x100001280")
        .arg("0x100001284")
        .arg("0x100001288")
}

#[bench]
fn bench_atos_100_addrs(b: &mut test::Bencher) {
    b.iter(|| {
        let mut cmd = Command::new("/usr/bin/atos");
        let mut cmd = cmd.arg("-o");
        let mut cmd = add_obj_file_arg(cmd);
        let mut cmd = add_addrs_args(cmd);
        let out = cmd.output().expect("failed to execute process");
        assert!(out.status.success());
    });
}

#[bench]
fn bench_example_addr2line_100_addrs(b: &mut test::Bencher) {
    b.iter(|| {
        let mut cmd = Command::new("/Users/fitzgen/src/gimli/target/release/examples/addr2line");
        let mut cmd = cmd.arg("-e");
        let mut cmd = add_obj_file_arg(cmd);
        let mut cmd = add_addrs_args(cmd);
        let out = cmd.output().expect("failed to execute process");
        assert!(out.status.success());
    });
}

#[bench]
fn bench_atos_1_addr(b: &mut test::Bencher) {
    b.iter(|| {
        let mut cmd = Command::new("/usr/bin/atos");
        let mut cmd = cmd.arg("-o");
        let mut cmd = add_obj_file_arg(cmd);
        let mut cmd = cmd.arg("0x100001288");
        let out = cmd.output().expect("failed to execute process");
        assert!(out.status.success());
    });
}

#[bench]
fn bench_example_addr2line_1_addr(b: &mut test::Bencher) {
    b.iter(|| {
        let mut cmd = Command::new("/Users/fitzgen/src/gimli/target/release/examples/addr2line");
        let mut cmd = cmd.arg("-e");
        let mut cmd = add_obj_file_arg(cmd);
        let mut cmd = cmd.arg("0x100001288");
        let out = cmd.output().expect("failed to execute process");
        assert!(out.status.success());
    });
}
