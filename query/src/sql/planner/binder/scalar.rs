// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::any::Any;
use std::sync::Arc;

use common_ast::ast::BinaryOperator;
use common_ast::ast::Expr;
use common_datavalues::DataTypeImpl;
use common_exception::ErrorCode;
use common_exception::Result;

use crate::sql::planner::binder::BindContext;
use crate::sql::plans::Scalar;

/// Helper for binding scalar expression with `BindContext`.
pub struct ScalarBinder;

impl ScalarBinder {
    pub fn new() -> Self {
        ScalarBinder {}
    }

    pub fn bind_expr(&self, expr: &Expr, bind_context: &BindContext) -> Result<ScalarExprRef> {
        match expr {
            Expr::ColumnRef { table, column, .. } => {
                let table_name: Option<String> = table.clone().map(|ident| ident.name);
                let column_name = column.name.clone();
                let column_binding = bind_context.resolve_column(table_name, column_name)?;

                Ok(Arc::new(Scalar::ColumnRef {
                    index: column_binding.index,
                    data_type: column_binding.data_type.clone(),
                    nullable: column_binding.nullable,
                }))
            }
            Expr::BinaryOp { op, left, right } => {
                self.bind_binary_op(op, left.as_ref(), right.as_ref(), bind_context)
            }
            _ => Err(ErrorCode::UnImplement(format!(
                "Unsupported expr: {:?}",
                expr
            ))),
        }
    }

    fn bind_binary_op(
        &self,
        op: &BinaryOperator,
        left_child: &Expr,
        right_child: &Expr,
        bind_context: &BindContext,
    ) -> Result<ScalarExprRef> {
        let left_scalar = self.bind_expr(left_child, bind_context)?;
        let right_scalar = self.bind_expr(right_child, bind_context)?;
        match op {
            BinaryOperator::Eq => Ok(Arc::new(Scalar::Equal {
                left: left_scalar,
                right: right_scalar,
            })),
            _ => Err(ErrorCode::UnImplement(format!(
                "Unsupported binary operator: {op}",
            ))),
        }
    }
}

pub type ScalarExprRef = Arc<dyn ScalarExpr>;

pub trait ScalarExpr: Any {
    /// Get return type and nullability
    fn data_type(&self) -> (DataTypeImpl, bool);

    // TODO: implement this in the future
    // fn used_columns(&self) -> ColumnSet;

    // TODO: implement this in the future
    // fn outer_columns(&self) -> ColumnSet;

    fn contains_aggregate(&self) -> bool;

    fn contains_subquery(&self) -> bool;

    fn as_any(&self) -> &dyn Any;
}