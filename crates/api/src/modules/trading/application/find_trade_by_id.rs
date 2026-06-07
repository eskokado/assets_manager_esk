use std::sync::Arc;

use async_trait::async_trait;
use shared_kernel::{try_domain, Result, UseCase};

use crate::modules::trading::application::dto::{FindTradeInput, TradeOut};
use crate::modules::trading::domain::ports::TradeRepository;

pub struct FindTradeById {
    repository: Arc<dyn TradeRepository>,
}

impl FindTradeById {
    pub fn new(repository: Arc<dyn TradeRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UseCase<FindTradeInput, TradeOut> for FindTradeById {
    async fn execute(&self, input: FindTradeInput) -> Result<TradeOut> {
        let found = try_domain!(
            self.repository
                .find_by_id_for_user(input.trade_id, input.user_id)
                .await
        );

        match found {
            Some(item) => Result::ok(TradeOut::from_trade(&item.trade, Some(item.ticker))),
            None => Result::err("Trade not found"),
        }
    }
}
