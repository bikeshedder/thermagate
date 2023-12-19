use num_enum::FromPrimitive;

#[derive(Debug, FromPrimitive, strum::Display)]
#[repr(u8)]
pub enum Betriebsart {
    Standby = 0,
    Heizen = 1,
    Kühlen = 2,
    Abtauen = 3,
    Warmwasserbereitung = 4,
    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Debug, FromPrimitive, strum::Display)]
#[repr(u8)]
pub enum Programm {
    Bereitschaft = 1,
    Heizen = 3,
    Absenken = 4,
    Sommer = 5,
    Automatik1 = 11,
    Automatik2 = 12,
    Kuehlen = 17,
    #[num_enum(catch_all)]
    Unknown(u8),
}

// 1=Bereitschaft,3=Heizen,4=Absenken,5=Sommer,17=Kühlen,11=Automatik 1,12=Automatik 2

#[derive(Debug, FromPrimitive, strum::Display)]
#[repr(u8)]
pub enum SmartGrid {
    Normal = 0,
    SG1 = 1,
    SG2 = 2,
    SG3 = 3,
    #[num_enum(catch_all)]
    Unknown(u8),
}
