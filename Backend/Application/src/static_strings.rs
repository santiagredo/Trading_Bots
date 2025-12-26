pub static DEV_DATABASE_URL: &'static str = "postgres://postgres:postgres@localhost/trading_bots_dev";
pub static PROD_DATABASE_URL: &'static str = "postgres://postgres:postgres@localhost/trading_bots_prod";

// DEV_DATABASE_URL=sqlite://dev.db?mode=rwc
// PROD_DATABASE_URL=sqlite://prod.db?mode=rwc

pub static BINANCE_WEBSOCKET_STREAM_URL: &'static str = "wss://stream.binance.com:9443/ws";

pub static X_MBX_APIKEY: &'static str = "X-MBX-APIKEY";

pub static ORDERS_TEST_ENDPOINT: &'static str = "https://api1.binance.com/api/v3/order/test";
pub static ORDERS_ENDPOINT: &'static str = "https://api1.binance.com/api/v3/order";

pub static ACCOUNT_INFORMATION_ENDPOINT: &'static str = "https://api1.binance.com/api/v3/account";

pub static EXCHANGE_INFORMATION_ENDPOINT: &'static str =
    "https://api.binance.com/api/v3/exchangeInfo";

pub static COINPAPRIKA_TICKERS_ENDPOINT: &'static str = "https://api.coinpaprika.com/v1/tickers";
