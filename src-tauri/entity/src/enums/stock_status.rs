use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    sea_orm::EnumIter,
    sea_orm::DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "String(Some(50))")]
pub enum StockStatus {
    #[serde(rename = "error")]
    #[sea_orm(string_value = "error")]
    Error,

    #[serde(rename = "pending")]
    #[sea_orm(string_value = "pending")]
    Pending,

    #[serde(rename = "live")]
    #[sea_orm(string_value = "live")]
    Live,

    #[serde(rename = "to_low_profit")]
    #[sea_orm(string_value = "to_low_profit")]
    ToLowProfit,

    #[serde(rename = "no_sellers")]
    #[sea_orm(string_value = "no_sellers")]
    NoSellers,

    #[serde(rename = "no_buyers")]
    #[sea_orm(string_value = "no_buyers")]
    NoBuyers,

    #[serde(rename = "inactive")]
    #[sea_orm(string_value = "inactive")]
    InActive,

    #[serde(rename = "sma_limit")]
    #[sea_orm(string_value = "sma_limit")]
    SMALimit,

    #[serde(rename = "order_limit")]
    #[sea_orm(string_value = "order_limit")]
    OrderLimit,

    #[serde(rename = "overpriced")]
    #[sea_orm(string_value = "overpriced")]
    Overpriced,

    #[serde(rename = "underpriced")]
    #[sea_orm(string_value = "underpriced")]
    Underpriced,

    #[serde(rename = "max_price_drop")]
    #[sea_orm(string_value = "max_price_drop")]
    MaxPriceDrop,

    #[serde(rename = "insufficient_standing")]
    #[sea_orm(string_value = "insufficient_standing")]
    InsufficientStanding,
}
impl StockStatus {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Pending => "pending",
            Self::Live => "live",
            Self::ToLowProfit => "to_low_profit",
            Self::NoSellers => "no_sellers",
            Self::NoBuyers => "no_buyers",
            Self::InActive => "inactive",
            Self::SMALimit => "sma_limit",
            Self::OrderLimit => "order_limit",
            Self::Overpriced => "overpriced",
            Self::Underpriced => "underpriced",
            Self::MaxPriceDrop => "max_price_drop",
            Self::InsufficientStanding => "insufficient_standing",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "error" => Some(Self::Error),
            "pending" => Some(Self::Pending),
            "live" => Some(Self::Live),
            "to_low_profit" => Some(Self::ToLowProfit),
            "no_sellers" => Some(Self::NoSellers),
            "no_buyers" => Some(Self::NoBuyers),
            "inactive" => Some(Self::InActive),
            "sma_limit" => Some(Self::SMALimit),
            "order_limit" => Some(Self::OrderLimit),
            "overpriced" => Some(Self::Overpriced),
            "underpriced" => Some(Self::Underpriced),
            "max_price_drop" => Some(Self::MaxPriceDrop),
            "insufficient_standing" => Some(Self::InsufficientStanding),
            _ => None,
        }
    }
}

impl std::fmt::Display for StockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
