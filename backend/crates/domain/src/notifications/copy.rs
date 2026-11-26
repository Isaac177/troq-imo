pub struct Template {
    pub key: &'static str,
    pub title_en: &'static str,
    pub body_en: &'static str,
    pub title_fr: &'static str,
    pub body_fr: &'static str,
}

const fn tpl(
    key: &'static str,
    title_en: &'static str,
    body_en: &'static str,
    title_fr: &'static str,
    body_fr: &'static str,
) -> Template {
    Template { key, title_en, body_en, title_fr, body_fr }
}

pub static TEMPLATES: &[Template] = &[
    tpl(
        "stay.reservation_held.host",
        "Reservation to confirm",
        "{guestName} requested {propertyName}. Please respond.",
        "Réservation à confirmer",
        "{guestName} a demandé {propertyName}. Merci de répondre.",
    ),
    tpl(
        "stay.reservation_accepted.guest",
        "Reservation accepted",
        "Your stay at {propertyName} was accepted.",
        "Réservation acceptée",
        "Votre séjour à {propertyName} a été accepté.",
    ),
    tpl(
        "stay.reservation_cancelled.guest",
        "Reservation cancelled",
        "Your stay at {propertyName} was cancelled.",
        "Réservation annulée",
        "Votre séjour à {propertyName} a été annulé.",
    ),
    tpl(
        "stay.reservation_cancelled.host",
        "Reservation cancelled",
        "A reservation at {propertyName} was cancelled.",
        "Réservation annulée",
        "Une réservation à {propertyName} a été annulée.",
    ),
    tpl(
        "stay.damage_claimed.guest",
        "Damage claim",
        "A damage claim was filed for your stay at {propertyName}.",
        "Réclamation pour dommages",
        "Une réclamation pour dommages a été déposée pour votre séjour à {propertyName}.",
    ),
    tpl(
        "stay.amendment_proposed.guest",
        "Change proposed",
        "A change to your stay at {propertyName} was proposed.",
        "Modification proposée",
        "Une modification de votre séjour à {propertyName} a été proposée.",
    ),
    tpl(
        "stay.amendment_declined.host",
        "Change declined",
        "{guestName} declined the change for {propertyName}.",
        "Modification refusée",
        "{guestName} a refusé la modification pour {propertyName}.",
    ),
    tpl(
        "stay.amendment_accepted.host",
        "Change accepted",
        "{guestName} accepted the change for {propertyName}.",
        "Modification acceptée",
        "{guestName} a accepté la modification pour {propertyName}.",
    ),
    tpl(
        "stay.message.guest",
        "New message from your host",
        "{propertyName}: {preview}",
        "Nouveau message de votre hôte",
        "{propertyName} : {preview}",
    ),
    tpl(
        "stay.message.host",
        "New message from a guest",
        "{propertyName}: {preview}",
        "Nouveau message d'un voyageur",
        "{propertyName} : {preview}",
    ),
    tpl(
        "stay.review_submitted.host",
        "New stay review",
        "{propertyName} received a new review.",
        "Nouvel avis de séjour",
        "{propertyName} a reçu un nouvel avis.",
    ),
    tpl(
        "stay.review_replied.guest",
        "Host replied to your review",
        "The host of {propertyName} replied to your review.",
        "L'hôte a répondu à votre avis",
        "L'hôte de {propertyName} a répondu à votre avis.",
    ),
    tpl(
        "stay.special_request_decided.guest",
        "Special request answered",
        "Your request for {propertyName} was {decision}.",
        "Demande spéciale traitée",
        "Votre demande pour {propertyName} a été {decisionFr}.",
    ),
    tpl(
        "stay.property_created.admin",
        "New property",
        "{propertyName} was submitted.",
        "Nouvelle propriété",
        "{propertyName} a été soumise.",
    ),
    tpl(
        "stay.revision_submitted.admin",
        "Property revision",
        "A revision of {propertyName} awaits review.",
        "Révision de propriété",
        "Une révision de {propertyName} attend validation.",
    ),
    tpl(
        "stay.property_suspended.host",
        "Property suspended",
        "{propertyName} was suspended.",
        "Propriété suspendue",
        "{propertyName} a été suspendue.",
    ),
    tpl(
        "stay.property_reinstated.host",
        "Property reinstated",
        "{propertyName} is active again.",
        "Propriété réactivée",
        "{propertyName} est de nouveau active.",
    ),
    tpl(
        "stay.case_opened.admin",
        "Case opened",
        "A new stay case needs attention.",
        "Dossier ouvert",
        "Un nouveau dossier de séjour demande votre attention.",
    ),
    tpl(
        "stay.case_opened.host",
        "Case opened",
        "A case was opened on one of your properties.",
        "Dossier ouvert",
        "Un dossier a été ouvert sur l'une de vos propriétés.",
    ),
    tpl(
        "stay.case_closed.parties",
        "Case closed",
        "A case on your properties was closed.",
        "Dossier clôturé",
        "Un dossier sur vos propriétés a été clôturé.",
    ),
];

pub fn find(key: &str) -> Option<&'static Template> {
    TEMPLATES.iter().find(|template| template.key == key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notifications::catalog::EVENTS;

    #[test]
    fn every_event_has_copy_in_both_languages() {
        for event in EVENTS {
            let template = find(event.key).expect("copy for event");
            assert!(!template.title_en.is_empty() && !template.title_fr.is_empty());
            assert!(!template.body_en.is_empty() && !template.body_fr.is_empty());
        }
        assert_eq!(TEMPLATES.len(), EVENTS.len());
    }

    #[test]
    fn placeholders_match_between_languages() {
        for template in TEMPLATES {
            let en = placeholders(template.body_en);
            let fr = placeholders(template.body_fr).into_iter().map(|name| name.trim_end_matches("Fr").to_string()).collect::<Vec<_>>();
            for name in &en {
                assert!(fr.contains(&name.trim_end_matches("Fr").to_string()), "{}", template.key);
            }
        }
    }

    fn placeholders(text: &str) -> Vec<String> {
        text.split('{').skip(1).filter_map(|rest| rest.split('}').next()).map(str::to_string).collect()
    }
}

pub static REGISTRATION_OTP: Template = tpl(
    "identity.registration_otp",
    "Mungini verification code",
    "Your registration code is {code}.",
    "Code de vérification Mungini",
    "Votre code d'inscription est {code}.",
);


pub static HOST_INVITATION: Template = tpl(
    "identity.invitation.created",
    "Invitation to join a host team",
    "{host} has invited you to join their team on Mungini. Sign in with this email address, or register with it and choose {city} as your home city to view your invitation.",
    "Invitation à rejoindre une équipe",
    "{host} vous invite à rejoindre son équipe sur Mungini. Connectez-vous avec cette adresse e-mail, ou inscrivez-vous avec celle-ci en choisissant {city} comme ville de résidence pour consulter votre invitation.",
);
