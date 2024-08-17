use capillary::{vec_dict::VecDict, Dictionary};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn search_comp(c: &mut Criterion) {
    let (key, value) = ("Some key to insert", "Substitution");
    let mut vec_dic = VecDict::default();
    let mut dict = Dictionary::default();

    vec_dic.insert(key.chars(), value);
    dict.insert(key.chars(), value);

    let mut group = c.benchmark_group("Comparing Dictionary and VecDict");

    group.bench_with_input(BenchmarkId::new("Dictionary", 1), &1, |b, _i| {
        b.iter(|| {
            let _ = dict.get(key.chars());
        });
    });

    group.bench_with_input(BenchmarkId::new("VecDict", 1), &1, |b, _i| {
        b.iter(|| {
            let _ = vec_dic.get(key.chars());
        });
    });
}

criterion_group!(benches, search_comp);
criterion_main!(benches);
