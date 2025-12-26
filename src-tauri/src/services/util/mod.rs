use machineid_rs::{Encryption, HWIDComponent, IdBuilder};

pub mod file;

///获取硬件ID https://crates.io/crates/machineid-rs
pub fn get_hardware_id() -> String {
    let mut builder = IdBuilder::new(Encryption::SHA256);
    builder.add_component(HWIDComponent::SystemID).add_component(HWIDComponent::CPUCores);
    let hwid = builder.build("mykey").unwrap();
    hwid
}
