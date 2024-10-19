use common::config::{TxnId, TXN_START_ID};
use transaction::TransactionManager;
use crate::BustubInstance;
use crate::result_writer::ResultWriter;

impl BustubInstance {
   pub fn cmd_dbg_mvcc<ResultWriterImpl: ResultWriter>(&self, params: Vec<&str>, writer: &mut ResultWriterImpl) {
       if params.len() != 2 {
           writer.one_cell("please provide a table name");

           return;
       }

       let table = params[1];
       writer.one_cell(format!("please view the result in the BusTub console (or Chrome DevTools console), table={}", table).as_str());

       let catalog = self.catalog.lock();
       if let Some(table_info) = catalog.get_table_by_name(&table) {
           self.txn_manager.debug("\\dbgmvcc".to_string(), Some(table_info.clone()), Some(table_info.get_table_heap()))
       } else {
           writer.one_cell(format!("table {} not found", table).as_str());
       }
   }

    pub fn cmd_display_tables<ResultWriterImpl: ResultWriter>(&self, writer: &mut ResultWriterImpl) {
        let catalog = self.catalog.lock();
        let table_names = catalog.get_table_names();

        writer.begin_table(false);

        writer.begin_header();
        writer.write_header_cell("oid");
        writer.write_header_cell("name");
        writer.write_header_cell("cols");
        writer.end_header();

        for name in &table_names {
            let table_info = catalog.get_table_by_name(name).expect("Table must exists");

            writer.begin_row();

            writer.write_cell(table_info.get_oid().to_string().as_str());
            writer.write_cell(table_info.get_name());
            writer.write_cell(format!("{}", table_info.get_schema()).as_str());

            writer.end_row();
        }

        writer.end_table();
    }

    pub fn cmd_display_indices<ResultWriterImpl: ResultWriter>(&self, writer: &mut ResultWriterImpl) {
        let catalog = self.catalog.lock();
        let table_names = catalog.get_table_names();

        writer.begin_table(false);

        writer.begin_header();
        writer.write_header_cell("table_name");
        writer.write_header_cell("index_oid");
        writer.write_header_cell("index_name");
        writer.write_header_cell("index_cols");
        writer.end_header();

        for table_name in &table_names {
            for index_info in catalog.get_table_indexes_by_name(table_name) {
                writer.begin_row();

                writer.write_cell(table_name);
                writer.write_cell(index_info.get_index_oid().to_string().as_str());
                writer.write_cell(index_info.get_name());
                writer.write_cell(format!("{}", index_info.get_key_schema()).as_str());

                writer.end_row();
            }
        }

        writer.end_table();
    }

    pub fn cmd_display_help<ResultWriterImpl: ResultWriter>(writer: &mut ResultWriterImpl) {
        writer.one_cell(r"(Welcome to the BusTub shell!

\dt: show all tables
\di: show all indices
\dbgmvcc <table>: show version chain of a table
\help: show this message again
\txn: show current txn information
\txn <txn_id>: switch to txn
\txn gc: run garbage collection
\txn -1: exit txn mode

BusTub shell currently only supports a small set of Postgres queries. We'll set
up a doc describing the current status later. It will silently ignore some parts
of the query, so it's normal that you'll get a wrong result when executing
unsupported SQL queries. This shell will be able to run `create table` only
after you have completed the buffer pool manager. It will be able to execute SQL
queries after you have implemented necessary query executors. Use `explain` to
see the execution plan of your query.
)")
    }

    pub fn cmd_txn<ResultWriterImpl: ResultWriter>(&mut self, params: Vec<&str>, writer: &mut ResultWriterImpl) {
        if !self.managed_txn_mode {
            writer.one_cell("only supported in managed mode, please use bustub-shell");

            return;
        }

        if params.len() == 1 {
            match self.current_txn {
                Some(_) => self.dump_current_txn(writer, ""),
                None => writer.one_cell("no active txn, each statement starts a new txn."),
            }

            return;
        }

        if params.len() == 2 {
            let param1 = &params[0];

            if param1 == &"gc".to_string() {
                self.txn_manager.garbage_collection();
                writer.one_cell("GC complete");

                return;
            }

            let txn_id = param1.parse::<TxnId>().expect("param1 must be a transaction id");

            if txn_id == -1 {
                self.dump_current_txn(writer, "pause current txn ");

                // Remove current transaction
                self.current_txn = None;

                return;
            }

            let transaction = self.txn_manager
                .get_transaction_by_id(txn_id)
                .or_else(|| self.txn_manager.get_transaction_by_id(txn_id + TXN_START_ID));

            if let Some(transaction) = transaction {
                self.current_txn = Some(transaction);
                self.dump_current_txn(writer, "switch to new txn ");
            } else {
                writer.one_cell("cannot find txn.");
            }

            return;
        }

        writer.one_cell("unsupported txn cmd.");
    }
}
