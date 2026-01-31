use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub info: Info,
    pub metadata: Metadata,
    #[serde(rename = "securityInfo")]
    pub security_info: SecurityInfo,
    #[serde(rename = "sddDetails")]
    pub sdd_details: SddDetails,
    #[serde(rename = "currentMarketType")]
    pub current_market_type: String,
    #[serde(rename = "priceInfo")]
    pub price_info: PriceInfo,
    #[serde(rename = "industryInfo")]
    pub industry_info: IndustryInfo,
    #[serde(rename = "preOpenMarket")]
    pub pre_open_market: PreOpenMarket,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub symbol: String,
    #[serde(rename = "companyName")]
    pub company_name: String,
    pub industry: String,
    #[serde(rename = "activeSeries")]
    pub active_series: Vec<String>,
    #[serde(rename = "debtSeries")]
    pub debt_series: Vec<String>,
    #[serde(rename = "isFNOSec")]
    pub is_fnosec: bool,
    #[serde(rename = "isCASec")]
    pub is_casec: bool,
    #[serde(rename = "isSLBSec")]
    pub is_slbsec: bool,
    #[serde(rename = "isDebtSec")]
    pub is_debt_sec: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    #[serde(rename = "tempSuspendedSeries")]
    pub temp_suspended_series: Vec<String>,
    #[serde(rename = "isETFSec")]
    pub is_etfsec: bool,
    #[serde(rename = "isDelisted")]
    pub is_delisted: bool,
    pub isin: String,
    pub slb_isin: String,
    #[serde(rename = "listingDate")]
    pub listing_date: String,
    #[serde(rename = "isMunicipalBond")]
    pub is_municipal_bond: bool,
    #[serde(rename = "isHybridSymbol")]
    pub is_hybrid_symbol: bool,
    pub segment: String,
    #[serde(rename = "isTop10")]
    pub is_top10: bool,
    pub identifier: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub series: String,
    pub symbol: String,
    pub isin: String,
    pub status: String,
    #[serde(rename = "listingDate")]
    pub listing_date: String,
    pub industry: String,
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: String,
    #[serde(rename = "pdSectorPe")]
    pub pd_sector_pe: String,
    #[serde(rename = "pdSymbolPe")]
    pub pd_symbol_pe: String,
    #[serde(rename = "pdSectorInd")]
    pub pd_sector_ind: String,
    #[serde(rename = "pdSectorIndAll")]
    pub pd_sector_ind_all: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PriceInfo {
    #[serde(rename = "lastPrice")]
    pub last_price: f64,
    pub change: f64,
    #[serde(rename = "pChange")]
    pub p_change: f64,
    #[serde(rename = "previousClose")]
    pub previous_close: f64,
    pub open: f64,
    pub close: f64,
    pub vwap: f64,
    #[serde(rename = "stockIndClosePrice")]
    pub stock_ind_close_price: f64,
    #[serde(rename = "lowerCP")]
    pub lower_cp: String,
    #[serde(rename = "upperCP")]
    pub upper_cp: String,
    #[serde(rename = "pPriceBand")]
    pub p_price_band: String,
    #[serde(rename = "basePrice")]
    pub base_price: f64,
    #[serde(rename = "intraDayHighLow")]
    pub intra_day_high_low: IntraDayHighLow,
    #[serde(rename = "weekHighLow")]
    pub week_high_low: WeekHighLow,
    #[serde(rename = "iNavValue")]
    pub i_nav_value: Option<f64>,
    #[serde(rename = "checkINAV")]
    pub check_inav: bool,
    #[serde(rename = "tickSize")]
    pub tick_size: f64,
    pub ieq: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IntraDayHighLow {
    pub min: f64,
    pub max: f64,
    pub value: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeekHighLow {
    pub min: f64,
    #[serde(rename = "minDate")]
    pub min_date: String,
    pub max: f64,
    #[serde(rename = "maxDate")]
    pub max_date: String,
    pub value: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityInfo {
    #[serde(rename = "boardStatus")]
    pub board_status: String,
    #[serde(rename = "tradingStatus")]
    pub trading_status: String,
    #[serde(rename = "tradingSegment")]
    pub trading_segment: String,
    #[serde(rename = "sessionNo")]
    pub session_no: String,
    pub slb: String,
    #[serde(rename = "classOfShare")]
    pub class_of_share: String,
    pub derivatives: String,
    pub surveillance: Surveillance,
    #[serde(rename = "faceValue")]
    pub face_value: f64,
    #[serde(rename = "issuedSize")]
    pub issued_size: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Surveillance {
    pub surv: String,
    pub desc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SddDetails {
    #[serde(rename = "SDDAuditor")]
    pub sddauditor: String,
    #[serde(rename = "SDDStatus")]
    pub sddstatus: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndustryInfo {
    pub r#macro: String,
    pub sector: String,
    pub industry: String,
    #[serde(rename = "basicIndustry")]
    pub basic_industry: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PreOpenMarket {
    pub preopen: Vec<PreOpenItem>,
    pub ato: Ato,
    #[serde(rename = "IEP")]
    pub iep: f64,
    #[serde(rename = "totalTradedVolume")]
    pub total_traded_volume: f64,
    #[serde(rename = "finalPrice")]
    pub final_price: f64,
    #[serde(rename = "finalQuantity")]
    pub final_quantity: f64,
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: String,
    #[serde(rename = "totalBuyQuantity")]
    pub total_buy_quantity: f64,
    #[serde(rename = "totalSellQuantity")]
    pub total_sell_quantity: f64,
    #[serde(rename = "atoBuyQty")]
    pub ato_buy_qty: f64,
    #[serde(rename = "atoSellQty")]
    pub ato_sell_qty: f64,
    #[serde(rename = "Change")]
    pub change: f64,
    #[serde(rename = "perChange")]
    pub per_change: f64,
    #[serde(rename = "prevClose")]
    pub prev_close: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PreOpenItem {
    pub price: f64,
    #[serde(rename = "buyQty")]
    pub buy_qty: f64,
    #[serde(rename = "sellQty")]
    pub sell_qty: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ato {
    buy: f64,
    sell: f64,
}
