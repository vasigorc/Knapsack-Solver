use rstest::rstest;

#[derive(Debug, PartialEq, Clone)]
pub struct Clock {
    weight: f32,
    price: f32,
}

impl Clock {
    pub fn new(weight: f32, price: f32) -> Clock {
        Clock { weight, price }
    }
}

pub struct Knapsack {
    contents: Vec<Clock>,
}

impl Knapsack {
    pub fn from_clocks(clocks: Vec<Clock>) -> Knapsack {
        Knapsack { contents: clocks }
    }

    pub fn empty() -> Knapsack {
        Knapsack::from_clocks(Vec::new())
    }

    pub fn get_contents(&self) -> &Vec<Clock> {
        &self.contents
    }
}

#[cfg(test)]
mod tests {
    use crate::Knapsack;

    use super::*;

    #[rstest]
    fn empty_knapsack_should_contain_no_clocks() {
        let empty_knapsack = Knapsack::empty();
        assert!(empty_knapsack.contents.is_empty())
    }

    #[rstest]
    fn filled_knapsack_should_contain_all_clocks_passed_at_construction() {
        let clocks = vec![
            Clock::new(0.5, 19.99),
            Clock::new(0.75, 29.99),
            Clock::new(0.9, 39.99),
        ];
        let filled_knapsack = Knapsack::from_clocks(clocks.clone());
        assert_eq!(&clocks, filled_knapsack.get_contents());
    }
}
