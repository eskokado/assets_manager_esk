use chrono::{DateTime, Utc};
use shared_kernel::{try_domain, Result};
use sqlx::FromRow;
use uuid::Uuid;

use crate::modules::assets::domain::entity::AssetRecordData;
use crate::modules::assets::domain::{Asset, AssetName, AssetType, Currency, Ticker};

#[derive(Debug, Clone, FromRow)]
pub struct AssetRecord {
    pub id: Uuid,
    pub ticker: String,
    pub name: String,
    pub asset_type: String,
    pub currency: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<AssetRecordData> for AssetRecord {
    fn from(data: AssetRecordData) -> Self {
        Self {
            id: data.id,
            ticker: data.ticker,
            name: data.name,
            asset_type: data.asset_type,
            currency: data.currency,
            active: data.active,
            created_at: data.created_at,
            updated_at: data.updated_at,
        }
    }
}

impl AssetRecord {
    pub fn to_domain(&self) -> Result<Asset> {
        shared_kernel::Result::ok(Asset::reconstitute(
            self.id,
            try_domain!(Ticker::try_new(&self.ticker)),
            try_domain!(AssetName::try_new(&self.name)),
            try_domain!(AssetType::try_from_str(&self.asset_type)),
            try_domain!(Currency::try_new(&self.currency)),
            self.active,
            self.created_at,
            self.updated_at,
        ))
    }
}
