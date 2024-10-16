use std::rc::Rc;
use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::{BinaryOpExpr, FuncCallExpr};
use expression::{Expression, ExpressionRef, StringExpression, StringExpressionType};
use crate::constants::UNNAMED_COLUMN;
use crate::expressions::traits::PlanExpression;

impl PlanExpression for FuncCallExpr {
    fn plan<'a>(&self, children: &Vec<Rc<PlanType>>, planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        let mut args = self.args
            .iter()
            .map(|arg| arg.plan(children, planner).1)
            .collect::<Vec<_>>();


        // 1. check if the parsed function name is "lower" or "upper".
        // 2. verify the number of args (should be 1), refer to the test cases for when you should throw an `Exception`.
        // 3. return a `StringExpression` std::shared_ptr.

        let expr = match self.func.as_str() {
            "lower" => {
                assert_eq!(args.len(), 1, "Number of args must be 1");

                let arg = args.pop().unwrap();

                StringExpression::new(arg, StringExpressionType::Lower).into_ref()
            },
            "upper" => {
                assert_eq!(args.len(), 1, "Number of args must be 1");

                let arg = args.pop().unwrap();

                StringExpression::new(arg, StringExpressionType::Upper).into_ref()
            },
            _ => unimplemented!(format!("Function {} not supported yet", self.func))
        };

        (UNNAMED_COLUMN.to_string(), expr)
    }
}
