use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
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
    group.bench_with_input(BenchmarkId::new("HASHMAP_WITHOUT_CAPACITY - O(N)", i), &nums1,
      |b, nums1| b.iter(|| Solution::longest_consecutive_og(nums1.to_vec())));
    let nums2 = nums.clone();
    group.bench_with_input(BenchmarkId::new("HASHMAP_WITH_CAPACITY - O(N)", i), &nums2,
      |b, nums2| b.iter(|| Solution::longest_consecutive_with_capacity(nums2.to_vec())));
    let nums3 = nums.clone();
    group.bench_with_input(BenchmarkId::new("HASHMAP_WITH_CAPACITY_AND_DEFAULT_HASH - O(N)", i), &nums3,
      |b, nums3| b.iter(|| Solution::longest_consecutive_with_capacity_and_default_hash(nums3.to_vec())));
    let nums4 = nums.clone();
    group.bench_with_input(BenchmarkId::new("HASHSET", i), &nums4,
      |b, nums4| b.iter(|| Solution::longest_consecutive_hashset(nums4.to_vec())));
    let nums5 = nums.clone();
    group.bench_with_input(BenchmarkId::new("SORT - O(NLOGN)", i), &nums5,
    |b, nums5| b.iter(|| Solution::longest_consecutive_sort(nums5.to_vec())));
    let nums6 = nums.clone();
    group.bench_with_input(BenchmarkId::new("AHASHMAP", i), &nums6,
    |b, nums6| b.iter(|| Solution::longest_consecutive_ahashmap(nums6.to_vec())));
    let nums7 = nums.clone();
    group.bench_with_input(BenchmarkId::new("AHASHSET", i), &nums7,
    |b, nums7| b.iter(|| Solution::longest_consecutive_ahashset(nums7.to_vec())));
    let nums8 = nums.clone();
    group.bench_with_input(BenchmarkId::new("HASHMAP_WITH_CAPACITY_AND_RANDOM_STATE - O(n)", i), &nums8,
    |b, nums8| b.iter(|| Solution::longest_consecutive_with_capacity_and_random_state(nums8.to_vec())));
    let nums9 = nums.clone();
    group.bench_with_input(BenchmarkId::new("HASHMAP_WITHOUT_CAPACITY_AND_RANDOM_STATE - O(n)", i), &nums9,
    |b, nums9| b.iter(|| Solution::longest_consecutive_without_capacity_and_random_state(nums9.to_vec())));
    let nums10 = nums.clone();
    group.bench_with_input(BenchmarkId::new("HASHMAP_WITH_CAPACITY_AND_REDUCED - O(n)", i), &nums10,
    |b, nums10| b.iter(|| Solution::longest_consecutive_og_reduced(nums10.to_vec())));
    let nums11 = nums.clone();
    group.bench_with_input(BenchmarkId::new("AHASHMAP_AND_REDUCED - O(n)", i), &nums11,
    |b, nums11| b.iter(|| Solution::longest_consecutive_ahashmap_reduced(nums11.to_vec())));
  }
  group.finish();
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);