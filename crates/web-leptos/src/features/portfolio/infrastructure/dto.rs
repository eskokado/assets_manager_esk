use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PortfolioResponseDto {
    pub total_invested: String,
    pub positions: Vec<PositionDto>,
}

#[derive(Debug, Deserialize)]
pub struct PositionDto {
    pub asset_id: String,
    pub asset_ticker: String,
    pub asset_name: String,
    pub quantity: String,
    pub average_price: String,
    #[allow(dead_code)]
    pub currency: String,
    pub total_invested: String,
}
