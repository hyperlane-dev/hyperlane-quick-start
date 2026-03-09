use super::*;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, ToSchema)]
pub enum Timezone {
    #[default]
    #[serde(rename = "UTC")]
    Utc,
    #[serde(rename = "EST")]
    Est,
    #[serde(rename = "EDT")]
    Edt,
    #[serde(rename = "CST")]
    Cst,
    #[serde(rename = "CDT")]
    Cdt,
    #[serde(rename = "MST")]
    Mst,
    #[serde(rename = "MDT")]
    Mdt,
    #[serde(rename = "PST")]
    Pst,
    #[serde(rename = "PDT")]
    Pdt,
    #[serde(rename = "GMT")]
    Gmt,
    #[serde(rename = "CST_CN")]
    CstCn,
    #[serde(rename = "JST")]
    Jst,
    #[serde(rename = "IST")]
    Ist,
    #[serde(rename = "AEST")]
    Aest,
    #[serde(rename = "AEDT")]
    Aedt,
    #[serde(rename = "CET")]
    Cet,
    #[serde(rename = "CEST")]
    Cest,
}
