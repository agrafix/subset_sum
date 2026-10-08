use dpss::dp::find_subset;

fn exhaustive(weights: &[i64], target: i64, max_length: usize) -> Vec<Vec<i64>> {
    let mut answers = Vec::new();
    for mask in 1usize..1 << weights.len() {
        if mask.count_ones() as usize > max_length {
            continue;
        }
        let mut subset: Vec<_> = weights
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, &w)| w)
            .collect();
        if subset.iter().sum::<i64>() == target {
            subset.sort_unstable();
            answers.push(subset);
        }
    }
    answers.sort_unstable();
    answers
}

#[test]
fn reconstructs_all_signed_subsets_including_duplicate_witnesses() {
    let mut state = 0x8f21_ade7_1234_5678u64;
    let mut random = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for case in 0..1600 {
        let len = case % 10;
        let weights: Vec<i64> = (0..len).map(|_| (random() % 13) as i64 - 6).collect();
        let target = (random() % 41) as i64 - 20;
        let max_length = random() as usize % (len + 2);
        let expected = exhaustive(&weights, target, max_length);
        let mut actual = find_subset(weights.clone(), target, max_length);
        for answer in &mut actual {
            answer.sort_unstable();
        }
        actual.sort_unstable();
        assert_eq!(
            actual, expected,
            "weights={weights:?}, target={target}, max_length={max_length}"
        );
    }
}
