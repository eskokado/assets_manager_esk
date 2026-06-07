use rust_decimal::Decimal;
use shared_kernel::{Entity, EntityId};

use crate::modules::trading::domain::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Portfolio {
    user_id: EntityId,
    positions: Vec<Position>,
}

impl Entity for Portfolio {
    fn id(&self) -> &EntityId {
        &self.user_id
    }
}

impl Portfolio {
    pub fn from_positions(user_id: EntityId, positions: Vec<Position>) -> Self {
        let positions = positions
            .into_iter()
            .filter(|position| position.quantity().value() > Decimal::ZERO)
            .collect();
        Self { user_id, positions }
    }

    pub fn user_id(&self) -> EntityId {
        self.user_id
    }

    pub fn positions(&self) -> &[Position] {
        &self.positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::trading::domain::value_objects::{Quantity, UnitPrice};
    use rust_decimal_macros::dec;
    use uuid::Uuid;

    #[test]
    fn from_positions_keeps_open_positions() {
        let user_id = Uuid::new_v4();
        let asset_id = Uuid::new_v4();
        let open = Position::new(
            user_id,
            asset_id,
            Quantity::try_new(dec!(10)).unwrap(),
            UnitPrice::try_new(dec!(5), "BRL").unwrap(),
        );
        let portfolio = Portfolio::from_positions(user_id, vec![open.clone()]);
        assert_eq!(portfolio.positions().len(), 1);
        assert_eq!(portfolio.positions()[0].asset_id(), open.asset_id());
    }
}
