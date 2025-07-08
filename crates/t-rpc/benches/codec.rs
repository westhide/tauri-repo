mod codec_impl;

use criterion::{Criterion, criterion_group, criterion_main};
use t_rpc::protos::internal::Username;

pub fn benchmark(c: &mut Criterion) {
    let username = format!("username");
    let pt = Username { username };
    let mut g = c.benchmark_group("codec");
    g.bench_function("rkyv", |b| b.iter(|| codec_impl::rkyv::roundtrip(pt.clone())));
    g.bench_function("prost", |b| b.iter(|| codec_impl::prost::roundtrip(pt.clone())));
}

criterion_group!(run, benchmark);
criterion_main!(run);
