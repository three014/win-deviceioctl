//! I DID NOT COME UP WITH THE DEVICE TYPES AND
//! CONTROL CODES AND TRANSFER METHOD TYPES
//!
//! I CANNOT REMEMBER WHO WROTE THIS ALL OUT BUT
//! I PROMISE I'LL CREDIT THEM BECAUSE THEY WERE
//! EXTREMELY HELPFUL IN MAKING THE WINDOWS TYPES
//! EASIER TO USE

use bitflags::bitflags;
use windows::Win32::{
    Storage::FileSystem::{FILE_READ_DATA, FILE_WRITE_DATA},
    System::Ioctl::{
        FILE_ANY_ACCESS, METHOD_BUFFERED, METHOD_IN_DIRECT, METHOD_NEITHER, METHOD_OUT_DIRECT,
    },
};

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeviceType {
    Port8042,
    Acpi,
    Battery,
    Beep,
    BusExtender,
    Cdrom,
    CdromFileSystem,
    Changer,
    Controller,
    DataLink,
    Dfs,
    DfsFileSystem,
    DfsVolume,
    Disk,
    DiskFileSystem,
    Dvd,
    FileSystem,
    Fips,
    FullscreenVideo,
    InportPort,
    Keyboard,
    Ks,
    Ksec,
    Mailslot,
    MassStorage,
    MidiIn,
    MidiOut,
    Modem,
    Mouse,
    MultiUncProvider,
    NamedPipe,
    Network,
    NetworkBrowser,
    NetworkFileSystem,
    NetworkRedirector,
    Null,
    ParallelPort,
    PhysicalNetcard,
    Printer,
    Scanner,
    Screen,
    Serenum,
    SerialPort,
    SerialMousePort,
    Smartcard,
    Smb,
    Sound,
    Streams,
    Tape,
    TapeFileSystem,
    Termsrv,
    Transport,
    Unknown,
    Vdm,
    Video,
    VirtualDisk,
    WaveIn,
    WaveOut,
}

impl DeviceType {
    pub const fn into_u32(self) -> u32 {
        use windows::Win32::System::Ioctl::*;
        match self {
            DeviceType::Port8042 => FILE_DEVICE_8042_PORT,
            DeviceType::Acpi => FILE_DEVICE_ACPI,
            DeviceType::Battery => FILE_DEVICE_BATTERY,
            DeviceType::Beep => FILE_DEVICE_BEEP,
            DeviceType::BusExtender => FILE_DEVICE_BUS_EXTENDER,
            //DeviceType::Cdrom => FILE_DEVICE_CD_ROM,
            DeviceType::CdromFileSystem => FILE_DEVICE_CD_ROM_FILE_SYSTEM,
            DeviceType::Changer => FILE_DEVICE_CHANGER,
            DeviceType::Controller => FILE_DEVICE_CONTROLLER,
            DeviceType::DataLink => FILE_DEVICE_DATALINK,
            DeviceType::Dfs => FILE_DEVICE_DFS,
            DeviceType::DfsFileSystem => FILE_DEVICE_DFS_FILE_SYSTEM,
            DeviceType::DfsVolume => FILE_DEVICE_DFS_VOLUME,
            //DeviceType::Disk => FILE_DEVICE_DISK,
            DeviceType::DiskFileSystem => FILE_DEVICE_DISK_FILE_SYSTEM,
            //DeviceType::Dvd => FILE_DEVICE_DVD,
            DeviceType::FileSystem => FILE_DEVICE_FILE_SYSTEM,
            DeviceType::Fips => FILE_DEVICE_FIPS,
            DeviceType::FullscreenVideo => FILE_DEVICE_FULLSCREEN_VIDEO,
            DeviceType::InportPort => FILE_DEVICE_INPORT_PORT,
            DeviceType::Keyboard => FILE_DEVICE_KEYBOARD,
            DeviceType::Ks => FILE_DEVICE_KS,
            DeviceType::Ksec => FILE_DEVICE_KSEC,
            DeviceType::Mailslot => FILE_DEVICE_MAILSLOT,
            DeviceType::MassStorage => FILE_DEVICE_MASS_STORAGE,
            DeviceType::MidiIn => FILE_DEVICE_MIDI_IN,
            DeviceType::MidiOut => FILE_DEVICE_MIDI_OUT,
            DeviceType::Modem => FILE_DEVICE_MODEM,
            DeviceType::Mouse => FILE_DEVICE_MOUSE,
            DeviceType::MultiUncProvider => FILE_DEVICE_MULTI_UNC_PROVIDER,
            DeviceType::NamedPipe => FILE_DEVICE_NAMED_PIPE,
            DeviceType::Network => FILE_DEVICE_NETWORK,
            DeviceType::NetworkBrowser => FILE_DEVICE_NETWORK_BROWSER,
            DeviceType::NetworkFileSystem => FILE_DEVICE_NETWORK_FILE_SYSTEM,
            DeviceType::NetworkRedirector => FILE_DEVICE_NETWORK_REDIRECTOR,
            DeviceType::Null => FILE_DEVICE_NULL,
            DeviceType::ParallelPort => FILE_DEVICE_PARALLEL_PORT,
            DeviceType::PhysicalNetcard => FILE_DEVICE_PHYSICAL_NETCARD,
            DeviceType::Printer => FILE_DEVICE_PRINTER,
            DeviceType::Scanner => FILE_DEVICE_SCANNER,
            DeviceType::Screen => FILE_DEVICE_SCREEN,
            DeviceType::Serenum => FILE_DEVICE_SERENUM,
            DeviceType::SerialMousePort => FILE_DEVICE_SERIAL_MOUSE_PORT,
            DeviceType::SerialPort => FILE_DEVICE_SERIAL_PORT,
            //DeviceType::Smartcard => FILE_DEVICE_SMARTCARD,
            DeviceType::Smb => FILE_DEVICE_SMB,
            DeviceType::Sound => FILE_DEVICE_SOUND,
            DeviceType::Streams => FILE_DEVICE_STREAMS,
            //DeviceType::Tape => FILE_DEVICE_TAPE,
            DeviceType::TapeFileSystem => FILE_DEVICE_TAPE_FILE_SYSTEM,
            DeviceType::Termsrv => FILE_DEVICE_TERMSRV,
            DeviceType::Transport => FILE_DEVICE_TRANSPORT,
            DeviceType::Unknown => FILE_DEVICE_UNKNOWN,
            DeviceType::Vdm => FILE_DEVICE_VDM,
            DeviceType::Video => FILE_DEVICE_VIDEO,
            DeviceType::VirtualDisk => FILE_DEVICE_VIRTUAL_DISK,
            DeviceType::WaveIn => FILE_DEVICE_WAVE_IN,
            DeviceType::WaveOut => FILE_DEVICE_WAVE_OUT,
            _ => unimplemented!(),
        }
    }

    pub const fn from_u32(value: u32) -> Self {
        use windows::Win32::System::Ioctl::*;
        match value {
            FILE_DEVICE_8042_PORT => DeviceType::Port8042,
            FILE_DEVICE_ACPI => DeviceType::Acpi,
            FILE_DEVICE_BATTERY => DeviceType::Battery,
            FILE_DEVICE_BEEP => DeviceType::Beep,
            FILE_DEVICE_BUS_EXTENDER => DeviceType::BusExtender,
            //FILE_DEVICE_CD_ROM => DeviceType::Cdrom,
            FILE_DEVICE_CD_ROM_FILE_SYSTEM => DeviceType::CdromFileSystem,
            FILE_DEVICE_CHANGER => DeviceType::Changer,
            FILE_DEVICE_CONTROLLER => DeviceType::Controller,
            FILE_DEVICE_DATALINK => DeviceType::DataLink,
            FILE_DEVICE_DFS => DeviceType::Dfs,
            FILE_DEVICE_DFS_FILE_SYSTEM => DeviceType::DfsFileSystem,
            FILE_DEVICE_DFS_VOLUME => DeviceType::DfsVolume,
            //FILE_DEVICE_DISK => DeviceType::Disk,
            FILE_DEVICE_DISK_FILE_SYSTEM => DeviceType::DiskFileSystem,
            //FILE_DEVICE_DVD => DeviceType::Dvd,
            FILE_DEVICE_FILE_SYSTEM => DeviceType::FileSystem,
            FILE_DEVICE_FIPS => DeviceType::Fips,
            FILE_DEVICE_FULLSCREEN_VIDEO => DeviceType::FullscreenVideo,
            FILE_DEVICE_INPORT_PORT => DeviceType::InportPort,
            FILE_DEVICE_KEYBOARD => DeviceType::Keyboard,
            FILE_DEVICE_KS => DeviceType::Ks,
            FILE_DEVICE_KSEC => DeviceType::Ksec,
            FILE_DEVICE_MAILSLOT => DeviceType::Mailslot,
            FILE_DEVICE_MASS_STORAGE => DeviceType::MassStorage,
            FILE_DEVICE_MIDI_IN => DeviceType::MidiIn,
            FILE_DEVICE_MIDI_OUT => DeviceType::MidiOut,
            FILE_DEVICE_MODEM => DeviceType::Modem,
            FILE_DEVICE_MOUSE => DeviceType::Mouse,
            FILE_DEVICE_MULTI_UNC_PROVIDER => DeviceType::MultiUncProvider,
            FILE_DEVICE_NAMED_PIPE => DeviceType::NamedPipe,
            FILE_DEVICE_NETWORK => DeviceType::Network,
            FILE_DEVICE_NETWORK_BROWSER => DeviceType::NetworkBrowser,
            FILE_DEVICE_NETWORK_FILE_SYSTEM => DeviceType::NetworkFileSystem,
            FILE_DEVICE_NETWORK_REDIRECTOR => DeviceType::NetworkRedirector,
            FILE_DEVICE_NULL => DeviceType::Null,
            FILE_DEVICE_PARALLEL_PORT => DeviceType::ParallelPort,
            FILE_DEVICE_PHYSICAL_NETCARD => DeviceType::PhysicalNetcard,
            FILE_DEVICE_PRINTER => DeviceType::Printer,
            FILE_DEVICE_SCANNER => DeviceType::Scanner,
            FILE_DEVICE_SCREEN => DeviceType::Screen,
            FILE_DEVICE_SERENUM => DeviceType::Serenum,
            FILE_DEVICE_SERIAL_MOUSE_PORT => DeviceType::SerialMousePort,
            FILE_DEVICE_SERIAL_PORT => DeviceType::SerialPort,
            //FILE_DEVICE_SMARTCARD => DeviceType::Smartcard,
            FILE_DEVICE_SMB => DeviceType::Smb,
            FILE_DEVICE_SOUND => DeviceType::Sound,
            FILE_DEVICE_STREAMS => DeviceType::Streams,
            //FILE_DEVICE_TAPE => DeviceType::Tape,
            FILE_DEVICE_TAPE_FILE_SYSTEM => DeviceType::TapeFileSystem,
            FILE_DEVICE_TERMSRV => DeviceType::Termsrv,
            FILE_DEVICE_TRANSPORT => DeviceType::Transport,
            FILE_DEVICE_UNKNOWN => DeviceType::Unknown,
            FILE_DEVICE_VDM => DeviceType::Vdm,
            FILE_DEVICE_VIDEO => DeviceType::Video,
            FILE_DEVICE_VIRTUAL_DISK => DeviceType::VirtualDisk,
            FILE_DEVICE_WAVE_IN => DeviceType::WaveIn,
            FILE_DEVICE_WAVE_OUT => DeviceType::WaveOut,
            _ => DeviceType::Unknown,
        }
    }
}

impl From<DeviceType> for u32 {
    fn from(val: DeviceType) -> Self {
        val.into_u32()
    }
}

impl From<u32> for DeviceType {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RequiredAccess: u32 {
        const ANY_ACCESS = FILE_ANY_ACCESS;
        const READ_DATA = FILE_READ_DATA.0;
        const WRITE_DATA = FILE_WRITE_DATA.0;
        const READ_WRITE_DATA = RequiredAccess::READ_DATA.bits() | RequiredAccess::WRITE_DATA.bits();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TransferMethod {
    Neither = METHOD_NEITHER,
    InputDirect = METHOD_IN_DIRECT,
    OutputDirect = METHOD_OUT_DIRECT,
    Buffered = METHOD_BUFFERED,
}

impl TransferMethod {
    pub const fn try_from_u32(value: u32) -> Option<Self> {
        match value & 0x3 {
            METHOD_NEITHER => Some(Self::Neither),
            METHOD_IN_DIRECT => Some(Self::InputDirect),
            METHOD_OUT_DIRECT => Some(Self::OutputDirect),
            METHOD_BUFFERED => Some(Self::Buffered),
            _ => None,
        }
    }

    pub const fn into_u32(self) -> u32 {
        match self {
            Self::Neither => METHOD_NEITHER,
            Self::InputDirect => METHOD_IN_DIRECT,
            Self::OutputDirect => METHOD_OUT_DIRECT,
            Self::Buffered => METHOD_BUFFERED,
        }
    }
}

impl From<TransferMethod> for u32 {
    fn from(val: TransferMethod) -> Self {
        val.into_u32()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ControlCode(pub DeviceType, pub RequiredAccess, pub u32, pub TransferMethod);

impl ControlCode {
    const METHOD_BITS: usize = 2;
    const NUM_BITS: usize = 12;
    const ACCESS_BITS: usize = 2;
    const TYPE_BITS: usize = 16;

    const METHOD_SHIFT: usize = 0;
    const NUM_SHIFT: usize = Self::METHOD_SHIFT + Self::METHOD_BITS;
    const ACCESS_SHIFT: usize = Self::NUM_SHIFT + Self::NUM_BITS;
    const TYPE_SHIFT: usize = Self::ACCESS_SHIFT + Self::ACCESS_BITS;

    const METHOD_MASK: u32 = (1 << Self::METHOD_BITS) - 1;
    const NUM_MASK: u32 = (1 << Self::NUM_BITS) - 1;
    const ACCESS_MASK: u32 = (1 << Self::ACCESS_BITS) - 1;
    const TYPE_MASK: u32 = (1 << Self::TYPE_BITS) - 1;

    pub const fn dev_type(&self) -> DeviceType {
        self.0
    }

    pub const fn required_access(&self) -> RequiredAccess {
        self.1
    }

    pub const fn num(&self) -> u32 {
        self.2
    }

    pub const fn transfer_method(&self) -> TransferMethod {
        self.3
    }

    pub const fn try_from_u32(value: u32) -> Option<Self> {
        let method = (value >> Self::METHOD_SHIFT) & Self::METHOD_MASK;
        let num = (value >> Self::NUM_SHIFT) & Self::NUM_MASK;
        let access = (value >> Self::ACCESS_SHIFT) & Self::ACCESS_MASK;
        let ty = (value >> Self::TYPE_SHIFT) & Self::TYPE_MASK;

        Some(Self(
            DeviceType::from_u32(ty),
            match RequiredAccess::from_bits(access) {
                Some(access) => access,
                None => return None,
            },
            num,
            match TransferMethod::try_from_u32(method) {
                Some(method) => method,
                None => return None,
            },
        ))
    }

    pub const fn into_u32(self) -> u32 {
        let method = self.transfer_method().into_u32() << Self::METHOD_SHIFT;
        let num = self.num() << Self::NUM_SHIFT;
        let access = self.required_access().bits() << Self::ACCESS_SHIFT;
        let ty = self.dev_type().into_u32() << Self::TYPE_SHIFT;

        ty | access | num | method
    }
}

impl From<ControlCode> for u32 {
    fn from(val: ControlCode) -> Self {
        val.into_u32()
    }
}
