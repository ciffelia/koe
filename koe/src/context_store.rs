use anyhow::{anyhow, Result};
use serenity::client::Context;
use serenity::prelude::TypeMapKey;
use serenity::Client;
use songbird::Songbird;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct ContextStore<T> {
    _marker: PhantomData<fn() -> T>,
}

impl<T> TypeMapKey for ContextStore<T>
where
    T: 'static + std::marker::Send + std::marker::Sync,
{
    type Value = Arc<T>;
}

pub async fn insert<T>(client: &Client, value: T)
where
    T: 'static + std::marker::Send + std::marker::Sync,
{
    let mut data = client.data.write().await;
    data.insert::<ContextStore<T>>(Arc::new(value));
}

pub async fn extract<T>(ctx: &Context) -> Result<Arc<T>>
where
    T: 'static + std::marker::Send + std::marker::Sync,
{
    let data = ctx.data.read().await;

    let extracted = data
        .get::<ContextStore<T>>()
        .ok_or_else(|| anyhow!("{} store is not initialized", std::any::type_name::<T>()))?;

    Ok(extracted.clone())
}

pub async fn extract_songbird(ctx: &Context) -> Result<Arc<Songbird>> {
    let songbird = songbird::get(ctx)
        .await
        .ok_or_else(|| anyhow!("Songbird voice client is not initialized"))?;

    Ok(songbird)
}
