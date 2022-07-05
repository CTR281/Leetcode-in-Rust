use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::{Rng, distributions::Uniform};

use leetcode::Solution;

fn criterion_benchmark(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  let range = Uniform::new(-1_000_000_000, 1_000_000_000);
  let input: Vec<Vec<i32>> = 
  (0..1)
  .map(|_| 
    (0..10_000)
    .map(|_| rng.sample(&range))
    .collect()
  )
  .collect();

  let mut group = c.benchmark_group("longest_consecutive");
  for (i, nums) in input.iter().enumerate() {
    let nums1 = nums.clone();
    group.bench_with_input(BenchmarkId::new("HASHMAP_OG - O(N)", i), &nums1,
      |b, nums1| b.iter(|| Solution::longest_consecutive_OG(nums1.to_vec())));
    let nums2 = nums.clone();
    group.bench_with_input(BenchmarkId::new("HASHMAP_WITH_CAPACITY - O(N)", i), &nums2,
      |b, nums2| b.iter(|| Solution::longest_consecutive_with_capacity(nums2.to_vec())));
    let nums3 = nums.clone();
    group.bench_with_input(BenchmarkId::new("HASHMAP_WITH_CAPACITY_AND_CUSTOM_HASH - O(N)", i), &nums3,
      |b, nums3| b.iter(|| Solution::longest_consecutive_with_capacity_and_custom_hash(nums3.to_vec())));
    let nums4 = nums.clone();
    group.bench_with_input(BenchmarkId::new("HASHSET", i), &nums4,
      |b, nums4| b.iter(|| Solution::longest_consecutive_hashset(nums4.to_vec())));
    let nums5 = nums.clone();
    group.bench_with_input(BenchmarkId::new("SORT - O(NLOGN)", i), &nums5,
    |b, nums5| b.iter(|| Solution::longest_consecutive_sort(nums5.to_vec())));
    let nums6 = nums.clone();
    group.bench_with_input(BenchmarkId::new("AHASHMAP", i), &nums6,
    |b, nums6| b.iter(|| Solution::longest_consecutive_AHASH(nums6.to_vec())));
    let nums7 = nums.clone();
    group.bench_with_input(BenchmarkId::new("AHASHSET", i), &nums7,
    |b, nums7| b.iter(|| Solution::longest_consecutive_AHASH(nums7.to_vec())));
  }
  group.finish();
  /* c.bench_function("longest_consecutive - HASHMAP", |b| 
    b.iter(|| Solution::longest_consecutive2(
      (0..10_000)
      .map(|_| rng.sample(&range))
      .collect()))
  ); */
  //c.bench_function("longest_consecutive - HASHSET", |b| b.iter(|| Solution::longest_consecutive3(black_box(vec![1, 2, 3]))));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);