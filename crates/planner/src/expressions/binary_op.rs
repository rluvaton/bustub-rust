use std::rc::Rc;
use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::{BinaryOp, BinaryOpExpr};
use expression::{ArithmeticExpression, ExpressionRef};
use crate::expressions::traits::PlanExpression;

impl PlanExpression for BinaryOpExpr {
    fn plan<'a>(&self, children: Vec<Rc<PlanType>>, planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        todo!()
        // match self.op {
        //     BinaryOp::Plus | BinaryOp::Minus | BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Modulo => ArithmeticExpression::new()
        //
        //     BinaryOp::Gt => {}
        //     BinaryOp::Lt => {}
        //     BinaryOp::GtEq => {}
        //     BinaryOp::LtEq => {}
        //     BinaryOp::Eq => {}
        //     BinaryOp::NotEq => {}
        //     BinaryOp::And => {}
        //     BinaryOp::Or => {}
        // }
    }
}
