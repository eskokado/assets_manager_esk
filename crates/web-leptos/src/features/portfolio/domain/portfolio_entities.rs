use shared_kernel::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionSummary {
    asset_id: String,
    ticker: String,
    name: String,
    quantity: String,
    average_price: String,
    total_invested: String,
}

impl PositionSummary {
    pub fn try_new(
        asset_id: String,
        ticker: String,
        name: String,
        quantity: String,
        average_price: String,
        total_invested: String,
    ) -> Result<Self> {
        if asset_id.trim().is_empty() {
            return Result::err("Asset id is required");
        }
        Result::ok(Self {
            asset_id,
            ticker,
            name,
            quantity,
            average_price,
            total_invested,
        })
    }

    pub fn asset_id(&self) -> &str {
        &self.asset_id
    }

    pub fn ticker(&self) -> &str {
        &self.ticker
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn quantity(&self) -> &str {
        &self.quantity
    }

    pub fn average_price(&self) -> &str {
        &self.average_price
    }

    pub fn total_invested(&self) -> &str {
        &self.total_invested
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortfolioSummary {
    total_invested: String,
    positions: Vec<PositionSummary>,
}

impl PortfolioSummary {
    pub fn try_new(total_invested: String, positions: Vec<PositionSummary>) -> Result<Self> {
        Result::ok(Self {
            total_invested,
            positions,
        })
    }

    pub fn total_invested(&self) -> &str {
        &self.total_invested
    }

    pub fn positions(&self) -> &[PositionSummary] {
        &self.positions
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionDetail {
    asset_id: String,
    ticker: String,
    name: String,
    quantity: String,
    average_price: String,
    total_invested: String,
}

impl PositionDetail {
    pub fn try_new(
        asset_id: String,
        ticker: String,
        name: String,
        quantity: String,
        average_price: String,
        total_invested: String,
    ) -> Result<Self> {
        if asset_id.trim().is_empty() {
            return Result::err("Asset id is required");
        }
        Result::ok(Self {
            asset_id,
            ticker,
            name,
            quantity,
            average_price,
            total_invested,
        })
    }

    pub fn asset_id(&self) -> &str {
        &self.asset_id
    }

    pub fn ticker(&self) -> &str {
        &self.ticker
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn quantity(&self) -> &str {
        &self.quantity
    }

    pub fn average_price(&self) -> &str {
        &self.average_price
    }

    pub fn total_invested(&self) -> &str {
        &self.total_invested
    }
}
