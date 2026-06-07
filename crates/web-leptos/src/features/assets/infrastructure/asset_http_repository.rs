use async_trait::async_trait;
use reqwest::Client;
use shared_kernel::{DomainError, Result};

use crate::features::assets::domain::{AssetForm, AssetListItem};
use crate::features::assets::infrastructure::dto::{
    AssetDto, CreateAssetRequestDto, ListAssetsResponseDto, UpdateAssetRequestDto,
};
use crate::features::assets::ports::{AssetRepository, ListAssetsQuery, PaginatedAssetList};
use crate::shared::config::api_base_url;

pub struct AssetHttpRepository {
    client: Client,
    base_url: String,
}

impl AssetHttpRepository {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: api_base_url(),
        }
    }

    async fn parse_error(res: reqwest::Response) -> Vec<DomainError> {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        vec![DomainError::new(format!("HTTP {status}: {body}"))]
    }

    fn map_asset(dto: AssetDto) -> Result<AssetListItem> {
        AssetListItem::try_new(dto.id, dto.ticker, dto.name, dto.asset_type, dto.active)
    }

    fn map_form(dto: AssetDto) -> Result<AssetForm> {
        AssetForm::try_new(
            Some(dto.id),
            dto.ticker,
            dto.name,
            dto.asset_type,
            dto.currency,
            dto.active,
        )
    }

    fn parse_url(base: &str) -> Result<reqwest::Url> {
        match reqwest::Url::parse(base) {
            Ok(url) => Result::ok(url),
            Err(e) => Result::fail(vec![DomainError::new(e.to_string())]),
        }
    }
}

impl Default for AssetHttpRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl AssetRepository for AssetHttpRepository {
    async fn list(&self, access_token: &str, query: ListAssetsQuery) -> Result<PaginatedAssetList> {
        let mut url = match Self::parse_url(&format!("{}/api/assets", self.base_url)) {
            shared_kernel::Result::Ok(value) => value,
            shared_kernel::Result::Err(errors) => return shared_kernel::Result::Err(errors),
        };
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(page) = query.page {
                pairs.append_pair("page", &page.to_string());
            }
            if let Some(limit) = query.limit {
                pairs.append_pair("limit", &limit.to_string());
            }
            if let Some(asset_type) = query.asset_type.as_deref() {
                pairs.append_pair("asset_type", asset_type);
            }
            if let Some(active) = query.active {
                pairs.append_pair("active", &active.to_string());
            }
            if let Some(search) = query.search.as_deref() {
                pairs.append_pair("search", search);
            }
        }

        let res = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {access_token}"))
            .send()
            .await;

        let res = match res {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        if !res.status().is_success() {
            return Result::fail(Self::parse_error(res).await);
        }

        let dto: ListAssetsResponseDto = match res.json().await {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        let mut items = Vec::with_capacity(dto.items.len());
        for item in dto.items {
            match Self::map_asset(item) {
                shared_kernel::Result::Ok(mapped) => items.push(mapped),
                shared_kernel::Result::Err(errors) => return shared_kernel::Result::Err(errors),
            }
        }

        Result::ok(PaginatedAssetList {
            items,
            total: dto.total,
            page: dto.page,
            limit: dto.limit,
        })
    }

    async fn load(&self, access_token: &str, id: &str) -> Result<AssetForm> {
        let url = format!("{}/api/assets/{id}", self.base_url);
        let res = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {access_token}"))
            .send()
            .await;

        let res = match res {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        if !res.status().is_success() {
            return Result::fail(Self::parse_error(res).await);
        }

        let dto: AssetDto = match res.json().await {
            Ok(value) => value,
            Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
        };

        Self::map_form(dto)
    }

    async fn save(&self, access_token: &str, form: &AssetForm) -> Result<AssetListItem> {
        if form.is_edit() {
            let id = form.id().expect("edit form id");
            let url = format!("{}/api/assets/{id}", self.base_url);
            let res = self
                .client
                .put(url)
                .header("Authorization", format!("Bearer {access_token}"))
                .json(&UpdateAssetRequestDto {
                    name: form.name().to_string(),
                    asset_type: form.asset_type().to_string(),
                    currency: form.currency().to_string(),
                    active: form.active(),
                })
                .send()
                .await;

            let res = match res {
                Ok(value) => value,
                Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
            };

            if !res.status().is_success() {
                return Result::fail(Self::parse_error(res).await);
            }

            let dto: AssetDto = match res.json().await {
                Ok(value) => value,
                Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
            };
            Self::map_asset(dto)
        } else {
            let url = format!("{}/api/assets", self.base_url);
            let res = self
                .client
                .post(url)
                .header("Authorization", format!("Bearer {access_token}"))
                .json(&CreateAssetRequestDto {
                    ticker: form.ticker().to_string(),
                    name: form.name().to_string(),
                    asset_type: form.asset_type().to_string(),
                    currency: Some(form.currency().to_string()),
                })
                .send()
                .await;

            let res = match res {
                Ok(value) => value,
                Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
            };

            if !res.status().is_success() {
                return Result::fail(Self::parse_error(res).await);
            }

            let dto: AssetDto = match res.json().await {
                Ok(value) => value,
                Err(e) => return Result::fail(vec![DomainError::new(e.to_string())]),
            };
            Self::map_asset(dto)
        }
    }
}
