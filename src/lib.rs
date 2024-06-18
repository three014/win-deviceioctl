mod bincode_utils;
mod door;
mod driver_utils;

pub use bincode_utils::*;
use door::Door;
pub use driver_utils::*;
use std::os::windows::io::BorrowedHandle;

const STACK_SIZE: usize = 8192;

pub trait Recv {
    type Output;
    fn buf_starting_capacity(&self) -> Option<usize>;
    fn recv(bytes: &[u8]) -> DecResult<Self::Output>;
}

pub trait Send {
    fn send<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> EncResult;
}

pub trait CtrlCode {
    const CODE: ControlCode;
}

pub enum Error<E: std::error::Error + TryFrom<i32>> {
    Encode(bincode::error::EncodeError),
    Decode(bincode::error::DecodeError),
    Driver(E),
    Io(std::io::Error),
}

impl<E: std::error::Error + TryFrom<i32>> From<bincode::error::EncodeError> for Error<E> {
    fn from(value: bincode::error::EncodeError) -> Self {
        Self::Encode(value)
    }
}

impl<E: std::error::Error + TryFrom<i32>> From<bincode::error::DecodeError> for Error<E> {
    fn from(value: bincode::error::DecodeError) -> Self {
        Self::Decode(value)
    }
}

struct EncodeAdapter<'a, T: Send>(&'a T);
impl<T: Send> bincode::Encode for EncodeAdapter<'_, T> {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        self.0.send(encoder)
    }
}

pub fn send_recv<T, E>(handle: BorrowedHandle, ioctl: T) -> Result<T::Output, Error<E>>
where
    T: Send + Recv + CtrlCode,
    E: std::error::Error + TryFrom<i32>,
{
    let mut door = Door::new(handle, T::CODE.into_u32());
    let input = bincode::encode_to_vec(EncodeAdapter(&ioctl), bincode_config())?;

    let mut stack_buf = [0u8; STACK_SIZE];
    let mut start = 0;

    let extra_cap = match ioctl.buf_starting_capacity() {
        Some(cap) if cap > STACK_SIZE => Some(cap),
        _ => match door.read_write::<E>(Some(&input), Some(&mut stack_buf)) {
            Ok(bytes_read) => {
                start += bytes_read;
                // We did it!! No heap allocations here! :D
                None
            }
            Err(door::Error::MoreData(bytes_read)) => {
                start += bytes_read;
                // We need more storage! To the heap!
                Some(bytes_read * 2)
            }
            Err(door::Error::StdIo(err)) => Err(Error::Io(err))?,
            Err(door::Error::Driver(err)) => Err(Error::Driver(err))?,
        },
    };

    let mut heap_buf = Vec::new();
    let slice = if let Some(extra_cap) = extra_cap {
        // Now we can allocate some space to copy the old buffer
        heap_buf.resize(extra_cap, 0);
        heap_buf[..start].copy_from_slice(&stack_buf[..start]);

        loop {
            match door.read_write::<E>(Some(&input), Some(&mut heap_buf[start..])) {
                Ok(bytes_read) => {
                    start += bytes_read;
                    break &heap_buf[..start];
                }
                Err(door::Error::MoreData(bytes_read)) => {
                    start += bytes_read;
                    let new_len = heap_buf
                        .len()
                        .checked_mul(2)
                        .ok_or_else(|| Error::Io(std::io::ErrorKind::OutOfMemory.into()))?;
                    heap_buf.resize(new_len, 0);
                }
                Err(door::Error::StdIo(err)) => Err(Error::Io(err))?,
                Err(door::Error::Driver(err)) => Err(Error::Driver(err))?,
            }
        }
    } else {
        &stack_buf[..start]
    };

    T::recv(&slice).map_err(Error::Decode)
}

pub fn send<T, E>(handle: BorrowedHandle, ioctl: T) -> Result<(), Error<E>>
where
    T: Send + CtrlCode,
    E: std::error::Error + TryFrom<i32>,
{
    let mut door = Door::new(handle, T::CODE.into_u32());
    let input = bincode::encode_to_vec(EncodeAdapter(&ioctl), bincode_config())?;

    door.read_write::<E>(Some(&input), None)
        .map_err(|err| match err {
            door::Error::StdIo(err) => Error::Io(err),
            door::Error::Driver(err) => Error::Driver(err),
            _ => unreachable!(),
        })?;

    Ok(())
}

pub fn recv<T, E>(handle: BorrowedHandle, ioctl: T) -> Result<T::Output, Error<E>>
where
    T: Recv + CtrlCode,
    E: std::error::Error + TryFrom<i32>,
{
    let mut door = Door::new(handle, T::CODE.into_u32());

    let mut stack_buf = [0u8; STACK_SIZE];
    let mut start = 0;

    let extra_cap = match ioctl.buf_starting_capacity() {
        Some(cap) if cap > STACK_SIZE => Some(cap),
        _ => match door.read_write::<E>(None, Some(&mut stack_buf)) {
            Ok(bytes_read) => {
                start += bytes_read;
                // We did it!! No heap allocations here! :D
                None
            }
            Err(door::Error::MoreData(bytes_read)) => {
                start += bytes_read;
                // We need more storage! To the heap!
                Some(bytes_read * 2)
            }
            Err(door::Error::StdIo(err)) => Err(Error::Io(err))?,
            Err(door::Error::Driver(err)) => Err(Error::Driver(err))?,
        },
    };

    let mut heap_buf = Vec::new();
    let slice = if let Some(extra_cap) = extra_cap {
        // Now we can allocate some space to copy the old buffer
        heap_buf.resize(extra_cap, 0);
        heap_buf[..start].copy_from_slice(&stack_buf[..start]);

        loop {
            match door.read_write::<E>(None, Some(&mut heap_buf[start..])) {
                Ok(bytes_read) => {
                    start += bytes_read;
                    break &heap_buf[..start];
                }
                Err(door::Error::MoreData(bytes_read)) => {
                    start += bytes_read;
                    let new_len = heap_buf
                        .len()
                        .checked_mul(2)
                        .ok_or_else(|| Error::Io(std::io::ErrorKind::OutOfMemory.into()))?;
                    heap_buf.resize(new_len, 0);
                }
                Err(door::Error::StdIo(err)) => Err(Error::Io(err))?,
                Err(door::Error::Driver(err)) => Err(Error::Driver(err))?,
            }
        }
    } else {
        &stack_buf[..start]
    };

    T::recv(&slice).map_err(Error::Decode)
}
