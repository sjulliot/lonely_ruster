use num_rational::Rational32;
use itertools::Itertools;

pub mod combinations;

/// Check if a given time `t` is a solution for the given set of speeds.
pub fn is_solution(t: Rational32, speeds: &[i32]) -> bool {
    let n = speeds.len() as i32;
    for &other_speed in speeds {
        let position = (Rational32::from(other_speed) * t).fract();
        let min_distance = position.min(Rational32::from(1) - position);
        if min_distance < Rational32::new(1, n + 1) {
            return false;
        }
    }
    true
}

/// Find the smallest time `t` at which the stationary runner is lonely.
pub fn find_lonely_time(speeds: &[i32]) -> Option<Rational32> {
    let n = speeds.len() as i32;
    if n < 1 {
        return None; // At least one runner is required
    }

    // Iterate over each speed to find the smallest t
    let mut min_t: Option<Rational32> = None;
    for &speed in speeds {
        for k in 0..=(speed - 1) {
            let t = Rational32::new(k, speed) + Rational32::new(1, (n + 1) * speed);

            if t > min_t.unwrap_or(Rational32::new(1, 1)) {
                break; // No need to check larger k for this speed
            }

            if is_solution(t, speeds) {
                if min_t.is_none() || t < min_t.unwrap() {
                    min_t = Some(t);
                }
                break; // No need to check larger k for this speed
            }
        }
    }

    min_t
}

/// Generate all combinations of `n-1` positive integers less than `max_speed`.
pub fn generate_combinations(n: usize, max_speed: i32) -> Vec<Vec<i32>> {
    (1..max_speed)
        .combinations(n - 1)
        .collect()
}