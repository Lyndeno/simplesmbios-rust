use std::path::Path;

use smbioslib::{load_smbios_data_from_file, table_load_from_device, SMBiosData};

use crate::memerror::SMBiosError;

type SMBiosResult<T> = Result<T, SMBiosError>;

pub struct SMBios {
    pub data: SMBiosData,
}

impl SMBios {
    pub fn new_from_device() -> SMBiosResult<Self> {
        Ok(Self {
            data: table_load_from_device()?,
        })
    }

    pub fn new_from_file(path: &Path) -> SMBiosResult<Self> {
        Ok(Self {
            data: load_smbios_data_from_file(path)?,
        })
    }

    pub fn new_from_data(data: SMBiosData) -> Self {
        Self { data }
    }
}
