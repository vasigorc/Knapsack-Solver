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

impl PartialEq<&Clock> for Clock {
    fn eq(&self, other: &&Clock) -> bool {
        self == *other
    }
}

pub struct Knapsack<'a> {
  contents: Vec<&'a Clock>,
}

impl<'a> Knapsack<'a> {
  pub fn from_clocks(clocks: &'a [Clock]) -> Knapsack<'a> {
      Knapsack {
          contents: clocks.iter().collect(),
      }
  }

  pub fn empty() -> Knapsack<'a> {
      Knapsack {
          contents: Vec::new(),
      }
  }

  pub fn get_contents(&self) -> &[&'a Clock] {
      &self.contents
  }

  pub fn add_clock(&self, clock: &'a Clock) -> Knapsack<'a> {
      let mut new_contents = self.contents.clone();
      new_contents.push(clock);

      Knapsack {
          contents: new_contents,
      }
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
      let knapsack = Knapsack::empty();
      let clock = Clock::new(4.45, 2.29);
      let updated_knapsack = knapsack.add_clock(&clock);

      
      let expected_contents = vec![&clock];
      let actual_contents = updated_knapsack.get_contents();

      assert_eq!(&expected_contents[..], actual_contents);
    }
}
