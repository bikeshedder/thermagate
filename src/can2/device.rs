use std::fmt;

use num_enum::{FromPrimitive, IntoPrimitive};
use socketcan::StandardId;
use strum::AsRefStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, AsRefStr, FromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum Device {
    /// Control panel
    RoCon = 0x10a,
    /// 1st heat generator
    HG1 = 0x180,
    HG2 = 0x181,
    HG3 = 0x182,
    HG4 = 0x183,
    HG5 = 0x184,
    HG6 = 0x185,
    HG7 = 0x186,
    HG8 = 0x187,
    /// 1st heat generator
    /// This is the built-in circuit
    HC1 = 0x300,
    /// 2nd heat generator
    /// This is typically used for mixer devices such as the
    /// ROTEX RoCon M1 (Daikin EHS157068)
    HC2 = 0x301,
    HC3 = 0x302,
    HC4 = 0x303,
    HC5 = 0x304,
    HC6 = 0x305,
    HC7 = 0x306,
    HC8 = 0x307,
    HC9 = 0x308,
    HC10 = 0x309,
    HC11 = 0x30a,
    HC12 = 0x30b,
    HC13 = 0x30c,
    HC14 = 0x30d,
    HC15 = 0x30e,
    HC16 = 0x30f,
    HCAll = 0x379,
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
    HCMall = 0x679,
    /// This is the address used by the ROTEX RoCon G1 Gateway (Daikin EHS157056)
    G1 = 0x69d,
    /// This is the address used by this project
    GW = 0x666,
    #[num_enum(catch_all)]
    Other(u16),
}

impl From<Device> for socketcan::Id {
    fn from(value: Device) -> Self {
        socketcan::Id::Standard(StandardId::new(u16::from(value)).unwrap())
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Self::Other(addr) = self {
            write!(f, "0x{addr:03x}")
        } else {
            write!(f, "{}", self.as_ref())
        }
    }
}
