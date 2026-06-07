use std::sync::Arc;

use shared_kernel::Result;

use crate::features::assets::domain::AssetListItem;
use crate::features::assets::ports::{AssetRepository, ListAssetsQuery};
use crate::features::trading::domain::{BuyTradeForm, SellTradeForm, TradeListItem};
use crate::features::trading::ports::{ListTradesQuery, PaginatedTradeList, TradeRepository};

pub struct ListTradesUseCase {
    repository: Arc<dyn TradeRepository>,
}

impl ListTradesUseCase {
    pub fn new(repository: Arc<dyn TradeRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        access_token: &str,
        query: ListTradesQuery,
    ) -> Result<PaginatedTradeList> {
        if access_token.trim().is_empty() {
            return Result::err("Access token is required");
        }
        self.repository.list(access_token, query).await
    }
}

pub struct BuyAssetUseCase {
    repository: Arc<dyn TradeRepository>,
}

impl BuyAssetUseCase {
    pub fn new(repository: Arc<dyn TradeRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, access_token: &str, form: &BuyTradeForm) -> Result<TradeListItem> {
        if access_token.trim().is_empty() {
            return Result::err("Access token is required");
        }
        match BuyTradeForm::try_new(
            form.asset_id().to_string(),
            form.quantity().to_string(),
            form.unit_price().to_string(),
        ) {
            shared_kernel::Result::Ok(_) => {}
            shared_kernel::Result::Err(errors) => return shared_kernel::Result::Err(errors),
        }
        self.repository.buy(access_token, form).await
    }
}

pub struct SellAssetUseCase {
    repository: Arc<dyn TradeRepository>,
}

impl SellAssetUseCase {
    pub fn new(repository: Arc<dyn TradeRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, access_token: &str, form: &SellTradeForm) -> Result<TradeListItem> {
        if access_token.trim().is_empty() {
            return Result::err("Access token is required");
        }
        if let Ok(qty) = form.quantity().parse::<f64>() {
            if let Ok(max) = form.max_quantity().parse::<f64>() {
                if qty > max {
                    return Result::err(format!(
                        "Insufficient balance: available {}",
                        form.max_quantity()
                    ));
                }
            }
        }
        match SellTradeForm::try_new(
            form.asset_id().to_string(),
            form.quantity().to_string(),
            form.unit_price().to_string(),
            form.max_quantity().to_string(),
        ) {
            shared_kernel::Result::Ok(_) => {}
            shared_kernel::Result::Err(errors) => return shared_kernel::Result::Err(errors),
        }
        self.repository.sell(access_token, form).await
    }
}

pub struct LoadActiveAssetsUseCase {
    repository: Arc<dyn AssetRepository>,
}

impl LoadActiveAssetsUseCase {
    pub fn new(repository: Arc<dyn AssetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, access_token: &str) -> Result<Vec<AssetListItem>> {
        if access_token.trim().is_empty() {
            return Result::err("Access token is required");
        }
        let page = match self
            .repository
            .list(
                access_token,
                ListAssetsQuery {
                    page: Some(1),
                    limit: Some(100),
                    asset_type: None,
                    active: Some(true),
                    search: None,
                },
            )
            .await
        {
            shared_kernel::Result::Ok(value) => value,
            shared_kernel::Result::Err(errors) => return shared_kernel::Result::Err(errors),
        };
        Result::ok(page.items)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use shared_kernel::Result;

    use super::*;
    use crate::features::assets::domain::AssetListItem;
    use crate::features::assets::ports::{AssetRepository, ListAssetsQuery, PaginatedAssetList};

    struct MockTradeRepo;

    #[async_trait]
    impl TradeRepository for MockTradeRepo {
        async fn list(
            &self,
            _access_token: &str,
            _query: ListTradesQuery,
        ) -> Result<PaginatedTradeList> {
            Result::ok(PaginatedTradeList {
                items: vec![TradeListItem::try_new(
                    "1".into(),
                    "BUY".into(),
                    "PETR4".into(),
                    "10".into(),
                    "5".into(),
                    "50".into(),
                    "2026-01-01".into(),
                )
                .unwrap()],
                total: 1,
            })
        }

        async fn buy(&self, _access_token: &str, form: &BuyTradeForm) -> Result<TradeListItem> {
            if form.asset_id() == "inactive" {
                return Result::err("Asset is inactive and cannot be purchased");
            }
            TradeListItem::try_new(
                "1".into(),
                "BUY".into(),
                "PETR4".into(),
                form.quantity().into(),
                form.unit_price().into(),
                "50".into(),
                "2026-01-01".into(),
            )
        }

        async fn sell(&self, _access_token: &str, form: &SellTradeForm) -> Result<TradeListItem> {
            if form.quantity().parse::<f64>().unwrap_or(0.0)
                > form.max_quantity().parse::<f64>().unwrap_or(0.0)
            {
                return Result::err(format!(
                    "Insufficient balance: available {}",
                    form.max_quantity()
                ));
            }
            TradeListItem::try_new(
                "2".into(),
                "SELL".into(),
                "PETR4".into(),
                form.quantity().into(),
                form.unit_price().into(),
                "25".into(),
                "2026-01-02".into(),
            )
        }
    }

    struct MockAssetRepo;

    #[async_trait]
    impl AssetRepository for MockAssetRepo {
        async fn list(
            &self,
            _access_token: &str,
            _query: ListAssetsQuery,
        ) -> Result<PaginatedAssetList> {
            Result::ok(PaginatedAssetList {
                items: vec![AssetListItem::try_new(
                    "1".into(),
                    "PETR4".into(),
                    "Petrobras".into(),
                    "STOCK".into(),
                    true,
                )
                .unwrap()],
                total: 1,
                page: 1,
                limit: 100,
            })
        }

        async fn load(
            &self,
            _access_token: &str,
            _id: &str,
        ) -> Result<crate::features::assets::domain::AssetForm> {
            unimplemented!()
        }

        async fn save(
            &self,
            _access_token: &str,
            _form: &crate::features::assets::domain::AssetForm,
        ) -> Result<AssetListItem> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn load_active_assets_use_case_returns_active_assets() {
        let use_case = LoadActiveAssetsUseCase::new(Arc::new(MockAssetRepo));
        let result = use_case.execute("token").await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].ticker(), "PETR4");
    }

    #[tokio::test]
    async fn list_trades_use_case_returns_items() {
        let use_case = ListTradesUseCase::new(Arc::new(MockTradeRepo));
        let result = use_case
            .execute("token", ListTradesQuery::default())
            .await
            .unwrap();
        assert_eq!(result.items.len(), 1);
    }

    #[tokio::test]
    async fn buy_use_case_rejects_inactive_asset() {
        let use_case = BuyAssetUseCase::new(Arc::new(MockTradeRepo));
        let form = BuyTradeForm::try_new("inactive".into(), "1".into(), "10".into()).unwrap();
        assert!(use_case.execute("token", &form).await.is_err());
    }

    #[tokio::test]
    async fn sell_use_case_rejects_excess_quantity() {
        let use_case = SellAssetUseCase::new(Arc::new(MockTradeRepo));
        let form = SellTradeForm::try_new("1".into(), "10".into(), "5".into(), "5".into()).unwrap();
        assert!(use_case.execute("token", &form).await.is_err());
    }
}
