pub struct BackupFile {
    pub path: str,
    pub size: Option<u64>,
}

/// Parameters of a backup
#[derive(Debug, Clone)]
pub struct BackupConfiguration {
    pub ignore_checksum: bool,
    pub ignore_limbo: bool,
    pub metadata_only: bool,
    pub no_garbage_collect: bool,
    pub old_description: bool,
    pub non_transportable: bool,
    pub convert: bool,
    pub expand: bool,
    pub no_triggers: bool,

    pub verbose: bool,
    pub factor: Option<u32>,
}

impl Default for BackupConfiguration {
    fn default() -> Self {
        Self {
            ignore_checksum: false,
            ignore_limbo: false,
            metadata_only: false,
            no_garbage_collect: false,
            old_description: false,
            non_transportable: false,
            convert: false,
            expand: false,

            verbose: false,
            factor: None,
            no_triggers: false,
        }
    }
}

/// Parameters of a restore
#[derive(Debug, Clone)]
pub struct RestoreConfiguration {
    pub verbose: bool,
    pub cache_buffers: u32,
    pub page_size: u32, // TODO use pageSize
    pub read_only: bool,
    pub deactivate_indexes: bool,
    pub no_shadow: bool,
    pub no_validity: bool,
    pub individual_commit: bool,

    pub replace: bool, // TODO replace by mode ?
    pub create: bool,

    pub use_all_space: bool,
    pub metadata_only: bool,
    // pub fix_fs_data: bool,
    // pub fix_fss_metadata: bool,
}

impl Default for RestoreConfiguration {
    fn default() -> Self {
        Self {
            verbose: false,
            cache_buffers: 2048,
            page_size: 4096,
            read_only: false,
            deactivate_indexes: false,
            no_shadow: false,
            no_validity: false,
            individual_commit: true,
            replace: false, // TODO replace by mode ?
            create: true,
            use_all_space: false,
            metadata_only: false,
            // fix_fs_data: ,
            // fix_fss_metadata: ,
        }
    }
}