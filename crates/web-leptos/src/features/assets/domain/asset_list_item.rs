use shared_kernel::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetListItem {
    id: String,
    ticker: String,
    name: String,
    asset_type: String,
    active: bool,
}

impl AssetListItem {
    pub fn try_new(
        id: String,
        ticker: String,
        name: String,
        asset_type: String,
        active: bool,
    ) -> Result<Self> {
        if id.trim().is_empty() {
            return Result::err("Asset id is required");
        }
        if ticker.trim().is_empty() {
            return Result::err("Ticker is required");
        }
        if name.trim().is_empty() {
            return Result::err("Asset name is required");
        }
        Result::ok(Self {
            id,
            ticker,
            name,
            asset_type,
            active,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn ticker(&self) -> &str {
        &self.ticker
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn asset_type(&self) -> &str {
        &self.asset_type
    }

    pub fn active(&self) -> bool {
        self.active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_list_item() {
        let item = AssetListItem::try_new(
            "1".into(),
            "PETR4".into(),
            "Petrobras".into(),
            "STOCK".into(),
            true,
        )
        .unwrap();
        assert_eq!(item.ticker(), "PETR4");
    }
}
