use std::os::windows::io::{AsRawHandle, BorrowedHandle};

use windows::Win32::{
    Foundation::{ERROR_INSUFFICIENT_BUFFER, ERROR_MORE_DATA, HANDLE, WIN32_ERROR},
    System::IO::DeviceIoControl,
};

pub type Result<T, E> = std::result::Result<T, Error<E>>;

pub enum Error<E: std::error::Error + TryFrom<i32>> {
    StdIo(std::io::Error),
    Driver(E),
    MoreData(usize),
}

/// Struct for keeping track of
/// [`IoControl`] operations.
pub struct Door<'a> {
    end_of_req: bool,
    handle: BorrowedHandle<'a>,
    code: u32,
}

impl<'a> Door<'a> {
    pub const fn new(handle: BorrowedHandle<'a>, code: u32) -> Self {
        Self {
            end_of_req: false,
            handle,
            code,
        }
    }

    /// Performs a call to [`DeviceIoControl`], reading from `input` and writing
    /// to `output` and using the stored handle and control code as the request.
    ///
    /// Returns the number of bytes written to `output`. If `Ok(0)` is returned,
    /// then the function is done writing data for the specific request.
    /// Users are expected to perform repeated calls to [`Door::read_write`]
    /// until receiving 0 bytes, using the same buffer for input. The output
    /// buffer should start right after where this function stopped writing to.
    pub fn read_write<E: std::error::Error + TryFrom<i32>>(
        &mut self,
        input: Option<&[u8]>,
        output: Option<&mut [u8]>,
    ) -> Result<usize, E> {
        if self.end_of_req {
            return Ok(0);
        }

        let code = self.code;
        let handle = HANDLE(self.handle.as_raw_handle() as isize);
        let input_len = input
            .as_ref()
            .map(|buf| buf.len() as u32)
            .unwrap_or_default();
        let output_len = output
            .as_ref()
            .map(|buf| buf.len() as u32)
            .unwrap_or_default();
        let mut bytes_returned: u32 = 0;

        // SAFETY: Both `input` and `output` are valid slices.
        let result = unsafe {
            DeviceIoControl(
                handle,
                code,
                input.map(|buf| buf.as_ptr().cast()),
                input_len,
                output.map(|buf| buf.as_mut_ptr().cast()),
                output_len,
                Some(core::ptr::addr_of_mut!(bytes_returned)),
                None,
            )
        };

        if let Err(err) = result {
            if let Ok(err) = E::try_from(err.code().0) {
                return Err(Error::Driver(err));
            }

            let win32_err =
                WIN32_ERROR::from_error(&err).expect("Converting error from DeviceIoControl");
            match win32_err {
                ERROR_MORE_DATA => Err(Error::MoreData(bytes_returned.try_into().unwrap())),
                ERROR_INSUFFICIENT_BUFFER => {
                    Err(Error::StdIo(std::io::ErrorKind::WriteZero.into()))
                }
                _ => Err(Error::StdIo(err.into())),
            }
        } else {
            self.end_of_req = true;
            Ok(bytes_returned.try_into().unwrap())
        }
    }
}
