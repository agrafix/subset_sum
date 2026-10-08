//! Run with `cargo bench --bench subset_sum`. No extra benchmark dependencies.
use dpss::dp::find_subset;
use std::hint::black_box;
use std::time::{Duration, Instant};

fn main() {
    let cases = [
        ("repeated", vec![100; 24], 151, 3),
        ("repeated-solutions", vec![100; 20], 300, 3),
        ("dense-solutions", (1..=18).collect(), 50, 8),
        (
            "clustered",
            (0..24).map(|i| 1_000 + 2 * (i % 7)).collect(),
            3_001,
            3,
        ),
        ("powers", (0..16).map(|i| 1 << i).collect(), 21_845, 8),
        ("signed", vec![75, 467, 512, -835, 770, -69, 10], 711, 3),
    ];
    for (name, weights, target, max_length) in cases {
        let expected = find_subset(weights.clone(), target, max_length);
        let start = Instant::now();
        let mut count = 0;
        while start.elapsed() < Duration::from_millis(300) {
            let result = find_subset(black_box(weights.clone()), black_box(target), max_length);
            assert_eq!(result, expected);
            black_box(result);
            count += 1;
        }
        println!(
            "{name}: {:.0} ns/iteration ({count} iterations)",
            start.elapsed().as_nanos() as f64 / count as f64
        );
    }
}
