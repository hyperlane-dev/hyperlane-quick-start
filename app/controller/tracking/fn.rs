use super::*;

#[request_body_json(tracking_opt: Tracking)]
pub async fn report_tracking_data(ctx: Context) {
    let tracking: Tracking = tracking_opt.unwrap_or_default();
}
