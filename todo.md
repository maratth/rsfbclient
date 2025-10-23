src/connection/mod.rs

```rs
impl<C> BackupRestore for SvcConnection<C>
where
C: FirebirdSvcClient,
{
fn backup(
&mut self,
db_name: &str,
backup_files: Vec<&BackupFile>,
backup_configuration: &BackupConfiguration
) -> Result<(), FbError> {
self.cli.backup(&mut self.handle, db_name, backup_files, backup_configuration)?;

        Ok(())
    }

    fn restore(&mut self, backup_files: [&str], db_name: &str, restore_configuration: RestoreConfiguration) -> Result<(), FbError> {
        self.cli.restore();
    }
}
```

code/src/connection.rs

```rs
/// Responsible for service backup/restore
pub trait FirebirdClientSvcBackupRestoreOps {
/// A service handle
type SvcHandle: Send;

    fn backup(
        &mut self,
        svc_handle: &mut Self::SvcHandle,
        db_name: &str,
        backup_files: Vec<&BackupFile>,
        config: &BackupConfiguration
    ) -> Result<(), FbError>;
    fn restore(
        &mut self,
        svc_handle: &mut Self::SvcHandle,
        config: &RestoreConfiguration
    ) -> Result<(), FbError>;
}
```