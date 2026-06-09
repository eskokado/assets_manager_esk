use std::sync::Arc;

use shared_kernel::Result;

use crate::features::assets::domain::{AssetForm, AssetListItem};
use crate::features::assets::ports::{AssetRepository, ListAssetsQuery, PaginatedAssetList};

pub struct ListAssetsUseCase {
    repository: Arc<dyn AssetRepository>,
}

impl ListAssetsUseCase {
    pub fn new(repository: Arc<dyn AssetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        access_token: &str,
        query: ListAssetsQuery,
    ) -> Result<PaginatedAssetList> {
        if access_token.trim().is_empty() {
            return Result::err("Access token is required");
        }
        self.repository.list(access_token, query).await
    }
}

pub struct LoadAssetUseCase {
    repository: Arc<dyn AssetRepository>,
}

impl LoadAssetUseCase {
    pub fn new(repository: Arc<dyn AssetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, access_token: &str, id: &str) -> Result<AssetForm> {
        if access_token.trim().is_empty() {
            return Result::err("Access token is required");
        }
        if id.trim().is_empty() {
            return Result::err("Asset id is required");
        }
        self.repository.load(access_token, id).await
    }
}

pub struct SaveAssetUseCase {
    repository: Arc<dyn AssetRepository>,
}

impl SaveAssetUseCase {
    pub fn new(repository: Arc<dyn AssetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, access_token: &str, form: &AssetForm) -> Result<AssetListItem> {
        if access_token.trim().is_empty() {
            return Result::err("Access token is required");
        }
        match AssetForm::try_new(
            form.id().map(str::to_string),
            form.ticker().to_string(),
            form.name().to_string(),
            form.asset_type().to_string(),
            form.currency().to_string(),
            form.active(),
        ) {
            shared_kernel::Result::Ok(_) => {}
            shared_kernel::Result::Err(errors) => return shared_kernel::Result::Err(errors),
        }
        self.repository.save(access_token, form).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use shared_kernel::Result;

    use super::*;
    use crate::features::assets::domain::AssetListItem;

    struct MockRepo;

    #[async_trait]
    impl AssetRepository for MockRepo {
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
                limit: 20,
            })
        }

        async fn load(&self, _access_token: &str, id: &str) -> Result<AssetForm> {
            AssetForm::try_new(
                Some(id.to_string()),
                "PETR4".into(),
                "Petrobras".into(),
                "STOCK".into(),
                "BRL".into(),
                true,
            )
        }

        async fn save(&self, _access_token: &str, form: &AssetForm) -> Result<AssetListItem> {
            if form.ticker() == "DUP" {
                return Result::err("Ticker already exists");
            }
            AssetListItem::try_new(
                "1".into(),
                form.ticker().into(),
                form.name().into(),
                form.asset_type().into(),
                form.active(),
            )
        }
    }

    #[tokio::test]
    async fn list_assets_use_case_returns_items() {
        let use_case = ListAssetsUseCase::new(Arc::new(MockRepo));
        let result = use_case
            .execute(
                "token",
                ListAssetsQuery {
                    page: None,
                    limit: None,
                    asset_type: None,
                    active: None,
                    search: None,
                },
            )
            .await
            .unwrap();
        assert_eq!(result.items.len(), 1);
    }

    #[tokio::test]
    async fn load_asset_use_case_returns_form() {
        let use_case = LoadAssetUseCase::new(Arc::new(MockRepo));
        let form = use_case.execute("token", "1").await.unwrap();
        assert_eq!(form.ticker(), "PETR4");
    }

    #[tokio::test]
    async fn save_asset_use_case_handles_duplicate_ticker() {
        let use_case = SaveAssetUseCase::new(Arc::new(MockRepo));
        let form = AssetForm::try_new(
            None,
            "DUP".into(),
            "Duplicate".into(),
            "STOCK".into(),
            "BRL".into(),
            true,
        )
        .unwrap();
        assert!(use_case.execute("token", &form).await.is_err());
    }
}
