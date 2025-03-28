use rocket::serde::json::Json;

use bambangshop_receiver::Result;
use crate::model::notification::Notification;
use crate::model::subscriber::SubscriberRequest;
use crate::service::notification::NotificationService;

#[get("/subscribe/<product_type>")]
pub fn subscribe(product_type: &str) -> Result<Json<SubscriberRequest>> {
    return match NotificationService::subscribe(product_type) {
        Ok(s) => Ok(Json::from(s)),
        Err(e) => Err(e)
    };
}