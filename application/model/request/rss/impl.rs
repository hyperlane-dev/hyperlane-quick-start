use super::*;

/// Implementation of `Timezone` for `std::str::FromStr`.
impl FromStr for Timezone {
    type Err = String;

    /// Parses the value from its textual form.
    ///
    /// # Arguments
    ///
    /// - `&str` - The textual input.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Err>` - The converted str, or the failure reason.
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
            TIMEZONE_CST_CN => Ok(Timezone::CstCn),
            "JST" => Ok(Timezone::Jst),
            "IST" => Ok(Timezone::Ist),
            TIMEZONE_AEST => Ok(Timezone::Aest),
            TIMEZONE_AEDT => Ok(Timezone::Aedt),
            "CET" => Ok(Timezone::Cet),
            TIMEZONE_CEST => Ok(Timezone::Cest),
            _ => Err(format!("Unknown timezone: {}", s)),
        }
    }
}
