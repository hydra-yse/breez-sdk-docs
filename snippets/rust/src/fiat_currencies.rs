use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use breez_sdk_core::*;

async fn list_supported_fiat_currencies(sdk: Arc<BreezServices>) -> Result<()> {
    // ANCHOR: list-fiat-currencies
    let supported_fiat_currencies = sdk.list_fiat_currencies().await?;
    // ANCHOR_END: list-fiat-currencies

    Ok(())
}

async fn get_current_rates(sdk: Arc<BreezServices>) -> Result<()> {
    // ANCHOR: fetch-fiat-rates
    let fiat_rates = sdk.fetch_fiat_rates().await?;
    // ANCHOR_END: fetch-fiat-rates

    Ok(())
}

async fn get_fiat_currencies_and_rates(sdk: Arc<BreezServices>) -> Result<Vec<(FiatCurrency, Rate)>> {
    // ANCHOR: get-fiat-currencies-and-rates
    let supported_fiat_currencies = sdk.list_fiat_currencies().await?;
    let fiat_rates = sdk.fetch_fiat_rates().await?;

    let mut rates_map : HashMap<String, Rate> = HashMap::new();
    for rate in fiat_rates {
        rates_map.insert(rate.coin.to_string().to_lowercase(), rate);
    }

    let mut sorted = supported_fiat_currencies.clone();
    sorted.sort_by_key(|f| f.info.name.clone());

    let mut result : Vec<(FiatCurrency, Rate)> = Vec::new();
    for currency in sorted {
        let rate = rates_map.get(&currency.id.to_lowercase());
        if let Some(rate) = rate.cloned() {
            result.push((currency, rate));
        }
    }

    Ok(result)
    // ANCHOR_END: get-fiat-currencies-and-rates
}