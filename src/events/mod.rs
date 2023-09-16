pub mod user;

use crate::{database::ampq::AmpqConnection, helpers::error_helper::AppError, ApplicationContext};
use async_trait::async_trait;
use futures_util::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties,
};
use log::debug;
use serde::{de::DeserializeOwned, Serialize};
use std::any::type_name;
use user::user_registered;

#[async_trait]
pub trait AppEvent: DeserializeOwned + Serialize {
    async fn init(conn: &AmpqConnection, ctx: ApplicationContext) {
        let queue_name = Self::name();
        let channel = conn.create_channel().await;
        if channel.is_err() {
            debug!("Unable to create channel {:?}", channel.err().unwrap());
            return;
        }
        let channel = channel.unwrap();
        let queue = channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await;

        if queue.is_err() {
            debug!(
                "Unable to declare queue({}) {:?}",
                queue_name,
                queue.err().unwrap()
            );
            return;
        }

        let queue = queue.unwrap();

        debug!("Declared queue {:?}", queue);

        let consumer = channel
            .basic_consume(
                queue_name,
                queue_name,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await;

        if consumer.is_err() {
            debug!("Unable to create consumer {:?}", consumer.err().unwrap());
            return;
        }

        let mut consumer = consumer.unwrap();

        debug!("Created consumer {:?}", consumer);
        async_global_executor::spawn(async move {
            while let Some(delivery) = consumer.next().await {
                if delivery.is_err() {
                    continue;
                }
                let delivery = delivery.unwrap();

                let this: Self = serde_json::from_slice(&delivery.data).unwrap();

                let handled = this.handle(ctx.clone());
                let handled = handled.await;
                if handled.is_err() {
                    continue;
                }

                let result = channel
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await;

                if result.is_err() {
                    continue;
                }
            }
        })
        .detach();
    }
    async fn publish(&self, conn: &AmpqConnection) -> Result<(), AppError> {
        let queue_name = Self::name();
        let serialized_data = serde_json::to_string(&self).map_err(|_| {
            AppError::internal_server("Unable to serialize data for publishing to RabbitMQ")
        })?;

        let payload = serialized_data.as_bytes();

        let channel = conn
            .create_channel()
            .await
            .map_err(|_| AppError::internal_server("Unable to find queue channel"))?;

        channel
            .basic_publish(
                "",
                queue_name,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await
            .map_err(|_| AppError::internal_server("Unable to publish message"))?;

        Ok(())
    }
    async fn handle(&self, ctx: ApplicationContext) -> Result<(), AppError>;

    fn name() -> &'static str {
        type_name::<Self>().split("::").last().unwrap()
    }
}

pub struct AppEvents;

impl AppEvents {
    pub async fn init(ctx: ApplicationContext) -> Result<(), AppError> {
        let conn = ctx.db.ampq.get_connection().await?;
        user_registered::UserRegistered::init(&conn, ctx).await;
        Ok(())
    }
}
