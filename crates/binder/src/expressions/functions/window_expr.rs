use crate::expressions::{Expression, ExpressionTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::{Expr, WindowFrame, WindowFrameBound, WindowFrameUnits, WindowType};
use crate::expressions::functions::function_ext::FunctionExt;
use crate::expressions::functions::utils::is_aggregation_function_name;
use crate::order_by::OrderBy;

#[derive(Clone, Debug, PartialEq)]
pub enum WindowBoundary {
    UnboundedPreceding,
    UnboundedFollowing,
    CurrentRowRange,
    CurrentRowRows,
    ExprPrecedingRows(Box<ExpressionTypeImpl>),
    ExprFollowingRows(Box<ExpressionTypeImpl>),
    ExprPrecedingRange(Box<ExpressionTypeImpl>),
    ExprFollowingRange(Box<ExpressionTypeImpl>),
}

impl WindowBoundary {
    fn from_window_frame(units: &WindowFrameUnits, bounds: &WindowFrameBound, binder: &mut Binder) -> ParseASTResult<WindowBoundary> {
        Ok(match (units, bounds) {
            (WindowFrameUnits::Rows, WindowFrameBound::CurrentRow) => WindowBoundary::CurrentRowRows,
            (WindowFrameUnits::Range, WindowFrameBound::CurrentRow) => WindowBoundary::CurrentRowRange,

            (WindowFrameUnits::Rows, WindowFrameBound::Preceding(p)) => {
                if let Some(expr) = p {
                    WindowBoundary::ExprPrecedingRows(Box::new(ExpressionTypeImpl::try_parse_from_expr(&*expr, binder)?))
                } else {
                    WindowBoundary::UnboundedPreceding
                }
            }
            (WindowFrameUnits::Range, WindowFrameBound::Preceding(p)) => {
                if let Some(expr) = p {
                    WindowBoundary::ExprPrecedingRange(Box::new(ExpressionTypeImpl::try_parse_from_expr(&*expr, binder)?))
                } else {
                    WindowBoundary::UnboundedPreceding
                }
            }

            (WindowFrameUnits::Rows, WindowFrameBound::Following(p)) => {
                if let Some(expr) = p {
                    WindowBoundary::ExprFollowingRows(Box::new(ExpressionTypeImpl::try_parse_from_expr(&*expr, binder)?))
                } else {
                    WindowBoundary::UnboundedFollowing
                }
            }
            (WindowFrameUnits::Range, WindowFrameBound::Following(p)) => {
                if let Some(expr) = p {
                    WindowBoundary::ExprFollowingRange(Box::new(ExpressionTypeImpl::try_parse_from_expr(&*expr, binder)?))
                } else {
                    WindowBoundary::UnboundedFollowing
                }
            }

            _ => return Err(ParseASTError::Unimplemented(format!("unit {} and bound {} combination is not supported", units, bounds)))
        })
    }
}

/// A bound aggregate call, e.g., `sum(x)`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WindowExpr {
    // The function name
    pub(crate) func: String,

    /// function arguments
    pub(crate) args: Vec<Box<ExpressionTypeImpl>>,

    pub(crate) partition_by: Vec<Box<ExpressionTypeImpl>>,
    pub(crate) order_bys: Vec<Box<OrderBy>>,
    pub(crate) start: Option<WindowBoundary>,
    pub(crate) end: Option<WindowBoundary>,
}

impl WindowExpr {
    pub(crate) fn new(
        func: String,
        args: Vec<Box<ExpressionTypeImpl>>,

        partition_by: Vec<Box<ExpressionTypeImpl>>,
        order_bys: Vec<Box<OrderBy>>,
        start: Option<WindowBoundary>,
        end: Option<WindowBoundary>,
    ) -> Self {
        Self {
            func,
            args,
            partition_by,
            order_bys,
            start,
            end,
        }
    }

    pub(crate) fn is_window_function(f: &sqlparser::ast::Function) -> bool {
        is_aggregation_function_name(f.name.to_string()) && f.over.is_some()
    }
}

impl Into<ExpressionTypeImpl> for WindowExpr {
    fn into(self) -> ExpressionTypeImpl {
        ExpressionTypeImpl::Window(self)
    }
}

impl Expression for WindowExpr {
    fn has_aggregation(&self) -> bool {
        false
    }

    fn has_window_function(&self) -> bool {
        true
    }

    fn try_parse_from_expr(expr: &Expr, binder: &mut Binder) -> ParseASTResult<Self>
    where
        Self: Sized,
    {
        let f = match expr {
            Expr::Function(f) => {
                if Self::is_window_function(f) {
                    if f.is_distinct() {
                        return Err(ParseASTError::Unimplemented("DISTINCT is not supported in window functions".to_string()));
                    }

                    f
                } else {
                    return Err(ParseASTError::IncompatibleType);
                }
            }
            _ => return Err(ParseASTError::IncompatibleType)
        };

        let over = f.over.as_ref().unwrap();

        let spec = match over {
            WindowType::WindowSpec(s) => s,
            WindowType::NamedWindow(_) => return Err(ParseASTError::Unimplemented("named window is not supported in window functions".to_string()))
        };

        let partitions = ExpressionTypeImpl::parse_boxed_expression_list(spec.partition_by.as_ref(), binder)?;

        let sort: ParseASTResult<Vec<Box<OrderBy>>> = spec.order_by.iter().map(|item| OrderBy::parse_from_ast(item, binder).map(|expr| Box::new(expr))).collect();
        let sort = sort?;


        let mut start: Option<WindowBoundary> = None;
        let mut end: Option<WindowBoundary> = None;

        if let Some(frame) = &spec.window_frame {
            start = Some(WindowBoundary::from_window_frame(&frame.units, &frame.start_bound, binder)?);

            if let Some(end_bound) = &frame.end_bound {
                end = Some(WindowBoundary::from_window_frame(&frame.units, end_bound, binder)?);
            }
        }

        Ok(Self {
            func: f.name.to_string(),
            args: f.parse_args(binder)?,
            partition_by: partitions,
            order_bys: sort,
            start,
            end,
        })
    }
}
