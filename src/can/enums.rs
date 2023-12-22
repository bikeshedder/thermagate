use num_enum::FromPrimitive;

#[derive(Debug, FromPrimitive, strum::Display)]
#[repr(i16)]
pub enum Betriebsart {
    Standby = 0,
    Heizen = 1,
    Kühlen = 2,
    Abtauen = 3,
    Warmwasserbereitung = 4,
    #[num_enum(catch_all)]
    Unknown(i16),
}

#[derive(Debug, FromPrimitive, strum::Display)]
#[repr(i8)]
pub enum Programm {
    Bereitschaft = 1,
    Heizen = 3,
    Absenken = 4,
    Sommer = 5,
    Automatik1 = 11,
    Automatik2 = 12,
    Kuehlen = 17,
    #[num_enum(catch_all)]
    Unknown(i8),
}

#[derive(Debug, FromPrimitive, strum::Display)]
#[repr(i16)]
pub enum SG {
    SGN = 3,
    SG1 = 4,
    SG2 = 5,
    SG3 = 6,
    #[num_enum(catch_all)]
    Unknown(i16),
}
