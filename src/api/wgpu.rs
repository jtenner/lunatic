use std::{convert::TryInto, future::Future, path::Path, sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use wasmtime::{Caller, FuncType, Linker, Trap, Val, ValType};

use super::{
    get_memory, link_async1_if_match, link_async2_if_match, link_async3_if_match, link_async4_if_match,
    link_async5_if_match, link_async6_if_match, link_async7_if_match, link_if_match,
};
use crate::{
    api::error::IntoTrap,
    module::Module,
    process::{Signal, WasmProcess},
    state::ProcessState,
    EnvConfig, Environment, Process,
};

// Register the process APIs to the linker
pub(crate) fn register(
    linker: &mut Linker<ProcessState>,
    namespace_filter: &[String],
) -> Result<()> {
    link_if_match(
        linker,
        "lunatic::wgpu",
        "new_instance",
        FuncType::new([], [ValType::I64]),
        new_instance,
        namespace_filter,
    )?;
    link_async2_if_match(
        linker,
        "lunatic::process",
        "add_module",
        FuncType::new([ValType::I64, ValType::I32], [ValType::I32]),
        request_adapter,
        namespace_filter,
    )?;
    link_async3_if_match(
        linker,
        "lunatic::wgpu",
        "request_device",
        FuncType::new([ValType::I64, ValType::I32, ValType::I32], [ValType::I32]),
        request_device,
        namespace_filter,
    )?;
    Ok(())
}


//% lunatic::wgpu::new_instance() -> u64
//%
//% * Returns ID of newly created Instance.
//%
//%  Create a wgpu_instance and return it's ID.
fn new_instance(mut caller: Caller<ProcessState>) -> u64 {
    caller
        .data_mut()
        .resources
        .wgpu_instances
        .add(wgpu::Instance::new(wgpu::Backends::all()))
}

fn request_adapter(
    mut caller: Caller<ProcessState>,
    instance_id: u64,
    id_ptr: u32,
) -> Box<dyn Future<Output = Result<u32, Trap>> + Send + '_>  {
    Box::new(async move {
        let memory = get_memory(&mut caller)?;
        let instance = caller
            .data_mut()
            .resources
            .wgpu_instances
            .get(instance_id)
            .or_trap("")?;

        let (adapter_or_error_id, return_) = match instance
            .request_adapter(
                &wgpu::RequestAdapterOptions::default()
            ).await {
                Some(adapter) => {
                    (caller.data_mut().resources.wgpu_adapters.add(adapter), 0)
                },
                None => (caller.data_mut().errors.add(anyhow!("No adapter could be requested.")), 1)
            };

        memory
            .write(&mut caller, id_ptr as usize, &adapter_or_error_id.to_le_bytes())
            .or_trap("lunatic::wgpu::request_adapter")?;

        Ok(return_)
    })
}


fn request_device(
    mut caller: Caller<ProcessState>,
    adapter_id: u64,
    device_id_ptr: u32,
    queue_id_ptr: u32
) -> Box<dyn Future<Output = Result<u32, Trap>> + Send + '_>  {
    Box::new(async move {
        let memory = get_memory(&mut caller)?;
        let adapter = caller
            .data_mut()
            .resources
            .wgpu_adapters
            .get(adapter_id)
            .or_trap("")?;

        let (device_id, queue_id, return_) = match adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_defaults(),
                },
                None,
            )
            .await {
                Ok((device, queue)) => (
                    caller.data_mut().resources.wgpu_devices.add(device),
                    caller.data_mut().resources.wgpu_queues.add(queue),
                    1,
                ),
                Err(err) => (caller.data_mut().errors.add(anyhow!(err)), 0, 1)
            };


        memory
            .write(&mut caller, device_id_ptr as usize, &device_id.to_le_bytes())
            .or_trap("lunatic::wgpu::request_device")?;

        memory
            .write(&mut caller, queue_id_ptr as usize, &queue_id.to_le_bytes())
            .or_trap("lunatic::wgpu::request_device")?;

        Ok(return_)
    })
}

