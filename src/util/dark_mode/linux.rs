use dbus::blocking::{BlockingSender, Connection};
use dbus::Message;
use std::time::Duration;

pub fn is_dark_mode() -> bool {
    let conn = Connection::new_session().expect("Failed to connect to D-Bus");

    let msg = Message::new_method_call(
        "org.freedesktop.portal.Desktop",
        "/org/freedesktop/portal/desktop",
        "org.freedesktop.portal.Settings",
        "Read",
    )
    .expect("Failed to create D-Bus message")
    .append2("org.freedesktop.appearance", "color-scheme");

    let reply = conn
        .send_with_reply_and_block(msg, Duration::from_millis(1_000))
        .expect("Failed to send D-Bus message");

    let value: (u32,) = reply.get1().expect("Failed to get reply");

    // 0 = No preference, 1 = prefer-dark, 2 = prefer-light
    value.0 == 1
}
