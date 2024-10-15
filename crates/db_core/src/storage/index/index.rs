use std::fmt::{Debug, Formatter};
use crate::storage::{IndexMetadata};
use std::sync::Arc;
use catalog_schema::Schema;
use rid::RID;
use transaction::Transaction;
use tuple::Tuple;

/// class Index - Base class for derived indices of different types
///
/// The index structure majorly maintains information on the schema of the
/// underlying table and the mapping relation between index key
/// and tuple key, and provides an abstracted way for the external world to
/// interact with the underlying index implementation without exposing
/// the actual implementation's interface.
///
/// Index object also handles predicate scan, in addition to simple insert,
/// delete, predicate insert, point query, and full index scan. Predicate scan
/// only supports conjunction, and may or may not be optimized depending on
/// the type of expressions inside the predicate.
///
// TODO - this should be a trait and not a struct
pub struct Index {
    /// The Index structure owns its metadata
    metadata: Arc<IndexMetadata>,
}

impl Index {
    pub fn new(metadata: Arc<IndexMetadata>) -> Self {
        Self {
            metadata,
        }
    }


    /// A non-owning pointer to the metadata object associated with the index
    pub fn get_metadata(&self) -> Arc<IndexMetadata> {
        self.metadata.clone()
    }

    /// @return The number of indexed columns
    pub fn get_index_column_count(&self) -> u32 {
        self.metadata.get_index_column_count()
    }

    /// @return The index name
    pub fn get_name(&self) -> &str {
        self.metadata.get_name()
    }

    /// @return The index key schema
    pub fn get_key_schema(&self) -> Arc<Schema> {
        self.metadata.get_key_schema()
    }

    /// @return The index key attributes
    pub fn get_key_attrs(&self) -> &Vec<u32> {
        self.metadata.get_key_attrs()
    }

    ///////////////////////////////////////////////////////////////////
    // Point Modification
    ///////////////////////////////////////////////////////////////////

    /// Insert an entry into the index.
    ///
    /// # Arguments
    /// - `key` The index key
    /// - `rid` The RID associated with the key
    /// - `transaction` The transaction context
    ///
    /// returns `bool`: whether insertion is successful
    pub fn insert_entry(&mut self, _key: &Tuple, _rid: RID, _transaction: &Transaction) -> bool {
        unimplemented!()
    }

    /// Delete an index entry by key.
    ///
    /// # Arguments
    /// - `key` The index key
    /// - `rid` The RID associated with the key (unused)
    /// - `transaction` The transaction context
    ///
    pub fn delete_entry(&mut self, _key: &Tuple, _rid: RID, _transaction: &Transaction) {
        unimplemented!()
    }

    /// Search the index for the provided key.
    ///
    /// # Arguments
    /// - `key` The index key
    /// - `transaction` The transaction context
    ///
    /// returns `Vec<RID>`: The collection of RIDs with the search results
    pub fn scan_key(&self, _key: &Tuple, _transaction: &Transaction) -> Vec<RID> {
        unimplemented!()
    }
}


impl Debug for Index {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "INDEX: ({}) {:?}", self.get_name(), self.metadata)
    }
}
