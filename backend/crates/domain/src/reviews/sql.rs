pub const REVIEW_COLUMNS: &str = r#"stay_reviews.id, stay_reviews."reservationId" AS reservation_id,
    stay_reviews."propertyId" AS property_id, stay_reviews."operatorId" AS operator_id,
    stay_reviews.rating, stay_reviews.comment, stay_reviews.status,
    stay_reviews."operatorReply" AS operator_reply,
    stay_reviews."operatorRepliedAt" AS operator_replied_at,
    stay_reviews."createdAt" AS created_at,
    (SELECT i."unitTypeId" FROM stay_reservation_items i
      WHERE i."reservationId" = stay_reviews."reservationId" LIMIT 1) AS unit_type_id,
    (SELECT t."nameEn" FROM stay_reservation_items i JOIN stay_unit_types t ON t.id = i."unitTypeId"
      WHERE i."reservationId" = stay_reviews."reservationId" LIMIT 1) AS unit_type_name_en,
    (SELECT t."nameFr" FROM stay_reservation_items i JOIN stay_unit_types t ON t.id = i."unitTypeId"
      WHERE i."reservationId" = stay_reviews."reservationId" LIMIT 1) AS unit_type_name_fr"#;
