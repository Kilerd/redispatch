use redis::aio::{Connection, ConnectionLike};
use redis::{cmd, AsyncCommands, FromRedisValue, RedisFuture, ToRedisArgs};
use serde::{Serialize, de::DeserializeOwned};

#[async_trait::async_trait]
trait JsonSerdeCommands: AsyncCommands {
    async fn get<'a, K, RV>(&'a mut self, key: K) -> Option<RV>
    where
        K: Serialize + Send + Sync + 'a,
        RV: DeserializeOwned,
    {
        let data: Option<Vec<u8>> = AsyncCommands::get(self, serde_json::to_string(&key).unwrap())
            .await
            .unwrap();
        if let Some(data) = data {
            Some(serde_json::from_slice(data.as_slice()).unwrap())
        } else {
            None
        }
    }

    async fn set<'a, K, V, RV>(&'a mut self, key: K, value: V) -> ()
    where
        K: Serialize + Send + Sync + 'a,
        V: Serialize + Send + Sync + 'a,
    {
        AsyncCommands::set::<_, _, ()>(
            self,
            serde_json::to_string(&key).unwrap(),
            serde_json::to_string(&value).unwrap(),
        )
        .await
        .unwrap();
    }
}