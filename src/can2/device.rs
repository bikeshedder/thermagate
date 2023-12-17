use std::fmt;

use strum::{AsRefStr, FromRepr};

#[derive(Debug, strum::Display, FromRepr, AsRefStr)]
#[repr(u16)]
pub enum Device {
    RoCon = 0x10a,
    HG1 = 0x180, // aka. Main
    HG2 = 0x181,
    HG3 = 0x182,
    HG4 = 0x183,
    HG5 = 0x184,
    HG6 = 0x185,
    HG7 = 0x186,
    HG8 = 0x187,
    HC1 = 0x300, // aka. Boiler
    HC2 = 0x301,
    HC3 = 0x302,
    HC4 = 0x303,
    HC5 = 0x304,
    HC6 = 0x305,
    HC7 = 0x306,
    HC8 = 0x307,
    HCM1 = 0x600,
    HCM2 = 0x601,
    HCM3 = 0x602,
    HCM4 = 0x603,
    HCM5 = 0x604,
    HCM6 = 0x605,
    HCM7 = 0x606,
    HCM8 = 0x607,
    HCM9 = 0x608,
    HCM10 = 0x609,
    HCM11 = 0x60a,
    HCM12 = 0x60b,
    HCM13 = 0x60c,
    HCM14 = 0x60d,
    HCM15 = 0x60e,
    HCM16 = 0x60f,
    G1_2 = 0x68d,
    G1 = 0x69d,
    // FIXME use a better name for ourselves
    NRG = 0x666,
}

#[derive(Debug)]
pub enum Addr {
    Device(Device),
    Unknown(u16),
}

impl fmt::Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown(addr) => write!(f, "0x{:03x}", addr),
            Self::Device(dev) => write!(f, "{}", dev),
        }
    }
}

impl From<u16> for Addr {
    fn from(id: u16) -> Self {
        Device::from_repr(id)
            .map(Self::Device)
            .unwrap_or(Self::Unknown(id))
    }
}
