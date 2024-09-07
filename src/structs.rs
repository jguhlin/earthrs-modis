use serde::de::{self, Deserializer};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct DateInfo {
    pub modis_date: String,
    pub calendar_date: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DatesWrapper {
    pub dates: Vec<DateInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub product: ProductType,
    pub description: String,
    pub frequency: String,
    pub resolution_meters: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductsData {
    pub products: Vec<Product>,
}

#[derive(Debug)]
pub enum ProductType {
    Daymet,
    ECO4ESIPTJPL,
    ECO4WUE,
    GEDI03,
    GEDI04_B,
    MCD12Q1,
    MCD12Q2,
    MCD15A2H,
    MCD15A3H,
    MCD43A,
    MCD43A1,
    MCD43A4,
    MCD64A1,
    MOD09A1,
    MOD11A2,
    MOD13Q1,
    MOD14A2,
    MOD15A2H,
    MOD16A2,
    MOD16A2GF,
    MOD17A2H,
    MOD17A2HGF,
    MOD17A3HGF,
    MOD21A2,
    MOD44B,
    MYD09A1,
    MYD11A2,
    MYD13Q1,
    MYD14A2,
    MYD15A2H,
    MYD16A2,
    MYD16A2GF,
    MYD17A2H,
    MYD17A2HGF,
    MYD17A3HGF,
    MYD21A2,
    SIF005,
    SIF_ANN,
    SPL3SMP_E,
    SPL4CMDL,
    VNP09A1,
    VNP09H1,
    VNP13A1,
    VNP15A2H,
    VNP21A2,
    VNP22Q2,
}

impl Serialize for ProductType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match *self {
            ProductType::Daymet => "Daymet",
            ProductType::ECO4ESIPTJPL => "ECO4ESIPTJPL",
            ProductType::ECO4WUE => "ECO4WUE",
            ProductType::GEDI03 => "GEDI03",
            ProductType::GEDI04_B => "GEDI04_B",
            ProductType::MCD12Q1 => "MCD12Q1",
            ProductType::MCD12Q2 => "MCD12Q2",
            ProductType::MCD15A2H => "MCD15A2H",
            ProductType::MCD15A3H => "MCD15A3H",
            ProductType::MCD43A => "MCD43A",
            ProductType::MCD43A1 => "MCD43A1",
            ProductType::MCD43A4 => "MCD43A4",
            ProductType::MCD64A1 => "MCD64A1",
            ProductType::MOD09A1 => "MOD09A1",
            ProductType::MOD11A2 => "MOD11A2",
            ProductType::MOD13Q1 => "MOD13Q1",
            ProductType::MOD14A2 => "MOD14A2",
            ProductType::MOD15A2H => "MOD15A2H",
            ProductType::MOD16A2 => "MOD16A2",
            ProductType::MOD16A2GF => "MOD16A2GF",
            ProductType::MOD17A2H => "MOD17A2H",
            ProductType::MOD17A2HGF => "MOD17A2HGF",
            ProductType::MOD17A3HGF => "MOD17A3HGF",
            ProductType::MOD21A2 => "MOD21A2",
            ProductType::MOD44B => "MOD44B",
            ProductType::MYD09A1 => "MYD09A1",
            ProductType::MYD11A2 => "MYD11A2",
            ProductType::MYD13Q1 => "MYD13Q1",
            ProductType::MYD14A2 => "MYD14A2",
            ProductType::MYD15A2H => "MYD15A2H",
            ProductType::MYD16A2 => "MYD16A2",
            ProductType::MYD16A2GF => "MYD16A2GF",
            ProductType::MYD17A2H => "MYD17A2H",
            ProductType::MYD17A2HGF => "MYD17A2HGF",
            ProductType::MYD17A3HGF => "MYD17A3HGF",
            ProductType::MYD21A2 => "MYD21A2",
            ProductType::SIF005 => "SIF005",
            ProductType::SIF_ANN => "SIF_ANN",
            ProductType::SPL3SMP_E => "SPL3SMP_E",
            ProductType::SPL4CMDL => "SPL4CMDL",
            ProductType::VNP09A1 => "VNP09A1",
            ProductType::VNP09H1 => "VNP09H1",
            ProductType::VNP13A1 => "VNP13A1",
            ProductType::VNP15A2H => "VNP15A2H",
            ProductType::VNP21A2 => "VNP21A2",
            ProductType::VNP22Q2 => "VNP22Q2",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for ProductType {
    fn deserialize<D>(deserializer: D) -> Result<ProductType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        match s {
            "Daymet" => Ok(ProductType::Daymet),
            "ECO4ESIPTJPL" => Ok(ProductType::ECO4ESIPTJPL),
            "ECO4WUE" => Ok(ProductType::ECO4WUE),
            "GEDI03" => Ok(ProductType::GEDI03),
            "GEDI04_B" => Ok(ProductType::GEDI04_B),
            "MCD12Q1" => Ok(ProductType::MCD12Q1),
            "MCD12Q2" => Ok(ProductType::MCD12Q2),
            "MCD15A2H" => Ok(ProductType::MCD15A2H),
            "MCD15A3H" => Ok(ProductType::MCD15A3H),
            "MCD43A" => Ok(ProductType::MCD43A),
            "MCD43A1" => Ok(ProductType::MCD43A1),
            "MCD43A4" => Ok(ProductType::MCD43A4),
            "MCD64A1" => Ok(ProductType::MCD64A1),
            "MOD09A1" => Ok(ProductType::MOD09A1),
            "MOD11A2" => Ok(ProductType::MOD11A2),
            "MOD13Q1" => Ok(ProductType::MOD13Q1),
            "MOD14A2" => Ok(ProductType::MOD14A2),
            "MOD15A2H" => Ok(ProductType::MOD15A2H),
            "MOD16A2" => Ok(ProductType::MOD16A2),
            "MOD16A2GF" => Ok(ProductType::MOD16A2GF),
            "MOD17A2H" => Ok(ProductType::MOD17A2H),
            "MOD17A2HGF" => Ok(ProductType::MOD17A2HGF),
            "MOD17A3HGF" => Ok(ProductType::MOD17A3HGF),
            "MOD21A2" => Ok(ProductType::MOD21A2),
            "MOD44B" => Ok(ProductType::MOD44B),
            "MYD09A1" => Ok(ProductType::MYD09A1),
            "MYD11A2" => Ok(ProductType::MYD11A2),
            "MYD13Q1" => Ok(ProductType::MYD13Q1),
            "MYD14A2" => Ok(ProductType::MYD14A2),
            "MYD15A2H" => Ok(ProductType::MYD15A2H),
            "MYD16A2" => Ok(ProductType::MYD16A2),
            "MYD16A2GF" => Ok(ProductType::MYD16A2GF),
            "MYD17A2H" => Ok(ProductType::MYD17A2H),
            "MYD17A2HGF" => Ok(ProductType::MYD17A2HGF),
            "MYD17A3HGF" => Ok(ProductType::MYD17A3HGF),
            "MYD21A2" => Ok(ProductType::MYD21A2),
            "SIF005" => Ok(ProductType::SIF005),
            "SIF_ANN" => Ok(ProductType::SIF_ANN),
            "SPL3SMP_E" => Ok(ProductType::SPL3SMP_E),
            "SPL4CMDL" => Ok(ProductType::SPL4CMDL),
            "VNP09A1" => Ok(ProductType::VNP09A1),
            "VNP09H1" => Ok(ProductType::VNP09H1),
            "VNP13A1" => Ok(ProductType::VNP13A1),
            "VNP15A2H" => Ok(ProductType::VNP15A2H),
            "VNP21A2" => Ok(ProductType::VNP21A2),
            "VNP22Q2" => Ok(ProductType::VNP22Q2),
            _ => Err(de::Error::custom("unexpected product type")),
        }
    }
}

impl Into<&str> for ProductType {
    fn into(self) -> &'static str {
        match self {
            ProductType::Daymet => "Daymet",
            ProductType::ECO4ESIPTJPL => "ECO4ESIPTJPL",
            ProductType::ECO4WUE => "ECO4WUE",
            ProductType::GEDI03 => "GEDI03",
            ProductType::GEDI04_B => "GEDI04_B",
            ProductType::MCD12Q1 => "MCD12Q1",
            ProductType::MCD12Q2 => "MCD12Q2",
            ProductType::MCD15A2H => "MCD15A2H",
            ProductType::MCD15A3H => "MCD15A3H",
            ProductType::MCD43A => "MCD43A",
            ProductType::MCD43A1 => "MCD43A1",
            ProductType::MCD43A4 => "MCD43A4",
            ProductType::MCD64A1 => "MCD64A1",
            ProductType::MOD09A1 => "MOD09A1",
            ProductType::MOD11A2 => "MOD11A2",
            ProductType::MOD13Q1 => "MOD13Q1",
            ProductType::MOD14A2 => "MOD14A2",
            ProductType::MOD15A2H => "MOD15A2H",
            ProductType::MOD16A2 => "MOD16A2",
            ProductType::MOD16A2GF => "MOD16A2GF",
            ProductType::MOD17A2H => "MOD17A2H",
            ProductType::MOD17A2HGF => "MOD17A2HGF",
            ProductType::MOD17A3HGF => "MOD17A3HGF",
            ProductType::MOD21A2 => "MOD21A2",
            ProductType::MOD44B => "MOD44B",
            ProductType::MYD09A1 => "MYD09A1",
            ProductType::MYD11A2 => "MYD11A2",
            ProductType::MYD13Q1 => "MYD13Q1",
            ProductType::MYD14A2 => "MYD14A2",
            ProductType::MYD15A2H => "MYD15A2H",
            ProductType::MYD16A2 => "MYD16A2",
            ProductType::MYD16A2GF => "MYD16A2GF",
            ProductType::MYD17A2H => "MYD17A2H",
            ProductType::MYD17A2HGF => "MYD17A2HGF",
            ProductType::MYD17A3HGF => "MYD17A3HGF",
            ProductType::MYD21A2 => "MYD21A2",
            ProductType::SIF005 => "SIF005",
            ProductType::SIF_ANN => "SIF_ANN",
            ProductType::SPL3SMP_E => "SPL3SMP_E",
            ProductType::SPL4CMDL => "SPL4CMDL",
            ProductType::VNP09A1 => "VNP09A1",
            ProductType::VNP09H1 => "VNP09H1",
            ProductType::VNP13A1 => "VNP13A1",
            ProductType::VNP15A2H => "VNP15A2H",
            ProductType::VNP21A2 => "VNP21A2",
            ProductType::VNP22Q2 => "VNP22Q2",
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Site {
    pub siteid: String,
    pub sitename: String,
    pub network: String,
    pub latitude: f64,
    pub longitude: f64,
    pub state: Option<String>,
    pub country: String,
}

#[derive(Deserialize, Debug)]
pub struct Sites {
    pub sites: Vec<Site>,
}

#[derive(Deserialize, Debug)]
pub struct Subset {
    pub modis_date: String,
    pub calendar_date: String,
    pub band: String,
    pub tile: String,
    pub proc_date: String,
    pub data: Vec<i32>,
}

#[derive(Deserialize, Debug)]
pub struct ModisData {
    pub xllcorner: String,
    pub yllcorner: String,
    pub cellsize: f64,
    pub nrows: i32,
    pub ncols: i32,
    pub band: String,
    pub units: String,
    pub scale: String,
    pub latitude: f64,
    pub longitude: f64,
    pub header: String,
    pub subset: Vec<Subset>,
}
