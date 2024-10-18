use std::rc::Rc;
use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::{BinaryOp, BinaryOpExpr};
use expression::{ArithmeticExpression, ArithmeticExpressionType, ComparisonExpression, ComparisonType, Expression, ExpressionRef, LogicExpression, LogicType};
use crate::constants::UNNAMED_COLUMN;
use crate::expressions::traits::PlanExpression;

impl PlanExpression for BinaryOpExpr {
    fn plan<'a>(&self, children: &[&PlanType], planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        let (_, lhs) = self.larg.plan(children, planner);
        let (_, rhs) = self.rarg.plan(children, planner);

        let expr = match self.op {
            BinaryOp::Plus => ArithmeticExpression::new(lhs, rhs, ArithmeticExpressionType::Plus).into_ref(),
            BinaryOp::Minus => ArithmeticExpression::new(lhs, rhs, ArithmeticExpressionType::Minus).into_ref(),
            BinaryOp::Multiply => ArithmeticExpression::new(lhs, rhs, ArithmeticExpressionType::Multiply).into_ref(),
            BinaryOp::Divide => ArithmeticExpression::new(lhs, rhs, ArithmeticExpressionType::Divide).into_ref(),
            BinaryOp::Modulo => ArithmeticExpression::new(lhs, rhs, ArithmeticExpressionType::Modulo).into_ref(),

            BinaryOp::Gt => ComparisonExpression::new(lhs, rhs, ComparisonType::GreaterThan).into_ref(),
            BinaryOp::Lt => ComparisonExpression::new(lhs, rhs, ComparisonType::LessThan).into_ref(),
            BinaryOp::GtEq => ComparisonExpression::new(lhs, rhs, ComparisonType::GreaterThanOrEqual).into_ref(),
            BinaryOp::LtEq => ComparisonExpression::new(lhs, rhs, ComparisonType::LessThanOrEqual).into_ref(),
            BinaryOp::Eq => ComparisonExpression::new(lhs, rhs, ComparisonType::Equal).into_ref(),
            BinaryOp::NotEq => ComparisonExpression::new(lhs, rhs, ComparisonType::NotEqual).into_ref(),

            BinaryOp::And => LogicExpression::new(lhs, rhs, LogicType::And).into_ref(),
            BinaryOp::Or => LogicExpression::new(lhs, rhs, LogicType::Or).into_ref(),
        };

        (UNNAMED_COLUMN.to_string(), expr)
    }
}
