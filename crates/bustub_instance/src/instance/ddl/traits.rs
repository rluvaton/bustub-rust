use std::sync::Arc;
use binder::CreateStatement;
use transaction::Transaction;
use crate::instance::db_output::StatementOutput;
use crate::result_writer::ResultWriter;

pub(crate) trait StatementHandler {
    fn create_table(&self, txn: Arc<Transaction>, stmt: &CreateStatement) -> error_utils::anyhow::Result<StatementOutput>;
    // fn create_index<ResultWriterImpl: ResultWriter>(&self, txn: Arc<Transaction>, stmt: &CreateStatement, writer: &mut ResultWriterImpl);
    // fn explain<ResultWriterImpl: ResultWriter>(&self, txn: Arc<Transaction>, stmt: &CreateStatement, writer: &mut ResultWriterImpl);
    // fn variable_show<ResultWriterImpl: ResultWriter>(&self, txn: Arc<Transaction>, stmt: &CreateStatement, writer: &mut ResultWriterImpl);
    // fn variable_set<ResultWriterImpl: ResultWriter>(&self, txn: Arc<Transaction>, stmt: &CreateStatement, writer: &mut ResultWriterImpl);
    // fn txn<ResultWriterImpl: ResultWriter>(&self, txn: Arc<Transaction>, stmt: &CreateStatement, writer: &mut ResultWriterImpl);
}


