use crate::hwid::hwid::HardwareID;

mod hwid;

fn main() {
    let hardware_id = HardwareID::new();
    println!("Username: {}", hardware_id.get_username().unwrap());
    println!("Computername: {}", hardware_id.get_computername().unwrap());
    //println!("CPU Type: {}", hardware_id.get_cpu_info().unwrap().dwProcessorType);
    println!("CPU Model: {}", hardware_id.get_extended_cpu_info().unwrap());
    let gpu = hardware_id.get_gpu();
    println!("GPU Model: {}", gpu.get_gpu_model().expect("failed gathering gpu model information"));
    // https://crates.io/crates/sha256
    println!("GPU ID: {}", gpu.get_gpu_hardware_id().expect("failed gathering gpu id information"));
    println!("GPU ID SHA256: {}", sha256::digest(gpu.get_gpu_hardware_id().expect("failed gathering gpu id information")));
    println!("C:\\ Diskdrive Serial Number: {}", hardware_id.get_volume_information("C:\\").unwrap())
}
