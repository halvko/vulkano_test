use std::error::Error;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    device::{Device, DeviceExtensions, Features},
    instance::{Instance, InstanceExtensions, PhysicalDevice},
};

fn main() -> Result<(), Box<dyn Error>> {
    let instance = Instance::new(None, &InstanceExtensions::none(), None)?;
    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("No devices");
    for family in physical.queue_families() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queues_count()
        );
    }
    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("No graphical queue family");

    let (device, mut queues) = {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned(),
        )?
    };

    let queue = queues.next().unwrap(); // I'm not sure if this will never panic

    let data = 12;
    let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, data);

    Ok(())
}
