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

pub struct Knapsack<'a> {
    contents: &'a [Clock],
}

impl<'a> Knapsack<'a> {
    pub fn from_clocks(clocks: &'a [Clock]) -> Knapsack<'a> {
        Knapsack { contents: clocks }
    }

    pub fn empty() -> Knapsack<'a> {
        Knapsack { contents: &[] }
    }

    pub fn get_contents(&self) -> &[Clock] {
        self.contents
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
        let filled_knapsack = Knapsack::from_clocks(&clocks);
        assert_eq!(&clocks, filled_knapsack.get_contents());
    }

    #[rstest]
    fn one_should_be_able_to_add_clocks_to_contents_of_knapsack() {

    }
}
