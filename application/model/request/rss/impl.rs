use super::*;

impl std::str::FromStr for Timezone {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UTC" => Ok(Timezone::Utc),
            "EST" => Ok(Timezone::Est),
            "EDT" => Ok(Timezone::Edt),
            "CST" => Ok(Timezone::Cst),
            "CDT" => Ok(Timezone::Cdt),
            "MST" => Ok(Timezone::Mst),
            "MDT" => Ok(Timezone::Mdt),
            "PST" => Ok(Timezone::Pst),
            "PDT" => Ok(Timezone::Pdt),
            "GMT" => Ok(Timezone::Gmt),
            "CST_CN" => Ok(Timezone::CstCn),
            "JST" => Ok(Timezone::Jst),
            "IST" => Ok(Timezone::Ist),
            "AEST" => Ok(Timezone::Aest),
            "AEDT" => Ok(Timezone::Aedt),
            "CET" => Ok(Timezone::Cet),
            "CEST" => Ok(Timezone::Cest),
            _ => Err(format!("Unknown timezone: {}", s)),
        }
    }
}
