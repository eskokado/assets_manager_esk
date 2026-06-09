use shared_kernel::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TradeListItem {
    id: String,
    side: String,
    ticker: String,
    quantity: String,
    unit_price: String,
    total: String,
    traded_at: String,
}

impl TradeListItem {
    pub fn try_new(
        id: String,
        side: String,
        ticker: String,
        quantity: String,
        unit_price: String,
        total: String,
        traded_at: String,
    ) -> Result<Self> {
        if id.trim().is_empty() {
            return Result::err("Trade id is required");
        }
        Result::ok(Self {
            id,
            side,
            ticker,
            quantity,
            unit_price,
            total,
            traded_at,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn side(&self) -> &str {
        &self.side
    }

    pub fn ticker(&self) -> &str {
        &self.ticker
    }

    pub fn quantity(&self) -> &str {
        &self.quantity
    }

    pub fn unit_price(&self) -> &str {
        &self.unit_price
    }

    pub fn total(&self) -> &str {
        &self.total
    }

    pub fn traded_at(&self) -> &str {
        &self.traded_at
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuyTradeForm {
    asset_id: String,
    quantity: String,
    unit_price: String,
}

impl BuyTradeForm {
    pub fn try_new(asset_id: String, quantity: String, unit_price: String) -> Result<Self> {
        if asset_id.trim().is_empty() {
            return Result::err("Asset is required");
        }
        if quantity.trim().is_empty() {
            return Result::err("Quantity is required");
        }
        if unit_price.trim().is_empty() {
            return Result::err("Unit price is required");
        }
        Result::ok(Self {
            asset_id,
            quantity,
            unit_price,
        })
    }

    pub fn asset_id(&self) -> &str {
        &self.asset_id
    }

    pub fn quantity(&self) -> &str {
        &self.quantity
    }

    pub fn unit_price(&self) -> &str {
        &self.unit_price
    }

    pub fn set_asset_id(&mut self, asset_id: String) {
        self.asset_id = asset_id;
    }

    pub fn set_quantity(&mut self, quantity: String) {
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: String) {
        self.unit_price = unit_price;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SellTradeForm {
    asset_id: String,
    quantity: String,
    unit_price: String,
    max_quantity: String,
}

impl SellTradeForm {
    pub fn try_new(
        asset_id: String,
        quantity: String,
        unit_price: String,
        max_quantity: String,
    ) -> Result<Self> {
        if asset_id.trim().is_empty() {
            return Result::err("Asset is required");
        }
        if quantity.trim().is_empty() {
            return Result::err("Quantity is required");
        }
        if unit_price.trim().is_empty() {
            return Result::err("Unit price is required");
        }
        Result::ok(Self {
            asset_id,
            quantity,
            unit_price,
            max_quantity,
        })
    }

    pub fn asset_id(&self) -> &str {
        &self.asset_id
    }

    pub fn quantity(&self) -> &str {
        &self.quantity
    }

    pub fn unit_price(&self) -> &str {
        &self.unit_price
    }

    pub fn max_quantity(&self) -> &str {
        &self.max_quantity
    }

    pub fn set_asset_id(&mut self, asset_id: String) {
        self.asset_id = asset_id;
    }

    pub fn set_quantity(&mut self, quantity: String) {
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: String) {
        self.unit_price = unit_price;
    }

    pub fn set_max_quantity(&mut self, max_quantity: String) {
        self.max_quantity = max_quantity;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_trade_list_item() {
        let item = TradeListItem::try_new(
            "1".into(),
            "BUY".into(),
            "PETR4".into(),
            "10".into(),
            "5".into(),
            "50".into(),
            "2026-01-01".into(),
        )
        .unwrap();
        assert_eq!(item.ticker(), "PETR4");
    }
}
