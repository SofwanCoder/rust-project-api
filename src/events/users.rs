use crate::emails::welcome_email::WelcomeEmail;
use crate::emails::Email;
use crate::helpers::error::AppError;
use crate::models::user::UserModel;
use crate::ApplicationContext;
use futures_util::StreamExt;
use lapin::{options::*, types::FieldTable, BasicProperties, Channel, Connection};
use log::debug;
use serde::Serialize;

const QUEUE_NAME: &str = "user-registered";
const CONSUMER_TAG: &str = "user-registered-consumer";
pub struct UserRegistered;

impl UserRegistered {
    pub async fn init(conn: &Connection, ctx: ApplicationContext) {
        let channel = conn.create_channel().await;
        if channel.is_err() {
            debug!("Unable to create channel {:?}", channel.err().unwrap());
            return;
        }
        let channel = channel.unwrap();
        let queue = channel
            .queue_declare(
                QUEUE_NAME,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await;

        if queue.is_err() {
            debug!("Unable to declare queue {:?}", queue.err().unwrap());
            return;
        }

        let queue = queue.unwrap();

        debug!("Declared queue {:?}", queue);

        Self::listen(channel, ctx).await;
    }

    async fn listen(channel: Channel, ctx: ApplicationContext) {
        let consumer = channel
            .basic_consume(
                QUEUE_NAME,
                CONSUMER_TAG,
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

                let user: UserModel = serde_json::from_slice(&delivery.data).unwrap();

                let welcome_email = WelcomeEmail {
                    to: user.email.clone(),
                    name: user.name.clone(),
                };

                let _ = welcome_email
                    .send(ctx.email.smtp.sender.clone())
                    .await
                    .map(|_| println!("Welcome email sent"))
                    .map_err(|_| {
                        AppError::internal_server("Failed to send welcome email".to_string())
                    });

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

    pub async fn publish<T: Serialize>(conn: Connection, data: T) -> Result<(), AppError> {
        let serialized_data = serde_json::to_string(&data).map_err(|_| {
            AppError::internal_server("Unable to serialize data for publishing to RabbitMQ")
        })?;

        let payload = serialized_data.as_bytes();

        let channel = conn
            .create_channel()
            .await
            .map_err(|_| AppError::internal_server("Queue error"))?;

        channel
            .basic_publish(
                "",
                QUEUE_NAME,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await
            .map_err(|_| AppError::internal_server("Unable to publish message"))?;

        Ok(())
    }
}
