//!
//! Rust Firebird Client
//!
//! Service backand and restore
//!

use rsfbclient_core::{BackupConfiguration, BackupFile, FbError};

/// Implemented for types that can be used to perform backup/restore
pub trait BackupRestore {

    fn backup(
        &mut self,
        db_name: &str,
        backup_files: Vec<BackupFile>,
        backup_configuration: BackupConfiguration,
    ) -> Result<(), FbError>;

    fn restore(
        &mut self,
        backup_files: [&str],
        db_name: &str,
        restore_configuration: RestoreConfiguration,
    ) -> Result<(), FbError>;

}