use crate::expressions::{ColumnRef, ExpressionTypeImpl};
use crate::table_ref::join_ref::join_constraint_ext::JoinConstraintExt;
use crate::table_ref::join_ref::JoinType;
use crate::table_ref::table_reference_type::TableReferenceTypeImpl;
use crate::table_ref::TableRef;
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::{Join, JoinOperator, TableFactor, TableWithJoins};


/// A join. e.g., `SELECT * FROM x INNER JOIN y ON ...`, where `x INNER JOIN y ON ...` is `BoundJoinRef`.
#[derive(Clone, Debug, PartialEq)]
pub struct JoinRef {
    /// Type of join.
    pub join_type: JoinType,

    /// The left side of the join.
    pub left: Box<TableReferenceTypeImpl>,

    /** The right side of the join. */
    pub right: Box<TableReferenceTypeImpl>,

    /** Join condition. */
    pub condition: Option<ExpressionTypeImpl>,
}

impl JoinRef {
    pub(crate) fn parse_from_table_with_join(ast: &TableWithJoins, binder: &Binder) -> ParseASTResult<TableReferenceTypeImpl> {
        if ast.joins.is_empty() {
            return Err(ParseASTError::IncompatibleType);
        }

        let (first_join, joins) = ast.joins.split_first().unwrap();

        let join_ref = Self::parse_from_table_and_join(TableReferenceTypeImpl::try_from_ast(&ast.relation, binder)?, first_join, binder)?;

        joins
            .iter()
            .try_fold(TableReferenceTypeImpl::Join(join_ref), |left, join| Self::parse_from_table_and_join(left, join, binder).map(|r| r.into()))
    }

    fn parse_from_table_and_join(table_ref: TableReferenceTypeImpl, join: &Join, binder: &Binder) -> ParseASTResult<Self> {
        let (join_type, join_constraint) = match &join.join_operator {
            JoinOperator::Inner(join_constraint) => (JoinType::Inner, join_constraint),
            JoinOperator::LeftOuter(join_constraint) => (JoinType::Left, join_constraint),
            JoinOperator::RightOuter(join_constraint) => (JoinType::Right, join_constraint),
            JoinOperator::FullOuter(join_constraint) => (JoinType::Outer, join_constraint),
            _ => return Err(ParseASTError::Unimplemented(format!("join operator {:?} is not supported", join.join_operator)))
        };

        Ok(JoinRef {
            join_type,
            left: Box::new(table_ref),
            right: Box::new(TableReferenceTypeImpl::try_from_ast(&join.relation, binder)?),
            condition: join_constraint.convert_to_condition(binder)?,
        })
    }
}

impl TableRef for JoinRef {
    fn resolve_column(&self, col_name: &[String], binder: &Binder) -> ParseASTResult<Option<ColumnRef>> {
        let left_column = self.left.resolve_column(col_name, binder)?;
        let right_column = self.right.resolve_column(col_name, binder)?;

        if left_column.is_some() && right_column.is_some() {
            return Err(ParseASTError::FailedParsing(format!("{} is ambiguous", col_name.join("."))))
        }

        Ok(left_column.or(right_column))
    }

    fn get_all_columns(&self, binder: &Binder) -> ParseASTResult<Vec<ExpressionTypeImpl>> {
        Ok([
            self.left.get_all_columns(binder)?,
            self.right.get_all_columns(binder)?
        ].concat())
    }

    fn try_from_ast(_ast: &TableFactor, _binder: &Binder) -> ParseASTResult<Self> {
        // Always incompatible as we need to have table with joins
        Err(ParseASTError::IncompatibleType)
    }
}

impl From<JoinRef> for TableReferenceTypeImpl {
    fn from(value: JoinRef) -> Self {
        TableReferenceTypeImpl::Join(value)
    }
}

