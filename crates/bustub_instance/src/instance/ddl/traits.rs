use std::sync::Arc;
use binder::{CreateStatement, DropTableStatement};
use transaction::Transaction;
use crate::result_writer::ResultWriter;

pub(crate) trait StatementHandler {
    fn create_table<ResultWriterImpl: ResultWriter>(&self, txn: Arc<Transaction>, stmt: &CreateStatement, writer: &mut ResultWriterImpl);
    fn drop_table<ResultWriterImpl: ResultWriter>(&self, txn: Arc<Transaction>, stmt: &DropTableStatement, writer: &mut ResultWriterImpl);

    // fn create_index<ResultWriterImpl: ResultWriter>(&self, txn: Arc<Transaction>, stmt: &CreateStatement, writer: &mut ResultWriterImpl);
    // fn explain<ResultWriterImpl: ResultWriter>(&self, txn: Arc<Transaction>, stmt: &CreateStatement, writer: &mut ResultWriterImpl);
    // fn variable_show<ResultWriterImpl: ResultWriter>(&self, txn: Arc<Transaction>, stmt: &CreateStatement, writer: &mut ResultWriterImpl);
    // fn variable_set<ResultWriterImpl: ResultWriter>(&self, txn: Arc<Transaction>, stmt: &CreateStatement, writer: &mut ResultWriterImpl);
    // fn txn<ResultWriterImpl: ResultWriter>(&self, txn: Arc<Transaction>, stmt: &CreateStatement, writer: &mut ResultWriterImpl);
}


