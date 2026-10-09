//! Linux desktop colour scheme, including Wayland compositors that do not report a winit theme.

#[cfg(target_os = "linux")]
fn portal_theme() -> Option<egui::Theme> {
    let conn = zbus::blocking::connection::Builder::session().ok()?.method_timeout(std::time::Duration::from_secs(2)).build().ok()?;
    let proxy =
        zbus::blocking::Proxy::new(&conn, "org.freedesktop.portal.Desktop", "/org/freedesktop/portal/desktop", "org.freedesktop.portal.Settings").ok()?;
    let value: zbus::zvariant::OwnedValue = proxy.call("Read", &("org.freedesktop.appearance", "color-scheme")).ok()?;
    let code = portal_code(value)?;
    decode(code)
}

#[cfg(target_os = "linux")]
fn portal_code(value: zbus::zvariant::OwnedValue) -> Option<u32> {
    let mut value: zbus::zvariant::Value<'_> = value.into();
    // The Settings.Read result is a variant; some portals nest another variant inside it.
    for _ in 0..4 {
        match value {
            zbus::zvariant::Value::Value(inner) => value = *inner,
            other => return u32::try_from(other).ok(),
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn decode(code: u32) -> Option<egui::Theme> {
    match code {
        1 => Some(egui::Theme::Dark),
        2 => Some(egui::Theme::Light),
        _ => None,
    }
}

#[cfg(target_os = "linux")]
fn gtk_theme() -> Option<egui::Theme> {
    let output = std::process::Command::new("gsettings").args(["get", "org.gnome.desktop.interface", "color-scheme"]).output().ok()?;
    if !output.status.success() {
        return None;
    }
    parse_gtk_scheme(std::str::from_utf8(&output.stdout).ok()?)
}

#[cfg(target_os = "linux")]
fn parse_gtk_scheme(s: &str) -> Option<egui::Theme> {
    match s.trim().trim_matches('\'') {
        "prefer-dark" => Some(egui::Theme::Dark),
        "prefer-light" => Some(egui::Theme::Light),
        _ => None,
    }
}

pub fn service() -> Option<photocraft_ui_egui::SystemThemeFn> {
    #[cfg(target_os = "linux")]
    {
        use std::sync::{
            Arc,
            atomic::{AtomicU8, Ordering},
        };
        let value = Arc::new(AtomicU8::new(0));
        let worker_value = Arc::clone(&value);
        let (ready, wait) = std::sync::mpsc::channel();
        if std::thread::Builder::new()
            .name("appearance-portal".into())
            .spawn(move || {
                loop {
                    let code = match portal_theme().or_else(gtk_theme) {
                        Some(egui::Theme::Dark) => 1,
                        Some(egui::Theme::Light) => 2,
                        None => 0,
                    };
                    worker_value.store(code, Ordering::Relaxed);
                    let _ = ready.send(());
                    std::thread::sleep(std::time::Duration::from_secs(2));
                }
            })
            .is_err()
        {
            return None;
        }
        let _ = wait.recv_timeout(std::time::Duration::from_millis(250));
        Some(Box::new(move || decode(value.load(Ordering::Relaxed) as u32)))
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "linux")]
    #[test]
    fn portal_scheme_values() {
        assert_eq!(super::decode(0), None);
        assert_eq!(super::decode(1), Some(egui::Theme::Dark));
        assert_eq!(super::decode(2), Some(egui::Theme::Light));
        assert_eq!(super::decode(9), None);
        let nested = zbus::zvariant::Value::Value(Box::new(zbus::zvariant::Value::Value(Box::new(zbus::zvariant::Value::U32(2)))));
        assert_eq!(super::portal_code(zbus::zvariant::OwnedValue::try_from(nested).unwrap()), Some(2));
        assert_eq!(super::parse_gtk_scheme("'prefer-light'\n"), Some(egui::Theme::Light));
        assert_eq!(super::parse_gtk_scheme("'prefer-dark'\n"), Some(egui::Theme::Dark));
        assert_eq!(super::parse_gtk_scheme("'default'\n"), None);
    }
}
