use bytes::{buf, BytesMut};
use std::{
    convert::TryInto,
    future::Future,
    io::{Read, Write},
    sync::{ Arc, Mutex },
};

use anyhow::{anyhow, Result};
use lunatic_common_api::{get_memory, IntoTrap};

use tokio::time::{timeout, Duration};
use wasmtime::{Caller, Linker};

use hash_map_id::HashMapId;
use lunatic_process::{state::ProcessState, Signal};

#[derive(Debug, Default)]
pub struct Buffer {
    pub bytes: Vec<u8>,
    pub read_ptr: usize,
}

pub type BufferResource = HashMapId<Arc<Mutex<Buffer>>>;

pub trait BufferCtx {
    fn buffer_resources(&self) -> &BufferResource;
    fn buffer_resources_mut(&mut self) -> &mut BufferResource;
}

impl Buffer {
    fn seek(&mut self, index: usize) {
        self.read_ptr = (self.bytes.len() - 1).min(index);
    }
}

// Register the mailbox APIs to the linker
pub fn register<T: ProcessState + BufferCtx + Send + 'static>(
    linker: &mut Linker<T>,
) -> Result<()> {
    linker.func_wrap("lunatic::buffer", "create", create)?;
    linker.func_wrap("lunatic::buffer", "read_data", read_data)?;
    linker.func_wrap("lunatic::buffer", "seek_data", seek_data)?;
    Ok(())
}

fn create<T: BufferCtx>(mut caller: Caller<T>, capacity: u32) -> Result<u64> {
    let buffer = Buffer::default();

    let data = caller.data_mut().buffer_resources_mut();
    Ok(data.add(Arc::new(buffer)))
}

fn read_data<T: BufferCtx>(mut caller: Caller<T>, id: u64, ptr: u32, size: u32) -> Result<u32> {
    let memory = get_memory(&mut caller)?;
    let buffer = caller
        .data_mut()
        .buffer_resources_mut()
        .get_mut(id)
        .or_trap("lunatic::buffer::read::get_buffer")?
        .clone();

    let read_ptr = buffer.read_ptr;
    let min_read = size.min((buffer.bytes.len() - read_ptr) as u32);
    let bytes = buffer
        .lock()
        .or_trap("lunatic::buffer::read::write_memory")?
        .bytes
        .get(read_ptr..(read_ptr + min_read as usize))
        .or_trap("lunatic::buffer::read::write_memory")?;

    memory
        .write(&mut caller, ptr as usize, bytes)
        .or_trap("lunatic::buffer::read::write_memory")?;
    Ok(min_read)
}

fn seek_data<T: BufferCtx>(mut caller: Caller<T>, id: u64, index: u32) -> Result<()> {
  let mut buffer = &mut caller
    .data_mut()
    .buffer_resources_mut()
    .get(id)
    .or_trap("lunatic::buffer::seek_data::get_buffer")?
    .clone();

  buffer.seek(index as usize);
  Ok(())
}
