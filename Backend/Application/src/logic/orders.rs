use std::marker::PhantomData;

use models::entities::orders::Model;
use sea_orm::prelude::Decimal;

use crate::{
    types::Orders,
    utils::{Data, Logic},
};

impl Orders<Logic> {
    pub fn insert_order(mut order: Model) -> Result<Orders<Data>, String> {
        if order.status_id <= 0 {
            return Err(format!("Invalid status ID"));
        }

        if order.strategy_id <= 0 {
            return Err(format!("Invalid strategy ID"));
        }

        if order.base_asset_id <= 0 {
            return Err(format!("Invalid base asset ID"));
        }

        if order.quote_asset_id <= 0 {
            return Err(format!("Invalid quote asset ID"));
        }

        if order.price_target < Decimal::ZERO {
            return Err(format!("Invalid target price -- 0"));
        }

        match order.is_sell {
            true => {
                if order.base_asset_amount <= Decimal::ZERO
                    && order.quote_asset_amount <= Decimal::ZERO
                {
                    return Err(format!("Invalid amounts"));
                }

                if order.quote_asset_amount == Decimal::ZERO {
                    order.quote_asset_amount = order.base_asset_amount * order.price_target;
                }

                if order.price_abort != Decimal::ZERO && order.price_abort >= order.price_target {
                    return Err(format!("Invalid abort price"));
                }

                if (order.price_target <= Decimal::ZERO && order.status_id == 1)
                    || order.price_entry > order.price_target
                {
                    return Err(format!("Invalid sell price target"));
                }
            }
            false => {
                if order.base_asset_amount <= Decimal::ZERO
                    && order.quote_asset_amount <= Decimal::ZERO
                {
                    return Err(format!("Invalid amounts"));
                }

                if order.base_asset_amount == Decimal::ZERO {
                    order.base_asset_amount = order.quote_asset_amount / order.price_target;
                }

                if order.price_abort != Decimal::ZERO && order.price_abort <= order.price_target {
                    return Err(format!("Invalid abort price"));
                }

                if (order.price_target <= Decimal::ZERO && order.status_id == 1)
                    || order.price_entry < order.price_target
                {
                    return Err(format!("Invalid buy price target"));
                }
            }
        }

        Ok(Orders {
            phase: PhantomData::<Data>,
            model: order,
        })
    }

    pub fn select_order(id: i32) -> Result<(), String> {
        if id <= 0 {
            return Err(format!("Invalid order ID"));
        }

        Ok(())
    }

    pub fn update_order(order: Model) -> Result<Orders<Data>, String> {
        if order.id <= 0 {
            return Err(format!("Invalid order ID"));
        }

        if ![2, 3].contains(&order.status_id) {
            return Err(format!("Invalid status ID"));
        }

        Ok(Orders {
            phase: PhantomData::<Data>,
            model: order,
        })
    }
}
