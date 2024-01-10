use std::fmt::Display;

use measurements::{data::Data, Frequency};

use crate::{memerror::SMBiosError, smbios::SMBios};

use smbioslib::{MemorySize, MemorySizeExtended, SMBiosMemoryDevice};

type MemoryResult<T> = Result<T, SMBiosError>;

// This only works when running as root

// Type to store basic information about memory devices, such as ramsticks.
// Minimally tested, it is possible information shows up in here for empty dimm slots as well.
pub struct MemDevice<'a> {
    //pub speed: Option<Frequency>,
    //pub part_number: Option<String>,
    //pub location: String,
    //pub manufacturer: Option<String>,
    //pub size: Option<Data>,
    //pub mem_type: Option<String>,
    device: SMBiosMemoryDevice<'a>,
}

/// Wrapper for `MemoryFormFactor`
/// Done mainly for impling the display trait
#[derive(Eq, PartialEq)]
pub struct FormFactor(pub smbioslib::MemoryFormFactor);

impl Display for FormFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use smbioslib::MemoryFormFactor;
        let string = match &self.0 {
            // Most values format correctly when using debug formatting
            // Manually add cases for when format could be improved.
            MemoryFormFactor::RowOfChips => "Row of Chips".to_string(),
            MemoryFormFactor::ProprietaryCard => "Proprietary Card".to_string(),
            v => format!("{v:?}").to_uppercase(),
        };
        write!(f, "{string}")
    }
}

#[derive(Eq, PartialEq)]
pub struct MemoryType(pub smbioslib::MemoryDeviceType);

impl Display for MemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use smbioslib::MemoryDeviceType;
        let string = match &self.0 {
            MemoryDeviceType::ThreeDram => "3DRAM".to_string(),
            v => format!("{v:?}").to_uppercase(),
        };
        write!(f, "{string}")
    }
}

impl<'a> MemDevice<'a> {
    /// Returns vector of memory devices from smbios table
    ///
    /// # Errors
    ///
    /// Will return an error if parsing the smbios table fails
    pub fn from_smbios(smbios: &'a SMBios) -> MemoryResult<Option<Vec<Self>>> {
        let smb = smbios.data.defined_struct_iter::<SMBiosMemoryDevice>();
        let mut vec = Vec::new();
        for dev in smb {
            vec.push(Self { device: dev });
        }
        Ok(Some(vec))
    }

    /// Returns memory frequency
    pub fn speed(&self) -> Option<Frequency> {
        match self.device.configured_memory_speed() {
            Some(v) => match v {
                smbioslib::MemorySpeed::MTs(s) => Some(Frequency::from_megahertz(s as f64)),
                smbioslib::MemorySpeed::Unknown => None,
                smbioslib::MemorySpeed::SeeExtendedSpeed => todo!(),
            },
            _ => None,
        }
    }

    pub fn location(&self) -> Option<String> {
        self.device.device_locator().ok()
    }

    pub fn part_number(&self) -> Option<String> {
        self.device.part_number().ok().map(|s| s.trim().to_string())
    }
    pub fn manufacturer(&self) -> Option<String> {
        self.device.manufacturer().ok()
    }

    pub fn size(&self) -> Option<Data> {
        match self.device.size() {
            Some(MemorySize::Kilobytes(d)) => Some(Data::from_kibioctets(d as f64)),
            Some(MemorySize::Megabytes(d)) => Some(Data::from_mebioctets(d as f64)),
            Some(MemorySize::SeeExtendedSize) => match self.device.extended_size() {
                Some(MemorySizeExtended::Megabytes(d)) => Some(Data::from_mebioctets(d as f64)),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn mem_type(&self) -> Option<MemoryType> {
        match self.device.memory_type() {
            Some(v) => Some(MemoryType(v.value)), // TODO: This is gross
            None => None,
        }
    }

    pub fn form_factor(&self) -> Option<FormFactor> {
        match self.device.form_factor() {
            Some(v) => Some(FormFactor(v.value)),
            None => None,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::path::Path;

    use crate::smbios::SMBios;

    use super::*;
    fn get_first_mem_device(smbios: &SMBios) -> MemDevice<'_> {
        let mut mem = MemDevice::from_smbios(smbios).unwrap().unwrap();
        mem.swap_remove(0)
    }

    fn get_smbios() -> SMBios {
        let path = Path::new("./dmi.bin");
        SMBios::new_from_file(path).unwrap()
    }

    #[test]
    fn test_manufactuer() {
        let smb = get_smbios();
        let dev = get_first_mem_device(&smb);
        assert_eq!(dev.manufacturer(), Some(String::from("8C260000802C")));
    }

    #[test]
    fn test_part() {
        let smb = get_smbios();
        let dev = get_first_mem_device(&smb);
        assert_eq!(dev.part_number(), Some(String::from("TIMETEC-SD4-2666")));
    }

    #[test]
    fn test_type() {
        let smb = get_smbios();
        let dev = get_first_mem_device(&smb);
        assert_eq!(dev.mem_type().unwrap().to_string(), String::from("DDR4"));
    }

    #[test]
    fn test_capacity() {
        let smb = get_smbios();
        let dev = get_first_mem_device(&smb);
        assert_eq!(dev.size(), Some(Data::from_gibioctets(32.00)));
    }

    #[test]
    fn test_freq() {
        let smb = get_smbios();
        let dev = get_first_mem_device(&smb);
        assert_eq!(dev.speed(), Some(Frequency::from_megahertz(2667.00)));
    }

    #[test]
    fn test_form_factor() {
        let smb = get_smbios();
        let dev = get_first_mem_device(&smb);
        assert_eq!(
            dev.form_factor().unwrap().to_string(),
            String::from("SODIMM")
        );
    }
}
