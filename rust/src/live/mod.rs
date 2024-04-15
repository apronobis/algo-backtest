use std::collections::{HashMap, HashSet};

use crate::{
    connector::Connector,
    live::bot::{Bot, BotError, ErrorHandler},
    ty::Error,
    BuildError,
};
use crate::live::bot::OrderRecvHook;
use crate::ty::Order;

pub mod bot;

#[derive(Clone)]
pub struct AssetInfo {
    pub asset_no: usize,
    pub symbol: String,
    pub tick_size: f32,
    pub lot_size: f32,
}

/// Live [`Bot`] builder.
pub struct BotBuilder {
    conns: HashMap<String, Box<dyn Connector + Send + 'static>>,
    assets: Vec<(String, AssetInfo)>,
    error_handler: Option<ErrorHandler>,
    order_hook: Option<OrderRecvHook>,
}

impl BotBuilder {
    /// Registers a [`Connector`] with a specified name.
    /// The specified name for this connector is used when using [`BotBuilder::add`] to add an
    /// asset for trading through this connector.
    pub fn register<C>(mut self, name: &str, conn: C) -> Self
    where
        C: Connector + Send + 'static,
    {
        self.conns.insert(name.to_string(), Box::new(conn));
        self
    }

    /// Adds an asset.
    ///
    /// * `name` - Name of the [`Connector`], which is registered by [`BotBuilder::register`],
    ///            through which this asset will be traded.
    /// * `symbol` - Symbol of the asset. You need to check with the [`Connector`] which symbology
    ///              is used.
    /// * `tick_size` - The minimum price fluctuation.
    /// * `lot_size` -  The minimum trade size.
    pub fn add(mut self, name: &str, symbol: &str, tick_size: f32, lot_size: f32) -> Self {
        let asset_no = self.assets.len();
        self.assets.push((
            name.to_string(),
            AssetInfo {
                asset_no,
                symbol: symbol.to_string(),
                tick_size,
                lot_size,
            },
        ));
        self
    }

    /// Registers the error handler to deal with an error from connectors.
    pub fn error_handler<Handler>(mut self, handler: Handler) -> Self
    where
        Handler: FnMut(Error) -> Result<(), BotError> + 'static,
    {
        self.error_handler = Some(Box::new(handler));
        self
    }

    /// Registers the order response receive hook.
    pub fn order_recv_hook<Hook>(mut self, hook: Hook) -> Self
        where
            Hook: FnMut(&Order<()>, &Order<()>) -> Result<(), BotError> + 'static,
    {
        self.order_hook = Some(Box::new(hook));
        self
    }

    /// Builds a live [`Bot`] based on the registered connectors and assets.
    pub fn build(self) -> Result<Bot, BuildError> {
        let mut dup = HashSet::new();
        let mut conns = self.conns;
        for (an, (name, asset_info)) in self.assets.iter().enumerate() {
            if !dup.insert(format!("{}/{}", name, asset_info.symbol)) {
                Err(BuildError::Duplicate(
                    name.clone(),
                    asset_info.symbol.clone(),
                ))?;
            }
            let conn = conns
                .get_mut(name)
                .ok_or(BuildError::ConnectorNotFound(name.to_string()))?;
            conn.add(
                an,
                asset_info.symbol.clone(),
                asset_info.tick_size,
                asset_info.lot_size,
            )?;
        }

        let con = Bot::new(conns, self.assets, self.error_handler, self.order_hook);
        Ok(con)
    }
}
