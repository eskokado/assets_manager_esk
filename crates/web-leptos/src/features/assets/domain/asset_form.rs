use shared_kernel::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetForm {
    id: Option<String>,
    ticker: String,
    name: String,
    asset_type: String,
    currency: String,
    active: bool,
}

impl AssetForm {
    pub fn new_create() -> Self {
        Self {
            id: None,
            ticker: String::new(),
            name: String::new(),
            asset_type: "STOCK".into(),
            currency: "BRL".into(),
            active: true,
        }
    }

    pub fn try_new(
        id: Option<String>,
        ticker: String,
        name: String,
        asset_type: String,
        currency: String,
        active: bool,
    ) -> Result<Self> {
        if ticker.trim().is_empty() {
            return Result::err("Ticker is required");
        }
        if name.trim().is_empty() {
            return Result::err("Asset name is required");
        }
        if asset_type.trim().is_empty() {
            return Result::err("Asset type is required");
        }
        if currency.trim().is_empty() {
            return Result::err("Currency is required");
        }
        Result::ok(Self {
            id,
            ticker,
            name,
            asset_type,
            currency,
            active,
        })
    }

    pub fn is_edit(&self) -> bool {
        self.id.is_some()
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
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

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_ticker(&mut self, value: String) {
        self.ticker = value;
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }

    pub fn set_asset_type(&mut self, value: String) {
        self.asset_type = value;
    }

    pub fn set_currency(&mut self, value: String) {
        self.currency = value;
    }

    pub fn set_active(&mut self, value: bool) {
        self.active = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_create_has_defaults() {
        let form = AssetForm::new_create();
        assert!(!form.is_edit());
        assert_eq!(form.currency(), "BRL");
    }
}
