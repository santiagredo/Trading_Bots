use std::marker::PhantomData;

use models::entities::ledgers::Model;

use crate::{
    types::Ledgers,
    utils::{Data, Logic},
};

impl Ledgers<Logic> {
    pub fn insert_ledger(ledger: Model) -> Result<Ledgers<Data>, String> {
        // if ledger.order_id <= 0 {
        //     return Err(format!("Invalid order ID"));
        // }

        if ledger.record_type_id <= 0 {
            return Err(format!("Invalid record type ID"));
        }

        if ledger.base_asset_id <= 0 {
            return Err(format!("Invalid base asset ID"));
        }

        if ledger.quote_asset_id <= 0 {
            return Err(format!("Invalid quote asset ID"));
        }

        Ok(Ledgers {
            phase: PhantomData::<Data>,
            model: ledger,
        })
    }

    pub fn select_ledger(id: i32) -> Result<(), String> {
        if id <= 0 {
            return Err(format!("Invalid ledger id"));
        }

        Ok(())
    }

    pub fn select_ledgers() -> Result<(), ()> {
        Ok(())
    }
}
