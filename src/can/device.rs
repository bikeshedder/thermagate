use std::fmt;

use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, IntoStaticStr};

///
#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    AsRefStr,
    FromPrimitive,
    IntoPrimitive,
    Serialize,
    Deserialize,
    IntoStaticStr,
)]
#[repr(u16)]
pub enum Device {
    /// Control panel
    RoCon = 0x10a,
    /// First heat generator
    HG1 = 0x180,
    HG2 = 0x181,
    HG3 = 0x182,
    HG4 = 0x183,
    HG5 = 0x184,
    HG6 = 0x185,
    HG7 = 0x186,
    HG8 = 0x187,
    /// First heating circuit
    /// This is the built-in circuit
    HC1 = 0x300,
    /// Second heating circuit
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
    /// Broadcast address for all heating circuits
    HCX = 0x379,
    Outdoor = 0x500,
    /// First heating circuit module
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
    /// Broadcast address for all HCM devices
    HCMX = 0x679,
    /// This is the address used by the ROTEX RoCon G1 Gateway (Daikin EHS157056)
    G1 = 0x69d,
    /// This is the address used by the control panel when communicating with HC2
    RoCon2 = 0x69e,
    /// This is the address used by this project. Experiments have shown that
    /// only addresses in the range of 0x680 and 0x6ff are allowed to perform
    /// any SET operations on other devices. This software uses 0x6fe as 0x6ff
    /// is probably the broadcast address for this device category.
    TG = 0x6fe,
    #[num_enum(catch_all)]
    Other(u16),
}

impl Device {
    pub fn id(&self) -> u16 {
        u16::from(*self)
    }
    pub fn name(&self) -> &'static str {
        self.into()
    }
    pub fn is_other(&self) -> bool {
        matches!(self, Self::Other(_))
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

#[test]
fn test_device_name() {
    assert_eq!(Device::Other(42).name(), "Other");
}
