/// Custom iterator for generating combinations.
pub struct Combinations {
    // n: usize,
    max_speed: i32,
    current: Vec<i32>,
}

impl Combinations {
    pub fn new(n: usize, max_speed: i32) -> Self {
        Self {
            // n,
            max_speed,
            current: (1..n).map(|x| x as i32).collect(),
        }
    }
}

impl Iterator for Combinations {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_empty() {
            return None;
        }

        let result = self.current.clone();

        for i in (0..self.current.len()).rev() {
            if self.current[i] < self.max_speed - (self.current.len() - i) as i32 {
                self.current[i] += 1;
                for j in i + 1..self.current.len() {
                    self.current[j] = self.current[j - 1] + 1;
                }
                return Some(result);
            }
        }

        self.current.clear();
        Some(result)
    }
}