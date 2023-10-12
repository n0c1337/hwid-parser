use std::ffi::{CString};
use raw_cpuid::{CpuId};
use winapi::um::fileapi::GetVolumeInformationA;
use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
use winapi::um::winbase::{GetComputerNameA, GetUserNameA};

pub struct HardwareID;
pub struct GraphicsProcessingUnit;

impl HardwareID {
    pub fn new() -> Self {
        HardwareID {}
    }

    pub fn get_username(&self) -> Option<String> {
        let mut buffer: [u8; 256] = [0; 256];
        let mut size: u32 = buffer.len() as u32;
        unsafe {
            if GetUserNameA(buffer.as_mut_ptr() as *mut i8, &mut size) != 0 {
                let username = CString::from_raw(buffer.as_mut_ptr() as *mut i8).into_string().unwrap();
                return Some(username.to_string())
            }
        }
        None
    }

    pub fn get_computername(&self) -> Option<String> {
        let mut buffer: [u8; 256] = [0; 256];
        let mut size: u32 = buffer.len() as u32;
        unsafe {
            if GetComputerNameA(buffer.as_mut_ptr() as *mut i8, &mut size) != 0 {
                let computername = CString::from_raw(buffer.as_mut_ptr() as *mut i8).into_string().unwrap();
                return Some(computername.to_string())
            }
        }
        None
    }

    pub fn get_volume_information(&self, volume: &str) -> Option<u32> {
        let c_string = CString::new(volume).expect("Couldn't create string");
        let mut serial_number = 1;
        unsafe {
            if GetVolumeInformationA(
                c_string.as_ptr(),
                std::ptr::null_mut(),
                0,
                &mut serial_number,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                0,
            ) != 0 {
                Some(serial_number)
            } else {
                None
            }
        }
    }

    pub fn get_cpu_info(&self) -> Option<SYSTEM_INFO> {
        unsafe {
            let mut system_info: SYSTEM_INFO = std::mem::zeroed();
            GetSystemInfo(&mut system_info);
            Some(system_info)
        }
    }


    pub fn get_extended_cpu_info(&self) -> Option<String> {
        let cpuid = CpuId::new();
        let brand_string = cpuid
            .get_processor_brand_string()
            .as_ref()
            .map_or_else(|| "n/a", |pbs| pbs.as_str()).to_string();

        Some(brand_string)
    }

    pub fn get_gpu(&self) -> GraphicsProcessingUnit {
        GraphicsProcessingUnit{}
    }
}

impl GraphicsProcessingUnit {
    pub fn get_gpu_model(&self) -> Option<String> {
        let command = std::process::Command::new("cmd")
            .arg("/C")
            .arg("wmic path win32_VideoController get name")
            .output().unwrap();
        let output = String::from_utf8(command.stdout).unwrap();
        return match output
            .lines()
            .find(|l| !l.contains("Name"))
        {
            None => {
                None
            }
            Some(id) => {
                Some(id.to_string())
            }
        }
    }

    pub fn get_gpu_hardware_id(&self) -> Option<String> {
        let command = std::process::Command::new("cmd")
            .arg("/C")
            .arg("wmic path win32_VideoController get PNPDeviceID")
            .output().unwrap();
        let output = String::from_utf8(command.stdout).unwrap();
        return match output
            .lines()
            .find(|l| l.contains("PCI"))
        {
            None => {
                None
            }
            Some(id) => {
                Some(id.to_string())
            }
        }
    }
}