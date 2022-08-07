use mac_notification_sys::*;

/// Sets up the default application that will receive the notifications.
pub fn set_default_application() {
    let bundle = get_bundle_identifier_or_default("com.apple.Terminal");
    set_application(&bundle).unwrap();
}

/// Sends a notification warning that the focus time is over.
pub fn send_rest_notification(time: &u8) {
    send_notification(
        "Rest",
        Some("Focus time is over!"),
        format!("Take a breath, we'll be back in {} minutes", time).as_str(),
        Some(Notification::new().sound("Blow").asynchronous(true)),
    )
    .unwrap();
}

/// Sends a notification warning that the rest time is over.
pub fn send_focus_notification(time: &u8) {
    send_notification(
        "Focus",
        Some("Get to work!"),
        format!("See you again in {} minutes", time).as_str(),
        Some(Notification::new().sound("Blow").asynchronous(true)),
    )
    .unwrap();
}
