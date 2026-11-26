fn escape(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

pub struct NotificationMail<'a> {
    pub brand: &'a str,
    pub title_fr: &'a str,
    pub title_en: &'a str,
    pub body_fr: &'a str,
    pub body_en: &'a str,
    pub action_url: Option<&'a str>,
}

impl NotificationMail<'_> {
    pub fn subject(&self) -> String {
        format!("{} / {}", self.title_fr, self.title_en)
    }

    pub fn text(&self) -> String {
        format!("{}\n\n{}", self.body_fr, self.body_en)
    }

    pub fn html(&self) -> String {
        let action = self
            .action_url
            .map(|url| format!("<p><a href=\"{}\">{}</a></p>", escape(url), escape(self.title_en)))
            .unwrap_or_default();
        format!(
            "<!doctype html><html><body style=\"font-family:Arial,sans-serif;color:#18181b;line-height:1.5\"><h1 style=\"font-size:18px\">{}</h1><p>{}</p><hr><h2 style=\"font-size:15px\">{}</h2><p>{}</p>{}<p style=\"color:#71717a;font-size:12px\">{}</p></body></html>",
            escape(self.title_fr),
            escape(self.body_fr),
            escape(self.title_en),
            escape(self.body_en),
            action,
            escape(self.brand),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mail() -> NotificationMail<'static> {
        NotificationMail {
            brand: "Mungini",
            title_fr: "Réservation acceptée",
            title_en: "Reservation accepted",
            body_fr: "Votre séjour <b>a été</b> accepté.",
            body_en: "Your stay was accepted.",
            action_url: Some("https://app.example/fr/trips?reservation=r1&x=1"),
        }
    }

    #[test]
    fn subject_is_bilingual_and_text_has_both_bodies() {
        assert_eq!(mail().subject(), "Réservation acceptée / Reservation accepted");
        assert!(mail().text().contains("Your stay was accepted."));
    }

    #[test]
    fn html_escapes_content_and_links_the_action() {
        let html = mail().html();
        assert!(html.contains("&lt;b&gt;"));
        assert!(html.contains("reservation=r1&amp;x=1"));
        assert!(html.contains("Mungini"));
    }
}
