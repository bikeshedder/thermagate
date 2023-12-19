use strum::EnumMessage;

#[derive(Debug, Default)]
pub struct BoolParam {
    pub id: u16,
    pub name: &'static str,
}

#[derive(Debug, Default)]
pub struct IntParam {
    pub id: u16,
    pub name: &'static str,
    pub unit: Unit,
}

pub struct FloatParam {
    pub id: u16,
    pub name: &'static str,
    pub unit: Unit,
    pub scale: u16,
}

pub struct EnumParam<T> {
    pub id: u16,
    pub name: &'static str,
    pub default: Option<T>,
}

pub trait Param {
    type Value;
    fn id(&self) -> u16;
    fn name(&self) -> &str;
    fn unit(&self) -> Unit;
    fn decode(&self, data: [u8; 2]) -> Self::Value;
}

#[derive(Default, Debug, Copy, Clone, EnumMessage)]
pub enum Unit {
    #[strum(message = "°C")]
    DegCelsius,
    #[strum(message = "kWh")]
    KiloWattHours,
    #[strum(message = "l/h")]
    LitersPerHour,
    #[strum(message = "bar")]
    Bar,
    #[strum(message = "h")]
    Hours,
    #[strum(message = "%")]
    Percent,
    /// FIXME this unit should probably be removed?
    #[default]
    None,
}

impl Param for BoolParam {
    type Value = bool;
    fn id(&self) -> u16 {
        self.id
    }
    fn name(&self) -> &str {
        self.name
    }
    fn unit(&self) -> Unit {
        Unit::None
    }
    fn decode(&self, data: [u8; 2]) -> Self::Value {
        ((data[0] as u16) << 8) + (data[1] as u16) != 0
    }
}

impl Param for IntParam {
    type Value = i16;
    fn id(&self) -> u16 {
        self.id
    }
    fn name(&self) -> &str {
        self.name
    }
    fn unit(&self) -> Unit {
        self.unit
    }
    fn decode(&self, data: [u8; 2]) -> Self::Value {
        (((data[0] as u16) << 8) + (data[1] as u16)) as i16
    }
}

impl Param for FloatParam {
    type Value = f32;
    fn id(&self) -> u16 {
        self.id
    }
    fn name(&self) -> &str {
        self.name
    }
    fn unit(&self) -> Unit {
        self.unit
    }
    fn decode(&self, data: [u8; 2]) -> Self::Value {
        let value = (((data[0] as u16) << 8) + (data[1] as u16)) as i16;
        value as f32 / self.scale as f32
    }
}

impl<T> Param for EnumParam<T>
where
    T: From<u8>,
{
    type Value = T;
    fn id(&self) -> u16 {
        self.id
    }
    fn name(&self) -> &str {
        self.name
    }
    fn unit(&self) -> Unit {
        Unit::None
    }
    fn decode(&self, data: [u8; 2]) -> Self::Value {
        T::from(data[0])
    }
}
