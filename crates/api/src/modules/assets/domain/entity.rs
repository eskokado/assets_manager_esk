use chrono::{DateTime, Utc};
use shared_kernel::{combine2, Entity, EntityId, Result};

use super::value_objects::{AssetName, AssetType, Currency, Ticker};

#[derive(Debug, Clone)]
pub struct AssetRecordData {
    pub id: EntityId,
    pub ticker: String,
    pub name: String,
    pub asset_type: String,
    pub currency: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    id: EntityId,
    ticker: Ticker,
    name: AssetName,
    asset_type: AssetType,
    currency: Currency,
    active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Entity for Asset {
    fn id(&self) -> &EntityId {
        &self.id
    }
}

impl Asset {
    pub fn create(
        id: EntityId,
        ticker: Ticker,
        name: AssetName,
        asset_type: AssetType,
        currency: Currency,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            ticker,
            name,
            asset_type,
            currency,
            active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn reconstitute(
        id: EntityId,
        ticker: Ticker,
        name: AssetName,
        asset_type: AssetType,
        currency: Currency,
        active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            ticker,
            name,
            asset_type,
            currency,
            active,
            created_at,
            updated_at,
        }
    }

    pub fn validate_create(
        ticker: &str,
        name: &str,
        asset_type: &str,
        currency: Option<&str>,
    ) -> Result<(Ticker, AssetName, AssetType, Currency)> {
        let (ticker, name) = shared_kernel::try_domain!(combine2(
            Ticker::try_new(ticker),
            AssetName::try_new(name)
        ));
        let asset_type = shared_kernel::try_domain!(AssetType::try_from_str(asset_type));
        let currency = match currency {
            Some(value) => shared_kernel::try_domain!(Currency::try_new(value)),
            None => Currency::brl(),
        };
        shared_kernel::Result::ok((ticker, name, asset_type, currency))
    }

    pub fn validate_update(
        name: &str,
        asset_type: &str,
        currency: &str,
    ) -> Result<(AssetName, AssetType, Currency)> {
        let (name, asset_type) = shared_kernel::try_domain!(combine2(
            AssetName::try_new(name),
            AssetType::try_from_str(asset_type)
        ));
        let currency = shared_kernel::try_domain!(Currency::try_new(currency));
        shared_kernel::Result::ok((name, asset_type, currency))
    }

    pub fn update_details(
        &mut self,
        name: AssetName,
        asset_type: AssetType,
        currency: Currency,
    ) {
        self.name = name;
        self.asset_type = asset_type;
        self.currency = currency;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.updated_at = Utc::now();
    }

    pub fn set_active(&mut self, active: bool) {
        if active {
            self.activate();
        } else {
            self.deactivate();
        }
    }

    pub fn is_available_for_buy(&self) -> bool {
        self.active
    }

    pub fn ticker(&self) -> &Ticker {
        &self.ticker
    }

    pub fn name(&self) -> &AssetName {
        &self.name
    }

    pub fn asset_type(&self) -> AssetType {
        self.asset_type
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn to_record(&self) -> AssetRecordData {
        AssetRecordData {
            id: *self.id(),
            ticker: self.ticker().as_str().to_string(),
            name: self.name().as_str().to_string(),
            asset_type: self.asset_type().as_str().to_string(),
            currency: self.currency().as_str().to_string(),
            active: self.active(),
            created_at: self.created_at(),
            updated_at: self.updated_at(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn create_starts_active() {
        let asset = Asset::create(
            Uuid::new_v4(),
            Ticker::try_new("PETR4").unwrap(),
            AssetName::try_new("Petrobras").unwrap(),
            AssetType::Stock,
            Currency::brl(),
        );
        assert!(asset.active());
        assert!(asset.is_available_for_buy());
    }

    #[test]
    fn deactivate_blocks_buy() {
        let mut asset = Asset::create(
            Uuid::new_v4(),
            Ticker::try_new("PETR4").unwrap(),
            AssetName::try_new("Petrobras").unwrap(),
            AssetType::Stock,
            Currency::brl(),
        );
        asset.deactivate();
        assert!(!asset.is_available_for_buy());
    }
}
