use diesel::SelectableHelper;

use crate::database::models::{Budget, NewBudget};

// TODO CHECK IF THERE IS ALREADY A BUDGET
// IF THERE IS, RETURN THE BUDGET
#[allow(dead_code)]
pub fn create_budget(amount: i32) {
    #[allow(unused_variables)]
    let budget = NewBudget { amount };
}
pub fn get_budget()  {
}