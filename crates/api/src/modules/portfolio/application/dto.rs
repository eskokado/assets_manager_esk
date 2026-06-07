use rust_decimal::Decimal;
use serde::Serialize;
use shared_kernel::EntityId;

use crate::modules::portfolio::domain::PositionView;

#[derive(Debug, Clone)]
pub struct GetUserPortfolioInput {
    pub user_id: EntityId,
}

#[derive(Debug, Clone)]
pub struct GetPositionInput {
    pub user_id: EntityId,
    pub asset_id: EntityId,
}

#[derive(Debug, Clone, Serialize)]
pub struct PositionOut {
    pub asset_id: String,
    pub asset_ticker: String,
    pub asset_name: String,
    pub quantity: String,
    pub average_price: String,
    pub currency: String,
    pub total_invested: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PortfolioOut {
    pub total_invested: String,
    pub positions: Vec<PositionOut>,
}

impl PositionOut {
    pub fn from_view(view: &PositionView) -> Self {
        let quantity = view.position.quantity().value();
        let average_price = view.position.average_price().amount();
        let total_invested = quantity * average_price;
        Self {
            asset_id: view.position.asset_id().to_string(),
            asset_ticker: view.asset_ticker.clone(),
            asset_name: view.asset_name.clone(),
            quantity: quantity.to_string(),
            average_price: average_price.to_string(),
            currency: view.position.average_price().currency().to_string(),
            total_invested: total_invested.to_string(),
        }
    }
}

impl PortfolioOut {
    pub fn from_views(views: &[PositionView]) -> Self {
        let positions: Vec<PositionOut> = views.iter().map(PositionOut::from_view).collect();
        let total: Decimal = views.iter().fold(Decimal::ZERO, |acc, view| {
            let qty = view.position.quantity().value();
            let avg = view.position.average_price().amount();
            acc + qty * avg
        });
        Self {
            total_invested: total.to_string(),
            positions,
        }
    }
}
