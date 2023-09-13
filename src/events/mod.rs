pub mod users;

use crate::helpers::error::AppError;
use crate::ApplicationContext;

pub struct AppEvents;

impl AppEvents {
    pub async fn init(ctx: ApplicationContext) -> Result<(), AppError> {
        let conn = ctx.db.ampq.get_connection().await?;
        let a = conn.into_inner();
        users::UserRegistered::init(&a, ctx).await;
        Ok(())
    }
}
