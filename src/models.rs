use crate::nse_f64;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub info: Option<Info>,
    pub metadata: Option<Metadata>,
    #[serde(rename = "securityInfo")]
    pub security_info: Option<SecurityInfo>,
    #[serde(rename = "sddDetails")]
    pub sdd_details: Option<SddDetails>,
    #[serde(rename = "currentMarketType")]
    pub current_market_type: Option<String>,
    #[serde(rename = "priceInfo")]
    pub price_info: Option<PriceInfo>,
    #[serde(rename = "industryInfo")]
    pub industry_info: Option<IndustryInfo>,
    #[serde(rename = "preOpenMarket")]
    pub pre_open_market: Option<PreOpenMarket>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub symbol: Option<String>,
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,
    pub industry: Option<String>,
    #[serde(rename = "activeSeries")]
    pub active_series: Vec<String>,
    #[serde(rename = "debtSeries")]
    pub debt_series: Vec<String>,
    #[serde(rename = "isFNOSec")]
    pub is_fnosec: Option<bool>,
    #[serde(rename = "isCASec")]
    pub is_casec: Option<bool>,
    #[serde(rename = "isSLBSec")]
    pub is_slbsec: Option<bool>,
    #[serde(rename = "isDebtSec")]
    pub is_debt_sec: Option<bool>,
    #[serde(rename = "isSuspended")]
    pub is_suspended: Option<bool>,
    #[serde(rename = "tempSuspendedSeries")]
    pub temp_suspended_series: Vec<String>,
    #[serde(rename = "isETFSec")]
    pub is_etfsec: Option<bool>,
    #[serde(rename = "isDelisted")]
    pub is_delisted: Option<bool>,
    pub isin: Option<String>,
    pub slb_isin: Option<String>,
    #[serde(rename = "listingDate")]
    pub listing_date: Option<String>,
    #[serde(rename = "isMunicipalBond")]
    pub is_municipal_bond: Option<bool>,
    #[serde(rename = "isHybridSymbol")]
    pub is_hybrid_symbol: Option<bool>,
    pub segment: Option<String>,
    #[serde(rename = "isTop10")]
    pub is_top10: Option<bool>,
    pub identifier: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub series: Option<String>,
    pub symbol: Option<String>,
    pub isin: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "listingDate")]
    pub listing_date: Option<String>,
    pub industry: Option<String>,
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: Option<String>,
    #[serde(rename = "pdSectorPe", deserialize_with = "nse_f64")]
    pub pd_sector_pe: Option<f64>,
    #[serde(rename = "pdSymbolPe", deserialize_with = "nse_f64")]
    pub pd_symbol_pe: Option<f64>,
    #[serde(rename = "pdSectorInd")]
    pub pd_sector_ind: Option<String>,
    #[serde(rename = "pdSectorIndAll", deserialize_with = "nse_f64")]
    pub pd_sector_ind_all: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PriceInfo {
    #[serde(rename = "lastPrice")]
    pub last_price: Option<f64>,
    pub change: Option<f64>,
    #[serde(rename = "pChange")]
    pub p_change: Option<f64>,
    #[serde(rename = "previousClose")]
    pub previous_close: Option<f64>,
    pub open: Option<f64>,
    pub close: Option<f64>,
    pub vwap: Option<f64>,
    #[serde(rename = "stockIndClosePrice")]
    pub stock_ind_close_price: Option<f64>,
    #[serde(rename = "lowerCP")]
    pub lower_cp: Option<String>,
    #[serde(rename = "upperCP")]
    pub upper_cp: Option<String>,
    #[serde(rename = "pPriceBand")]
    pub p_price_band: Option<String>,
    #[serde(rename = "basePrice")]
    pub base_price: Option<f64>,
    #[serde(rename = "intraDayHighLow")]
    pub intra_day_high_low: Option<IntraDayHighLow>,
    #[serde(rename = "weekHighLow")]
    pub week_high_low: Option<WeekHighLow>,
    #[serde(rename = "iNavValue")]
    pub i_nav_value: Option<f64>,
    #[serde(rename = "checkINAV")]
    pub check_inav: Option<bool>,
    #[serde(rename = "tickSize", deserialize_with = "nse_f64")]
    pub tick_size: Option<f64>,
    pub ieq: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IntraDayHighLow {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeekHighLow {
    pub min: Option<f64>,
    #[serde(rename = "minDate")]
    pub min_date: Option<String>,
    pub max: Option<f64>,
    #[serde(rename = "maxDate")]
    pub max_date: Option<String>,
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityInfo {
    #[serde(rename = "boardStatus")]
    pub board_status: Option<String>,
    #[serde(rename = "tradingStatus")]
    pub trading_status: Option<String>,
    #[serde(rename = "tradingSegment")]
    pub trading_segment: Option<String>,
    #[serde(rename = "sessionNo")]
    pub session_no: Option<String>,
    pub slb: Option<String>,
    #[serde(rename = "classOfShare")]
    pub class_of_share: Option<String>,
    pub derivatives: Option<String>,
    pub surveillance: Option<Surveillance>,
    #[serde(rename = "faceValue")]
    pub face_value: Option<f64>,
    #[serde(rename = "issuedSize", deserialize_with = "nse_f64")]
    pub issued_size: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Surveillance {
    pub surv: Option<String>,
    pub desc: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SddDetails {
    #[serde(rename = "SDDAuditor")]
    pub sddauditor: Option<String>,
    #[serde(rename = "SDDStatus")]
    pub sddstatus: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndustryInfo {
    pub r#macro: Option<String>,
    pub sector: Option<String>,
    pub industry: Option<String>,
    #[serde(rename = "basicIndustry")]
    pub basic_industry: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PreOpenMarket {
    pub preopen: Vec<PreOpenItem>,
    pub ato: Option<Ato>,
    #[serde(rename = "IEP")]
    pub iep: Option<f64>,
    #[serde(rename = "totalTradedVolume")]
    pub total_traded_volume: Option<f64>,
    #[serde(rename = "finalPrice")]
    pub final_price: Option<f64>,
    #[serde(rename = "finalQuantity")]
    pub final_quantity: Option<f64>,
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: Option<String>,
    #[serde(rename = "totalBuyQuantity")]
    pub total_buy_quantity: Option<f64>,
    #[serde(rename = "totalSellQuantity")]
    pub total_sell_quantity: Option<f64>,
    #[serde(rename = "atoBuyQty")]
    pub ato_buy_qty: Option<f64>,
    #[serde(rename = "atoSellQty")]
    pub ato_sell_qty: Option<f64>,
    #[serde(rename = "Change")]
    pub change: Option<f64>,
    #[serde(rename = "perChange")]
    pub per_change: Option<f64>,
    #[serde(rename = "prevClose")]
    pub prev_close: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PreOpenItem {
    pub price: Option<f64>,
    #[serde(rename = "buyQty")]
    pub buy_qty: Option<f64>,
    #[serde(rename = "sellQty")]
    pub sell_qty: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ato {
    pub buy: Option<f64>,
    pub sell: Option<f64>,
}
