export interface paths {
    "/api/admin/accommodation-categories": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_accommodation_categories_list"];
        put?: never;
        post: operations["admin_accommodation_categories_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/accommodation-categories/{code}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["admin_accommodation_categories_update"];
        trace?: never;
    };
    "/api/admin/accommodation-templates": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_accommodation_templates_list"];
        put?: never;
        post: operations["admin_accommodation_templates_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/accommodation-templates/{templateId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["admin_accommodation_templates_update"];
        trace?: never;
    };
    "/api/admin/accommodation-templates/{templateId}/media": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_template_media_list"];
        put?: never;
        post: operations["admin_template_media_add"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/accommodation-templates/{templateId}/media/{mediaId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete: operations["admin_template_media_remove"];
        options?: never;
        head?: never;
        patch: operations["admin_template_media_update"];
        trace?: never;
    };
    "/api/admin/accommodation-templates/{templateId}/media/{mediaId}/content": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_template_media_content"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/amenities": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_amenities_list"];
        put?: never;
        post: operations["admin_amenities_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/amenities/{code}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["admin_amenities_update"];
        trace?: never;
    };
    "/api/admin/amenity-groups": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_amenity_groups_list"];
        put?: never;
        post: operations["admin_amenity_groups_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/amenity-groups/{code}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["admin_amenity_groups_update"];
        trace?: never;
    };
    "/api/admin/blocks": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_inventory_blocks_list"];
        put?: never;
        post: operations["admin_inventory_blocks_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/blocks/{blockId}/remove": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_inventory_blocks_remove"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/calendar": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_inventory_calendar_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/cases": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_cases_list"];
        put?: never;
        post: operations["admin_cases_open"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/cases/assignees": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_case_assignees_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/cases/evidence/{evidenceId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_cases_evidence"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/cases/{caseId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_cases_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/cases/{caseId}/assign": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_cases_assign"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/cases/{caseId}/close": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_cases_close"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/cases/{caseId}/escalate": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_cases_escalate"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/claims": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_claims_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/claims/summary": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_claims_summary"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/claims/{caseId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_claims_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/claims/{caseId}/decision": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_claims_decide"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/claims/{caseId}/payout": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_claims_payout"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/claims/{caseId}/statement": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_claims_statement"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/commission/hosts/{hostId}/rate": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_commission_rate_current"];
        put?: never;
        post: operations["admin_commission_rate_set"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/commission/hosts/{hostId}/rate/history": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_commission_rate_history"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/credits/accounts": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_credit_accounts_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/credits/hosts/{hostId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_credit_detail"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/credits/hosts/{hostId}/adjust": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_credit_adjust"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/credits/hosts/{hostId}/ledger": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_credit_ledger"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/credits/hosts/{hostId}/topup": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_credit_topup"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/credits/summary": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_credit_summary"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/dashboard": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_dashboard_summary"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/dashboard/hosts": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_dashboard_hosts"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/documents": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_admin_documents"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/documents/previews/{kind}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_admin_document_preview"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/documents/{id}/download": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_admin_document_download"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/guests": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_guests_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/guests/{userId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_guest_detail"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/guests/{userId}/bookings": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_guest_bookings"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/host-applications": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_host_applications_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/host-applications/{applicationId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_host_application_detail"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/host-applications/{applicationId}/decision": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_host_application_decide"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/host-applications/{applicationId}/documents/{documentId}/review": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_host_document_review"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/hosts": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_hosts_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/hosts/{hostId}/status": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_host_status_change"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/inventory-defaults": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_inventory_defaults_update"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/listings/queue": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_listing_queue"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/listings/{revisionId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_listing_revision_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/listings/{revisionId}/decision": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_listing_decide"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/media/{mediaId}/content": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_property_media_content"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/notification-preferences": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_admin_preferences"];
        put: operations["notifications_admin_set_preference"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/notifications": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_admin_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/notifications/read-all": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["notifications_admin_mark_all_read"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/notifications/unread-count": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_admin_unread_count"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/notifications/{id}/read": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["notifications_admin_mark_read"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/properties": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_properties_list"];
        put?: never;
        post: operations["admin_properties_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/properties/{propertyId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_properties_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["admin_properties_update"];
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/lifecycle": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_properties_lifecycle"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/media": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_property_media_list"];
        put?: never;
        post: operations["admin_property_media_add"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/media/{mediaId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete: operations["admin_property_media_remove"];
        options?: never;
        head?: never;
        patch: operations["admin_property_media_update"];
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/price-preview": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_price_preview"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/promotions": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_promotions_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/promotions/{promotionId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["admin_promotions_update"];
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/promotions/{promotionId}/funding": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put: operations["admin_promotions_funding_set"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/rate-plans": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_rate_plans_list"];
        put?: never;
        post: operations["admin_rate_plans_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/rate-plans/{ratePlanId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["admin_rate_plans_update"];
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/rate-plans/{ratePlanId}/rules": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_rate_rules_list"];
        put?: never;
        post: operations["admin_rate_rules_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/rate-plans/{ratePlanId}/rules/{ruleId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete: operations["admin_rate_rules_remove"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/reinstate": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_properties_reinstate"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/revisions": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_properties_revisions"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/suspend": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_properties_suspend"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/unit-types": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_inventory_unit_types_list"];
        put?: never;
        post: operations["admin_inventory_unit_types_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/properties/{propertyId}/unit-types/{unitTypeId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete: operations["admin_inventory_unit_types_archive"];
        options?: never;
        head?: never;
        patch: operations["admin_inventory_unit_types_update"];
        trace?: never;
    };
    "/api/admin/push-tokens": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put: operations["notifications_admin_register_token"];
        post?: never;
        delete: operations["notifications_admin_remove_token"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/rate-policies": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_rate_policies_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/reports/summary": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_reports_summary"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/reservations": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_reservations_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/reservations/export": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_reservations_export"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/reservations/summary": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_reservations_summary"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/reservations/{reservationId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_reservations_detail"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/reservations/{reservationId}/cancel": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_reservations_cancel"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/reservations/{reservationId}/cancel-preview": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_reservations_cancel_preview"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/restrictions/bulk": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_inventory_restrictions_bulk"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/reviews": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["reviews_admin_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/reviews/moderation-queue": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["reviews_admin_queue"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/reviews/{reviewId}/moderation": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["reviews_admin_decide"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/settings": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_settings_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/settings/{key}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["admin_settings_update"];
        trace?: never;
    };
    "/api/admin/tasks": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_tasks_list"];
        put?: never;
        post: operations["admin_tasks_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/tasks/{taskId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["admin_tasks_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/admin/tasks/{taskId}/transition": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["admin_tasks_transition"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/amendments/{amendmentId}/accept": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["amendments_accept"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/amendments/{amendmentId}/decline": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["amendments_decline"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/cases": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["cases_open_guest"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/cases/evidence/{evidenceId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["cases_evidence_get_guest"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/cases/mine": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["cases_mine"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/cases/{caseId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["cases_get_guest"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/cases/{caseId}/close": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["cases_close_guest"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/cases/{caseId}/evidence": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["cases_evidence_list_guest"];
        put?: never;
        post: operations["cases_evidence_add_guest"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/documents": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_guest_documents"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/documents/{id}/download": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_guest_document_download"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/invitations": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["guest_invitations_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/invitations/{invitationId}/accept": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["guest_invitation_accept"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/notification-preferences": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_guest_preferences"];
        put: operations["notifications_guest_set_preference"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/notifications": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_guest_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/notifications/read-all": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["notifications_guest_mark_all_read"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/notifications/unread-count": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_guest_unread_count"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/notifications/{id}/read": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["notifications_guest_mark_read"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/profile": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["auth_profile_update"];
        trace?: never;
    };
    "/api/guest/push-tokens": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put: operations["notifications_guest_register_token"];
        post?: never;
        delete: operations["notifications_guest_remove_token"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/quotes": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["guest_quotes_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/quotes/{quoteId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["guest_quotes_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/reservations": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["reservations_hold"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/reservations/mine": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["reservations_mine"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/reservations/{reservationId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["reservations_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/reservations/{reservationId}/amendments": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["reservation_amendments_list"];
        put?: never;
        post: operations["reservation_amendments_propose"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/reservations/{reservationId}/cancel": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["reservations_cancel"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/reservations/{reservationId}/cancellation": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["reservations_cancellation"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/reservations/{reservationId}/messages": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["reservation_messages_list_guest"];
        put?: never;
        post: operations["reservation_messages_send_guest"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/reservations/{reservationId}/review": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["reviews_submit"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/reservations/{reservationId}/withdraw": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["reservations_withdraw"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/saved-searches": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["guests_saved_searches_list"];
        put?: never;
        post: operations["guests_saved_searches_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/saved-searches/{savedSearchId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete: operations["guests_saved_searches_delete"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/session": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["auth_session"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/session/logout": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["auth_logout"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/wishlists": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["guests_wishlists_list"];
        put?: never;
        post: operations["guests_wishlists_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/wishlists/{wishlistId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["guests_wishlists_update"];
        trace?: never;
    };
    "/api/guest/wishlists/{wishlistId}/items": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["guests_wishlist_items_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/guest/wishlists/{wishlistId}/items/{propertyId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["guests_wishlist_items_add"];
        delete: operations["guests_wishlist_items_remove"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/accommodation-templates": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_accommodation_templates_list"];
        put?: never;
        post: operations["host_accommodation_templates_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/accommodation-templates/{templateId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["host_accommodation_templates_update"];
        trace?: never;
    };
    "/api/host/accommodation-templates/{templateId}/media": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_template_media_list"];
        put?: never;
        post: operations["host_template_media_add"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/accommodation-templates/{templateId}/media/{mediaId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete: operations["host_template_media_remove"];
        options?: never;
        head?: never;
        patch: operations["host_template_media_update"];
        trace?: never;
    };
    "/api/host/accommodation-templates/{templateId}/media/{mediaId}/content": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_template_media_content"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/analytics/summary": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_analytics_summary"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/applications": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_applications_mine"];
        put?: never;
        post: operations["host_application_save"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/applications/{applicationId}/documents": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_application_document_add"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/applications/{applicationId}/documents/{documentId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete: operations["host_application_document_remove"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/applications/{applicationId}/submit": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_application_submit"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/applications/{applicationId}/withdraw": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_application_withdraw"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/availability": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["inventory_availability_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/blocks": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["inventory_blocks_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/blocks/{blockId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["inventory_blocks_get"];
        put?: never;
        post?: never;
        delete: operations["inventory_blocks_remove"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/calendar": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["inventory_calendar_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/calendar/feed-tokens": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["inventory_feed_tokens_list"];
        put?: never;
        post: operations["inventory_feed_tokens_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/calendar/feed-tokens/{tokenId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete: operations["inventory_feed_tokens_revoke"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/calendar/feed.ics": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["inventory_calendar_feed"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/calendar/inventory-default": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["inventory_defaults_update"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/cases": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["cases_list_host"];
        put?: never;
        post: operations["cases_open_host"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/cases/evidence/{evidenceId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["cases_evidence_get_host"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/cases/{caseId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["cases_get_host"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/cases/{caseId}/close": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["cases_close_host"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/cases/{caseId}/evidence": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["cases_evidence_list_host"];
        put?: never;
        post: operations["cases_evidence_add_host"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/credits": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_credits_summary"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/credits/ledger": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_credits_ledger"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/dashboard": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_dashboard_summary"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/documents": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_host_documents"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/documents/{id}/download": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_host_document_download"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/hosts": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_hosts_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/hosts/{hostId}/contact": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_contact_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["host_contact_update"];
        trace?: never;
    };
    "/api/host/hosts/{hostId}/invitations": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_invitations_list"];
        put?: never;
        post: operations["host_invitation_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/hosts/{hostId}/invitations/{invitationId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete: operations["host_invitation_revoke"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/hosts/{hostId}/members": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_members_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/hosts/{hostId}/members/{userId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete: operations["host_member_remove"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/hosts/{hostId}/members/{userId}/overrides": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put: operations["host_member_override_set"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/hosts/{hostId}/members/{userId}/overrides/{overrideId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete: operations["host_member_override_clear"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/hosts/{hostId}/members/{userId}/permissions": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put: operations["host_member_permissions_set"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/media/{mediaId}/content": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_property_media_content"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/message-conversations": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_message_conversations"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/notification-preferences": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_host_preferences"];
        put: operations["notifications_host_set_preference"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/notifications": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_host_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/notifications/read-all": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["notifications_host_mark_all_read"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/notifications/unread-count": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["notifications_host_unread_count"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/notifications/{id}/read": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["notifications_host_mark_read"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/price-preview": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_price_preview"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_properties_list"];
        put?: never;
        post: operations["host_properties_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_properties_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/blocks": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["inventory_blocks_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/lifecycle": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_property_lifecycle"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/media": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_property_media_list"];
        put?: never;
        post: operations["host_property_media_add"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/media/{mediaId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete: operations["host_property_media_remove"];
        options?: never;
        head?: never;
        patch: operations["host_property_media_update"];
        trace?: never;
    };
    "/api/host/properties/{propertyId}/promotions": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_promotions_list"];
        put?: never;
        post: operations["host_promotions_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/promotions/{promotionId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["host_promotions_update"];
        trace?: never;
    };
    "/api/host/properties/{propertyId}/rate-plans": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_rate_plans_list"];
        put?: never;
        post: operations["host_rate_plans_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/rate-plans/{ratePlanId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["host_rate_plans_update"];
        trace?: never;
    };
    "/api/host/properties/{propertyId}/rate-plans/{ratePlanId}/rules": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_rate_rules_list"];
        put?: never;
        post: operations["host_rate_rules_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/rate-plans/{ratePlanId}/rules/{ruleId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete: operations["host_rate_rules_remove"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/restrictions": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["inventory_restrictions_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/reviews": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["reviews_host_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/revisions": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_revisions_list"];
        put?: never;
        post: operations["host_revisions_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/revisions/{revisionId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_revisions_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch: operations["host_revisions_update"];
        trace?: never;
    };
    "/api/host/properties/{propertyId}/revisions/{revisionId}/readiness": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_revisions_readiness"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/revisions/{revisionId}/submit": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_revisions_submit"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/revisions/{revisionId}/withdraw": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_revisions_withdraw"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/unit-types": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["inventory_unit_types_list"];
        put?: never;
        post: operations["inventory_unit_types_create"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/properties/{propertyId}/unit-types/{unitTypeId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        delete: operations["inventory_unit_types_archive"];
        options?: never;
        head?: never;
        patch: operations["inventory_unit_types_update"];
        trace?: never;
    };
    "/api/host/push-tokens": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put: operations["notifications_host_register_token"];
        post?: never;
        delete: operations["notifications_host_remove_token"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/rate-policies": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_rate_policies_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reservation-items/{itemId}/unit-assignments": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["inventory_assignments_history"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reservation-items/{itemId}/unit-options": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["inventory_assignments_options"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reservations": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_reservations_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reservations/export": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_reservations_export"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reservations/{reservationId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_reservations_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reservations/{reservationId}/amendments": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_reservations_amend"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reservations/{reservationId}/cancel": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_reservations_cancel"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reservations/{reservationId}/check-in": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_reservations_check_in"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reservations/{reservationId}/check-out": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_reservations_check_out"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reservations/{reservationId}/decision": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_reservations_decide"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reservations/{reservationId}/messages": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["reservation_messages_list_host"];
        put?: never;
        post: operations["reservation_messages_send_host"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reservations/{reservationId}/messages/read": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_messages_mark_read"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reservations/{reservationId}/no-show": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_reservations_no_show"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reservations/{reservationId}/special-request": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["host_reservations_special_request"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/restrictions/bulk": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["inventory_restrictions_bulk"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/reviews/{reviewId}/reply": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["reviews_reply"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/tasks": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["operations_tasks_list_host"];
        put?: never;
        post: operations["operations_tasks_create_host"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/tasks/{taskId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["operations_tasks_get_host"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/tasks/{taskId}/transition": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["operations_tasks_transition_host"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/unit-assignments": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["inventory_assignments_assign"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/host/unit-assignments/{assignmentId}/move": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["inventory_assignments_move"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/accommodation-categories": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["public_accommodation_categories"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/amenities": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["amenities_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/amenity-groups": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["amenity_groups_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/auth/login": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["auth_login"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/auth/otp-channel": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["auth_otp_channel"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/auth/refresh": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["auth_refresh"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/auth/register": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["auth_register"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/auth/register/otp": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["auth_register_otp"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/auth/register/verify": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post: operations["auth_register_verify"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/calendar/feeds/{city}/{token}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["inventory_feed_by_token"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/cities": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["cities_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/city-context": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["city_context"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/destinations": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["guests_destinations_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/health": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["health_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/host-reference": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["host_reference"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/languages": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["languages_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/media/{mediaId}/content": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["guests_media_content"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/permission-catalog": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["permission_catalog"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/properties": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["guests_properties_search"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/properties/{propertyId}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["guests_property_get"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/properties/{propertyId}/availability": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["guests_property_availability"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/properties/{propertyId}/calendar": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["guests_property_calendar"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/properties/{propertyId}/reviews": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["reviews_public_list"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/public/search/catalog": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["guests_search_catalog"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
}
export type webhooks = Record<string, never>;
export interface components {
    schemas: {
        AccountRow: {
            balance: string;
            enabled: boolean;
            hostId: string;
            hostName: string;
            lastTopUpAt?: string | null;
            low: boolean;
            rateBps: number;
        };
        AddDocumentInput: {
            contentBase64: string;
            fileName: string;
            kind: string;
            mimeType: string;
        };
        AddEvidenceInput: {
            contentBase64: string;
            fileName: string;
            mimeType: string;
        };
        AddMediaInput: {
            altEn?: string;
            altFr?: string;
            clientKey: string;
            contentBase64: string;
            fileName: string;
            mimeType: string;
        };
        AdjustInput: {
            amount: string;
            direction: string;
            note?: string | null;
            reasonCode: string;
        };
        AdminApplicationView: {
            applicantEmail?: string | null;
            applicantName?: string | null;
            applicantUserId: string;
            cityId: string;
            createdAt: string;
            decidedAt?: string | null;
            displayName: string;
            hostId?: string | null;
            id: string;
            kind: string;
            reasonCode?: string | null;
            status: string;
            submittedAt?: string | null;
        };
        AdminCaseDetail: {
            evidence: components["schemas"]["EvidenceView"][];
            item: components["schemas"]["AdminCaseItem"];
        };
        AdminCaseItem: {
            assignedAt?: string | null;
            assignedToUserId?: string | null;
            caseType: string;
            claimAmount?: string | null;
            closedAt?: string | null;
            createdAt: string;
            deadlineAt: string;
            escalatedAt?: string | null;
            escalationReason?: string | null;
            id: string;
            isOverdue: boolean;
            operatorId: string;
            operatorName: string;
            parties: Record<string, never>;
            reason: string;
            reservationId?: string | null;
            reservationReference?: string | null;
            resolution?: Record<string, never> | null;
            state: string;
            version: number;
        };
        AdminEvidenceContent: {
            contentBase64: string;
            fileName: string;
            mimeType: string;
        };
        AdminHostView: {
            cityId: string;
            createdAt: string;
            displayName: string;
            guestContactEmail: string;
            guestContactPhone: string;
            id: string;
            kind: string;
            memberCount: number;
            propertyCount: number;
            status: string;
        };
        AdminLedgerRow: {
            amount: string;
            balanceAfter: string;
            createdAt: string;
            createdByUserId?: string | null;
            entryType: string;
            id: string;
            note?: string | null;
            paymentRef?: string | null;
            reasonCode?: string | null;
            reservationId?: string | null;
            reservationReference?: string | null;
        };
        AdminOpenCaseInput: {
            caseType: string;
            reason: string;
            reservationId: string;
        };
        AdminPromotionView: components["schemas"]["PromotionView"] & {
            fundedBy: string;
        };
        AdminPropertyDetail: components["schemas"]["PropertyView"] & {
            hostName: string;
            hostStatus: string;
        };
        AdminPropertyItem: components["schemas"]["PropertyFacts"] & {
            addressLine: string;
            city: string;
            hostId: string;
            hostName: string;
            hostStatus: string;
            id: string;
            nameEn: string;
            nameFr: string;
            neighborhood: string;
            openRevisions: number;
            publishedRevision: number;
            status: string;
            updatedAt: string;
            version: number;
        };
        AdminReviewItem: {
            city: string;
            comment?: string | null;
            createdAt: string;
            guestName?: string | null;
            id: string;
            moderatedAt?: string | null;
            operatorName: string;
            operatorRepliedAt?: string | null;
            operatorReply?: string | null;
            propertyId: string;
            propertyNameEn: string;
            propertyNameFr: string;
            rating: number;
            reservationReference: string;
            status: string;
            unitTypeCoverUrl?: string | null;
            unitTypeId?: string | null;
            unitTypeNameEn?: string | null;
            unitTypeNameFr?: string | null;
        };
        AdminRevisionView: {
            live: components["schemas"]["ListingContent"];
            revision: components["schemas"]["RevisionView"];
        };
        AdminTaskItem: {
            assigneeUserId?: string | null;
            checklist: string[];
            completedAt?: string | null;
            createdAt: string;
            id: string;
            operatorId: string;
            operatorName: string;
            priority: string;
            propertyId: string;
            propertyNameEn: string;
            propertyNameFr: string;
            reservationReference?: string | null;
            state: string;
            taskType: string;
            unitCode?: string | null;
            version: number;
        };
        AdminUpdateInput: {
            content: components["schemas"]["ListingContent"];
            expectedVersion: number;
        };
        AmendmentListItem: {
            checkIn: string;
            checkOut: string;
            createdAt: string;
            delta: string;
            expiresAt: string;
            id: string;
            nights: components["schemas"]["NightLine"][];
            respondedAt?: string | null;
            state: string;
            total: string;
        };
        AmendmentProposal: {
            checkIn: string;
            checkOut: string;
            delta: string;
            expiresAt: string;
            id: string;
            nights: components["schemas"]["NightLine"][];
            reservationId: string;
            state: string;
            total: string;
        };
        Amenity: {
            active: boolean;
            code: string;
            groupCode: string;
            icon: string;
            name: string;
            revision: number;
            sortOrder: number;
            updatedAt: string;
        };
        AmenityGroup: {
            active: boolean;
            amenityCount: number;
            code: string;
            name: string;
            revision: number;
            sortOrder: number;
            updatedAt: string;
        };
        AmenityWrite: {
            active?: boolean | null;
            groupCode: string;
            icon: string;
            nameEn: string;
            nameFr: string;
            sortOrder?: number | null;
        };
        AnalyticsSummary: {
            approvedReviews: number;
            averageRating?: number | null;
            cancelled: number;
            checkedIn: number;
            checkedOut: number;
            confirmed: number;
            currency: string;
            grossRevenue: string;
            noShow: number;
            reservations: number;
        };
        ApplicationFilter: {
            page: number | null;
            pageSize: number | null;
            search: string | null;
            status: string | null;
        };
        ApplicationInput: {
            displayName: string;
            kind: string;
        };
        ApplicationView: {
            applicantUserId: string;
            cityId: string;
            createdAt: string;
            decidedAt?: string | null;
            displayName: string;
            documents: components["schemas"]["HostDocumentView"][];
            hostId?: string | null;
            id: string;
            kind: string;
            reasonCode?: string | null;
            status: string;
            submittedAt?: string | null;
        };
        AssignCaseInput: {
            assigneeUserId?: string | null;
            expectedVersion: number;
        };
        AssignUnitInput: {
            physicalUnitId: string;
            reservationItemId: string;
        };
        AssignmentView: {
            createdAt: string;
            endDate: string;
            id: string;
            physicalUnitId: string;
            reason: string;
            releasedAt?: string | null;
            reservationItemId: string;
            startDate: string;
        };
        AvailabilityOptionView: {
            bookingMode: string;
            cancellation: components["schemas"]["CancellationView"];
            mealPlan: string;
            ratePlanId: string;
            remainingRooms?: number | null;
            total?: string | null;
            unitTypeId: string;
        };
        AvailabilityView: {
            checkIn: string;
            checkOut: string;
            currency: string;
            nights: number;
            options: components["schemas"]["AvailabilityOptionView"][];
            propertyId: string;
        };
        BalanceView: {
            balance: string;
            hostId: string;
        };
        BlockView: {
            createdAt: string;
            endDate: string;
            id: string;
            origin: string;
            propertyId: string;
            reason: string;
            removedAt?: string | null;
            startDate: string;
            unitTypeId?: string | null;
            version: number;
        };
        BulkRestrictionInput: {
            idempotencyKey?: string | null;
            operations: components["schemas"]["RestrictionOperation"][];
            propertyId: string;
        };
        BulkRestrictionResult: {
            affectedNights: number;
            applied: components["schemas"]["RestrictionView"][];
            removed: string[];
        };
        CalendarBlock: {
            endDate: string;
            id: string;
            physicalUnitId?: string | null;
            reason: string;
            startDate: string;
            unitTypeId?: string | null;
            version: number;
        };
        CalendarDay: {
            allocated: number;
            capacity: number;
            date: string;
            price?: string | null;
        };
        CalendarDayView: {
            available: boolean;
            date: string;
        };
        CalendarReservation: {
            adultCount: number;
            checkIn: string;
            checkOut: string;
            childCount: number;
            guestName?: string | null;
            id: string;
            pendingAmendment?: unknown;
            physicalUnitId?: string | null;
            reference: string;
            state: string;
            total: string;
            unitTypeCoverUrl?: string | null;
            unitTypeId: string;
            version: number;
        };
        CalendarUnit: {
            days: components["schemas"]["CalendarDay"][];
            defaultNightlyCapacity: boolean;
            inventoryModel: string;
            inventoryVersion?: number | null;
            nameEn: string;
            nameFr: string;
            unitTypeCoverUrl?: string | null;
            unitTypeId: string;
        };
        CalendarView: {
            days: components["schemas"]["CalendarDayView"][];
            unitTypeId: string;
        };
        CancelPreview: {
            canCancel: boolean;
            deadline?: string | null;
            policyPenalty?: string | null;
            policyReason?: string | null;
            policyRefund?: string | null;
            refund: string;
            reservationId: string;
            state: string;
            total: string;
            version: number;
        };
        CancelStayInput: {
            expectedVersion: number;
            idempotencyKey?: string | null;
        };
        CancellationView: {
            class: string;
            freeUntilHours?: number | null;
            penaltyPercent?: number | null;
        };
        CancelledStayView: {
            cancelledAt: string;
            chargeDueAtProperty: string;
            currency: string;
            reason: string;
            refund: string;
            reservationId: string;
            state: string;
        };
        CaseAssignee: {
            email?: string | null;
            name?: string | null;
            userId: string;
        };
        CaseView: {
            caseType: string;
            closedAt?: string | null;
            createdAt: string;
            deadlineAt: string;
            id: string;
            operatorId: string;
            parties: Record<string, never>;
            reason: string;
            reservationId?: string | null;
            reservationReference?: string | null;
            resolution?: Record<string, never> | null;
            state: string;
            version: number;
        };
        CatalogView: {
            permissions: components["schemas"]["PermissionView"][];
            presets: components["schemas"]["PresetView"][];
        };
        CategoryInput: {
            active?: boolean | null;
            icon: string;
            nameEn: string;
            nameFr: string;
            sortOrder?: number | null;
        };
        CategoryView: {
            active: boolean;
            code: string;
            icon: string;
            nameEn: string;
            nameFr: string;
            revision: number;
            sortOrder: number;
            unitTypeCount: number;
            updatedAt: string;
        };
        CityFigures: components["schemas"]["DashboardFigures"] & {
            city: string;
        };
        CityView: {
            code: string;
            name: string;
            timezone: string;
        };
        ClaimCore: {
            caseId: string;
            caseState: string;
            checkIn?: string | null;
            checkOut?: string | null;
            claimAmount: string;
            deadlineAt: string;
            guestName?: string | null;
            openedAt: string;
            operatorId: string;
            operatorName: string;
            propertyNameEn?: string | null;
            propertyNameFr?: string | null;
            reason: string;
            reservationId?: string | null;
            reservationReference?: string | null;
            stayTotal?: string | null;
            version: number;
        };
        ClaimDecisionResult: {
            approvedAmount: string;
            caseId: string;
            decision: string;
            payoutState: string;
        };
        ClaimDetail: components["schemas"]["ClaimCore"] & {
            decision?: null | components["schemas"]["DecisionRow"];
            evidence: components["schemas"]["EvidenceRow"][];
            statements: components["schemas"]["StatementRow"][];
        };
        ClaimRow: {
            approvedAmount?: string | null;
            caseId: string;
            caseState: string;
            checkOut?: string | null;
            claimAmount: string;
            deadlineAt: string;
            decision?: string | null;
            guestName?: string | null;
            openedAt: string;
            operatorId: string;
            operatorName: string;
            payoutState?: string | null;
            propertyNameEn?: string | null;
            propertyNameFr?: string | null;
            reservationReference?: string | null;
        };
        ClaimSummary: {
            approved: string;
            claimed: string;
            paid: string;
            payoutPending: string;
            total: number;
            undecided: number;
        };
        CloseCaseInput: {
            expectedVersion: number;
            resolution: string;
        };
        ConversationView: {
            lastAt: string;
            lastBody: string;
            propertyNameEn: string;
            propertyNameFr: string;
            reference: string;
            reservationId: string;
            timezone: string;
            unreadCount: number;
        };
        Core: {
            approvalDeadlineAt?: string | null;
            bookingMode: string;
            cancellationChargeAmount?: string | null;
            cancellationReason?: string | null;
            cancelledAt?: string | null;
            city: string;
            commissionAmount: string;
            commissionHeld: string;
            commissionRateBps: number;
            commissionRefunded: string;
            commissionTaken: string;
            createdAt: string;
            id: string;
            operatorId: string;
            operatorName: string;
            operatorNetAmount: string;
            paymentRef?: string | null;
            propertyId: string;
            propertyNameEn: string;
            propertyNameFr: string;
            reference: string;
            source: string;
            specialRequest?: string | null;
            state: string;
            totalAmount: string;
            updatedAt: string;
            version: number;
        };
        CoverView: {
            alt: string;
            contentPath: string;
            mediaId: string;
            url: string;
        };
        CreateBlockInput: {
            endDate: string;
            idempotencyKey?: string | null;
            propertyId: string;
            reason: string;
            startDate: string;
            unitTypeId: string;
        };
        CreateFeedTokenInput: {
            propertyId: string;
        };
        CreateInvitation: {
            email: string;
            permissions: string[];
        };
        CreatePlanInput: {
            bookingMode?: string;
            cancellationPolicyVersionId: string;
            childNightlyAmount?: string;
            idempotencyKey?: string | null;
            mealPlan?: string;
            occupancyBasis: string;
            paymentPolicyVersionId: string;
            unitTypeId: string;
        };
        CreatePromotionInput: {
            capAmount?: string | null;
            discountPercent: string;
            endDate?: string | null;
            idempotencyKey?: string | null;
            minNights: number;
            priority: number;
            ratePlanId?: string | null;
            redemptionLimit?: number | null;
            startDate?: string | null;
        };
        CreatePropertyInput: {
            addressLine?: string;
            descriptionEn?: string;
            descriptionFr?: string;
            hostId: string;
            idempotencyKey?: string | null;
            latitude?: number | null;
            longitude?: number | null;
            nameEn: string;
            nameFr: string;
            timezone: string;
        };
        CreateRevision: {
            content?: null | components["schemas"]["ListingContent"];
        };
        CreateRuleInput: {
            adjustmentType: string;
            amount: string;
            endDate?: string | null;
            idempotencyKey?: string | null;
            priority: number;
            startDate?: string | null;
            weekdays: number[];
        };
        CreateSavedSearch: {
            alertConsent?: boolean;
            idempotencyKey?: string | null;
            parameters: unknown;
        };
        CreateTaskInput: {
            checklist?: string[];
            physicalUnitId?: string | null;
            priority?: string | null;
            propertyId: string;
            reservationItemId?: string | null;
            taskType: string;
        };
        CreateUnitTypeInput: {
            accessibility?: string[];
            bathroomCount?: number;
            bedCount?: number;
            bedroomCount?: number;
            categoryCode: string;
            descriptionEn?: string;
            descriptionFr?: string;
            idempotencyKey?: string | null;
            inventoryModel: string;
            maxAdults: number;
            maxChildren?: number;
            nameEn: string;
            nameFr: string;
            sellableQuantity?: number;
            templateId?: string | null;
        };
        CreateWishlist: {
            idempotencyKey?: string | null;
            name: string;
            visibility?: string | null;
        };
        CreditDetail: {
            balance: string;
            enabled: boolean;
            hostId: string;
            low: boolean;
            lowThreshold: number;
            rateBps: number;
        };
        CreditSummary: {
            accounts: number;
            commissionTaken30d: string;
            disabled: number;
            low: number;
            refunded30d: string;
            toppedUp30d: string;
            totalBalance: string;
        };
        CurrentRate: {
            configured: boolean;
            effectiveFrom?: string | null;
            hostId: string;
            rateBps: number;
        };
        DashboardDay: {
            bookings: number;
            cancellations: number;
            date: string;
            revenue: string;
        };
        DashboardFigures: {
            bookings: number;
            cancellations: number;
            commissionCompleted: string;
            commissionTaken: string;
            commissionUpcoming: string;
            creditBalance: string;
            creditsToppedUp: string;
            disabledBusinesses: number;
            nightsAvailable: number;
            nightsSold: number;
            openCases: number;
            payoutsPaid: string;
            payoutsPending: string;
            pendingRequests: number;
            revenue: string;
        };
        DashboardSummary: {
            cities: components["schemas"]["CityFigures"][];
            from: string;
            series: components["schemas"]["DashboardDay"][];
            to: string;
            totals: components["schemas"]["DashboardFigures"];
        };
        DecideInput: {
            approvedAmount?: string | null;
            decision: string;
            expectedVersion: number;
            reasonCode: string;
        };
        DecideReviewInput: {
            reasonCode?: string | null;
            status: string;
        };
        DecisionInput: {
            action: string;
            expectedVersion: number;
        };
        DecisionRow: {
            approvedAmount: string;
            decidedAt: string;
            decision: string;
            payoutRef?: string | null;
            payoutState: string;
            payoutUpdatedAt?: string | null;
            reasonCode: string;
        };
        DestinationView: {
            city: string;
            propertyCount: number;
        };
        Detail: components["schemas"]["Core"] & {
            amendments: components["schemas"]["Page_Amendment"];
            cancellationDeadline?: string | null;
            cases: components["schemas"]["Page_Case"];
            guests: components["schemas"]["Page_Guest"];
            items: components["schemas"]["Page_Item"];
            paymentState: string;
            timeline: components["schemas"]["Page_Timeline"];
            unitType?: null | components["schemas"]["UnitType"];
        };
        DiscountView: {
            amount: string;
            promotionId: string;
        };
        DistanceStats: {
            maxKm: number;
            minKm: number;
        };
        DocumentReview: {
            reasonCode?: string | null;
            status: string;
        };
        DocumentView: {
            bytes: number;
            createdAt: string;
            documentNumber?: string | null;
            entityId: string;
            entityType: string;
            fileName: string;
            id: string;
            kind: string;
            locale: string;
            pageCount: number;
            sha256Prefix: string;
        };
        Envelope_AdminCaseDetail: {
            data: {
                evidence: components["schemas"]["EvidenceView"][];
                item: components["schemas"]["AdminCaseItem"];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_AdminCaseItem: {
            data: {
                assignedAt?: string | null;
                assignedToUserId?: string | null;
                caseType: string;
                claimAmount?: string | null;
                closedAt?: string | null;
                createdAt: string;
                deadlineAt: string;
                escalatedAt?: string | null;
                escalationReason?: string | null;
                id: string;
                isOverdue: boolean;
                operatorId: string;
                operatorName: string;
                parties: Record<string, never>;
                reason: string;
                reservationId?: string | null;
                reservationReference?: string | null;
                resolution?: Record<string, never> | null;
                state: string;
                version: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_AdminEvidenceContent: {
            data: {
                contentBase64: string;
                fileName: string;
                mimeType: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_AdminPromotionView: {
            data: components["schemas"]["PromotionView"] & {
                fundedBy: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_AdminPropertyDetail: {
            data: components["schemas"]["PropertyView"] & {
                hostName: string;
                hostStatus: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_AdminRevisionView: {
            data: {
                live: components["schemas"]["ListingContent"];
                revision: components["schemas"]["RevisionView"];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_AdminTaskItem: {
            data: {
                assigneeUserId?: string | null;
                checklist: string[];
                completedAt?: string | null;
                createdAt: string;
                id: string;
                operatorId: string;
                operatorName: string;
                priority: string;
                propertyId: string;
                propertyNameEn: string;
                propertyNameFr: string;
                reservationReference?: string | null;
                state: string;
                taskType: string;
                unitCode?: string | null;
                version: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_AmendmentProposal: {
            data: {
                checkIn: string;
                checkOut: string;
                delta: string;
                expiresAt: string;
                id: string;
                nights: components["schemas"]["NightLine"][];
                reservationId: string;
                state: string;
                total: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_AnalyticsSummary: {
            data: {
                approvedReviews: number;
                averageRating?: number | null;
                cancelled: number;
                checkedIn: number;
                checkedOut: number;
                confirmed: number;
                currency: string;
                grossRevenue: string;
                noShow: number;
                reservations: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_ApplicationView: {
            data: {
                applicantUserId: string;
                cityId: string;
                createdAt: string;
                decidedAt?: string | null;
                displayName: string;
                documents: components["schemas"]["HostDocumentView"][];
                hostId?: string | null;
                id: string;
                kind: string;
                reasonCode?: string | null;
                status: string;
                submittedAt?: string | null;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_AssignmentView: {
            data: {
                createdAt: string;
                endDate: string;
                id: string;
                physicalUnitId: string;
                reason: string;
                releasedAt?: string | null;
                reservationItemId: string;
                startDate: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_AvailabilityView: {
            data: {
                checkIn: string;
                checkOut: string;
                currency: string;
                nights: number;
                options: components["schemas"]["AvailabilityOptionView"][];
                propertyId: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_BalanceView: {
            data: {
                balance: string;
                hostId: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_BlockView: {
            data: {
                createdAt: string;
                endDate: string;
                id: string;
                origin: string;
                propertyId: string;
                reason: string;
                removedAt?: string | null;
                startDate: string;
                unitTypeId?: string | null;
                version: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_BulkRestrictionResult: {
            data: {
                affectedNights: number;
                applied: components["schemas"]["RestrictionView"][];
                removed: string[];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_CalendarView: {
            data: {
                days: components["schemas"]["CalendarDayView"][];
                unitTypeId: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_CancelPreview: {
            data: {
                canCancel: boolean;
                deadline?: string | null;
                policyPenalty?: string | null;
                policyReason?: string | null;
                policyRefund?: string | null;
                refund: string;
                reservationId: string;
                state: string;
                total: string;
                version: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_CancellationView: {
            data: {
                currency: string;
                deadline: string;
                evaluatedAt: string;
                penalty: string;
                reason: string;
                refund: string;
                reservationId: string;
                total: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_CancelledStayView: {
            data: {
                cancelledAt: string;
                chargeDueAtProperty: string;
                currency: string;
                reason: string;
                refund: string;
                reservationId: string;
                state: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_CaseView: {
            data: {
                caseType: string;
                closedAt?: string | null;
                createdAt: string;
                deadlineAt: string;
                id: string;
                operatorId: string;
                parties: Record<string, never>;
                reason: string;
                reservationId?: string | null;
                reservationReference?: string | null;
                resolution?: Record<string, never> | null;
                state: string;
                version: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_CatalogView: {
            data: {
                permissions: components["schemas"]["PermissionView"][];
                presets: components["schemas"]["PresetView"][];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_CategoryView: {
            data: {
                active: boolean;
                code: string;
                icon: string;
                nameEn: string;
                nameFr: string;
                revision: number;
                sortOrder: number;
                unitTypeCount: number;
                updatedAt: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_CityView: {
            data: {
                code: string;
                name: string;
                timezone: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_ClaimDecisionResult: {
            data: {
                approvedAmount: string;
                caseId: string;
                decision: string;
                payoutState: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_ClaimDetail: {
            data: components["schemas"]["ClaimCore"] & {
                decision?: null | components["schemas"]["DecisionRow"];
                evidence: components["schemas"]["EvidenceRow"][];
                statements: components["schemas"]["StatementRow"][];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_ClaimSummary: {
            data: {
                approved: string;
                claimed: string;
                paid: string;
                payoutPending: string;
                total: number;
                undecided: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_CreditDetail: {
            data: {
                balance: string;
                enabled: boolean;
                hostId: string;
                low: boolean;
                lowThreshold: number;
                rateBps: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_CreditSummary: {
            data: {
                accounts: number;
                commissionTaken30d: string;
                disabled: number;
                low: number;
                refunded30d: string;
                toppedUp30d: string;
                totalBalance: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_CurrentRate: {
            data: {
                configured: boolean;
                effectiveFrom?: string | null;
                hostId: string;
                rateBps: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_DashboardFigures: {
            data: {
                bookings: number;
                cancellations: number;
                commissionCompleted: string;
                commissionTaken: string;
                commissionUpcoming: string;
                creditBalance: string;
                creditsToppedUp: string;
                disabledBusinesses: number;
                nightsAvailable: number;
                nightsSold: number;
                openCases: number;
                payoutsPaid: string;
                payoutsPending: string;
                pendingRequests: number;
                revenue: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_DashboardSummary: {
            data: {
                cities: components["schemas"]["CityFigures"][];
                from: string;
                series: components["schemas"]["DashboardDay"][];
                to: string;
                totals: components["schemas"]["DashboardFigures"];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Detail: {
            data: components["schemas"]["Core"] & {
                amendments: components["schemas"]["Page_Amendment"];
                cancellationDeadline?: string | null;
                cases: components["schemas"]["Page_Case"];
                guests: components["schemas"]["Page_Guest"];
                items: components["schemas"]["Page_Item"];
                paymentState: string;
                timeline: components["schemas"]["Page_Timeline"];
                unitType?: null | components["schemas"]["UnitType"];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_EvidenceContent: {
            data: {
                contentBase64: string;
                evidence: components["schemas"]["EvidenceView"];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_EvidenceView: {
            data: {
                addedByUserId: string;
                caseId: string;
                createdAt: string;
                fileName: string;
                id: string;
                mimeType: string;
                sizeBytes: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_FeedTokenCreated: {
            data: {
                feedPath: string;
                tokenId: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_FeedTokenRevoked: {
            data: {
                revoked: boolean;
                tokenId: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_GuestItem: {
            data: {
                activeBookings: number;
                bookings: number;
                cancelledBookings: number;
                contactEmail?: string | null;
                contactPhone?: string | null;
                fullName?: string | null;
                lastBookingAt?: string | null;
                openCases: number;
                registeredAt?: string | null;
                status?: string | null;
                totalSpent: string;
                userId: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_HostContact: {
            data: {
                guestContactEmail: string;
                guestContactPhone: string;
                hostId: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_HostDocumentView: {
            data: {
                createdAt: string;
                fileName: string;
                id: string;
                kind: string;
                mimeType: string;
                reasonCode?: string | null;
                sizeBytes: number;
                status: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_HostOverride: {
            data: {
                effect: components["schemas"]["OverrideEffect"];
                hostId: string;
                id: string;
                permission: string;
                userId: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_HostReferenceView: {
            data: {
                approveReasons: string[];
                documentKinds: string[];
                documentRejectReasons: string[];
                kinds: string[];
                reinstateReasons: string[];
                rejectReasons: string[];
                suspendReasons: string[];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_HostStatusView: {
            data: {
                hostId: string;
                status: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_InventoryCalendarView: {
            data: {
                blocks: components["schemas"]["CalendarBlock"][];
                from: string;
                page: number;
                pageSize: number;
                propertyId: string;
                reservations: components["schemas"]["CalendarReservation"][];
                restrictions: components["schemas"]["RestrictionView"][];
                timezone: string;
                to: string;
                totalUnits: number;
                units: components["schemas"]["CalendarUnit"][];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_InventoryDefaultsResult: {
            data: {
                enabled: boolean;
                unitTypeId: string;
                version: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_InvitationView: {
            data: {
                createdAt: string;
                email: string;
                expiresAt: string;
                hostId: string;
                id: string;
                permissions: string[];
                status: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_MarkedCount: {
            data: {
                updated: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_MediaContentView: {
            data: {
                alt: string;
                mediaId: string;
                mimeType: string;
                url: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_MemberView: {
            data: {
                email?: string | null;
                firstName?: string | null;
                isOwner: boolean;
                lastName?: string | null;
                overrides: components["schemas"]["MemberOverride"][];
                permissions: string[];
                phone?: string | null;
                userId: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_MessageResponse: {
            data: {
                message: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_MessageView: {
            data: {
                body: string;
                createdAt: string;
                id: string;
                senderRole: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_MineSummary: {
            data: {
                balance: string;
                enabled: boolean;
                low: boolean;
                lowThreshold: number;
                rateBps: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_MyReservationsPage: {
            data: {
                counts: components["schemas"]["MyReservationCounts"];
                items: components["schemas"]["MyReservationView"][];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_OperationNotice: {
            data: {
                message: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_OperatorReservationView: {
            data: {
                adultCount: number;
                allowedActions: string[];
                approvalDeadlineAt?: string | null;
                assignedUnitCode?: string | null;
                assignmentId?: string | null;
                assignmentStart?: string | null;
                checkIn: string;
                checkOut: string;
                childCount: number;
                commissionAmount: string;
                commissionRateBps: number;
                createdAt: string;
                currency: string;
                guest?: null | components["schemas"]["GuestView"];
                id: string;
                itemId: string;
                netAmount: string;
                nights: components["schemas"]["NightLine"][];
                propertyId: string;
                propertyName: string;
                reference: string;
                specialRequest?: string | null;
                specialRequestState: string;
                state: string;
                total: string;
                unitType?: Record<string, never> | null;
                unitTypeCoverUrl?: string | null;
                unitTypeId: string;
                unitTypeName: string;
                version: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_OtpChannelResponse: {
            data: {
                channel: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_AccountRow: {
            data: {
                items: {
                    balance: string;
                    enabled: boolean;
                    hostId: string;
                    hostName: string;
                    lastTopUpAt?: string | null;
                    low: boolean;
                    rateBps: number;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_AdminApplicationView: {
            data: {
                items: {
                    applicantEmail?: string | null;
                    applicantName?: string | null;
                    applicantUserId: string;
                    cityId: string;
                    createdAt: string;
                    decidedAt?: string | null;
                    displayName: string;
                    hostId?: string | null;
                    id: string;
                    kind: string;
                    reasonCode?: string | null;
                    status: string;
                    submittedAt?: string | null;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_AdminCaseItem: {
            data: {
                items: {
                    assignedAt?: string | null;
                    assignedToUserId?: string | null;
                    caseType: string;
                    claimAmount?: string | null;
                    closedAt?: string | null;
                    createdAt: string;
                    deadlineAt: string;
                    escalatedAt?: string | null;
                    escalationReason?: string | null;
                    id: string;
                    isOverdue: boolean;
                    operatorId: string;
                    operatorName: string;
                    parties: Record<string, never>;
                    reason: string;
                    reservationId?: string | null;
                    reservationReference?: string | null;
                    resolution?: Record<string, never> | null;
                    state: string;
                    version: number;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_AdminHostView: {
            data: {
                items: {
                    cityId: string;
                    createdAt: string;
                    displayName: string;
                    guestContactEmail: string;
                    guestContactPhone: string;
                    id: string;
                    kind: string;
                    memberCount: number;
                    propertyCount: number;
                    status: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_AdminLedgerRow: {
            data: {
                items: {
                    amount: string;
                    balanceAfter: string;
                    createdAt: string;
                    createdByUserId?: string | null;
                    entryType: string;
                    id: string;
                    note?: string | null;
                    paymentRef?: string | null;
                    reasonCode?: string | null;
                    reservationId?: string | null;
                    reservationReference?: string | null;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_AdminPromotionView: {
            data: {
                items: (components["schemas"]["PromotionView"] & {
                    fundedBy: string;
                })[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_AdminPropertyItem: {
            data: {
                items: (components["schemas"]["PropertyFacts"] & {
                    addressLine: string;
                    city: string;
                    hostId: string;
                    hostName: string;
                    hostStatus: string;
                    id: string;
                    nameEn: string;
                    nameFr: string;
                    neighborhood: string;
                    openRevisions: number;
                    publishedRevision: number;
                    status: string;
                    updatedAt: string;
                    version: number;
                })[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_AdminReviewItem: {
            data: {
                items: {
                    city: string;
                    comment?: string | null;
                    createdAt: string;
                    guestName?: string | null;
                    id: string;
                    moderatedAt?: string | null;
                    operatorName: string;
                    operatorRepliedAt?: string | null;
                    operatorReply?: string | null;
                    propertyId: string;
                    propertyNameEn: string;
                    propertyNameFr: string;
                    rating: number;
                    reservationReference: string;
                    status: string;
                    unitTypeCoverUrl?: string | null;
                    unitTypeId?: string | null;
                    unitTypeNameEn?: string | null;
                    unitTypeNameFr?: string | null;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_AdminTaskItem: {
            data: {
                items: {
                    assigneeUserId?: string | null;
                    checklist: string[];
                    completedAt?: string | null;
                    createdAt: string;
                    id: string;
                    operatorId: string;
                    operatorName: string;
                    priority: string;
                    propertyId: string;
                    propertyNameEn: string;
                    propertyNameFr: string;
                    reservationReference?: string | null;
                    state: string;
                    taskType: string;
                    unitCode?: string | null;
                    version: number;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_AmendmentListItem: {
            data: {
                items: {
                    checkIn: string;
                    checkOut: string;
                    createdAt: string;
                    delta: string;
                    expiresAt: string;
                    id: string;
                    nights: components["schemas"]["NightLine"][];
                    respondedAt?: string | null;
                    state: string;
                    total: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_Amenity: {
            data: {
                items: {
                    active: boolean;
                    code: string;
                    groupCode: string;
                    icon: string;
                    name: string;
                    revision: number;
                    sortOrder: number;
                    updatedAt: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_AmenityGroup: {
            data: {
                items: {
                    active: boolean;
                    amenityCount: number;
                    code: string;
                    name: string;
                    revision: number;
                    sortOrder: number;
                    updatedAt: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_ApplicationView: {
            data: {
                items: {
                    applicantUserId: string;
                    cityId: string;
                    createdAt: string;
                    decidedAt?: string | null;
                    displayName: string;
                    documents: components["schemas"]["HostDocumentView"][];
                    hostId?: string | null;
                    id: string;
                    kind: string;
                    reasonCode?: string | null;
                    status: string;
                    submittedAt?: string | null;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_BlockView: {
            data: {
                items: {
                    createdAt: string;
                    endDate: string;
                    id: string;
                    origin: string;
                    propertyId: string;
                    reason: string;
                    removedAt?: string | null;
                    startDate: string;
                    unitTypeId?: string | null;
                    version: number;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_CaseAssignee: {
            data: {
                items: {
                    email?: string | null;
                    name?: string | null;
                    userId: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_CaseView: {
            data: {
                items: {
                    caseType: string;
                    closedAt?: string | null;
                    createdAt: string;
                    deadlineAt: string;
                    id: string;
                    operatorId: string;
                    parties: Record<string, never>;
                    reason: string;
                    reservationId?: string | null;
                    reservationReference?: string | null;
                    resolution?: Record<string, never> | null;
                    state: string;
                    version: number;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_CategoryView: {
            data: {
                items: {
                    active: boolean;
                    code: string;
                    icon: string;
                    nameEn: string;
                    nameFr: string;
                    revision: number;
                    sortOrder: number;
                    unitTypeCount: number;
                    updatedAt: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_ClaimRow: {
            data: {
                items: {
                    approvedAmount?: string | null;
                    caseId: string;
                    caseState: string;
                    checkOut?: string | null;
                    claimAmount: string;
                    deadlineAt: string;
                    decision?: string | null;
                    guestName?: string | null;
                    openedAt: string;
                    operatorId: string;
                    operatorName: string;
                    payoutState?: string | null;
                    propertyNameEn?: string | null;
                    propertyNameFr?: string | null;
                    reservationReference?: string | null;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_ConversationView: {
            data: {
                items: {
                    lastAt: string;
                    lastBody: string;
                    propertyNameEn: string;
                    propertyNameFr: string;
                    reference: string;
                    reservationId: string;
                    timezone: string;
                    unreadCount: number;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_DocumentView: {
            data: {
                items: {
                    bytes: number;
                    createdAt: string;
                    documentNumber?: string | null;
                    entityId: string;
                    entityType: string;
                    fileName: string;
                    id: string;
                    kind: string;
                    locale: string;
                    pageCount: number;
                    sha256Prefix: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_FeedTokenView: {
            data: {
                items: {
                    createdAt: string;
                    id: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_GuestBooking: {
            data: {
                items: {
                    checkIn?: string | null;
                    checkOut?: string | null;
                    city: string;
                    createdAt: string;
                    id: string;
                    nights?: number | null;
                    operatorName: string;
                    propertyNameEn: string;
                    propertyNameFr: string;
                    reference: string;
                    state: string;
                    totalAmount: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_GuestItem: {
            data: {
                items: {
                    activeBookings: number;
                    bookings: number;
                    cancelledBookings: number;
                    contactEmail?: string | null;
                    contactPhone?: string | null;
                    fullName?: string | null;
                    lastBookingAt?: string | null;
                    openCases: number;
                    registeredAt?: string | null;
                    status?: string | null;
                    totalSpent: string;
                    userId: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_HostFigures: {
            data: {
                items: (components["schemas"]["DashboardFigures"] & {
                    city: string;
                    hostId: string;
                    hostName: string;
                })[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_HostMembership: {
            data: {
                items: {
                    displayName: string;
                    hostId: string;
                    isOwner: boolean;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_InvitationView: {
            data: {
                items: {
                    createdAt: string;
                    email: string;
                    expiresAt: string;
                    hostId: string;
                    id: string;
                    permissions: string[];
                    status: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_MemberView: {
            data: {
                items: {
                    email?: string | null;
                    firstName?: string | null;
                    isOwner: boolean;
                    lastName?: string | null;
                    overrides: components["schemas"]["MemberOverride"][];
                    permissions: string[];
                    phone?: string | null;
                    userId: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_MessageView: {
            data: {
                items: {
                    body: string;
                    createdAt: string;
                    id: string;
                    senderRole: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_MineLedgerRow: {
            data: {
                items: {
                    amount: string;
                    balanceAfter: string;
                    createdAt: string;
                    entryType: string;
                    id: string;
                    note?: string | null;
                    paymentRef?: string | null;
                    reasonCode?: string | null;
                    reservationId?: string | null;
                    reservationReference?: string | null;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_MyInvitationView: {
            data: {
                items: {
                    expiresAt: string;
                    hostId: string;
                    hostName: string;
                    id: string;
                    permissions: string[];
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_NotificationView: {
            data: {
                items: {
                    audience?: string | null;
                    body: string;
                    category?: string | null;
                    createdAt: string;
                    deepLink?: string | null;
                    entityId?: string | null;
                    entityType?: string | null;
                    eventKey?: string | null;
                    id: string;
                    params?: unknown;
                    priority: string;
                    read: boolean;
                    readAt?: string | null;
                    seenAt?: string | null;
                    title: string;
                    type: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_OperatorReservationView: {
            data: {
                items: {
                    adultCount: number;
                    allowedActions: string[];
                    approvalDeadlineAt?: string | null;
                    assignedUnitCode?: string | null;
                    assignmentId?: string | null;
                    assignmentStart?: string | null;
                    checkIn: string;
                    checkOut: string;
                    childCount: number;
                    commissionAmount: string;
                    commissionRateBps: number;
                    createdAt: string;
                    currency: string;
                    guest?: null | components["schemas"]["GuestView"];
                    id: string;
                    itemId: string;
                    netAmount: string;
                    nights: components["schemas"]["NightLine"][];
                    propertyId: string;
                    propertyName: string;
                    reference: string;
                    specialRequest?: string | null;
                    specialRequestState: string;
                    state: string;
                    total: string;
                    unitType?: Record<string, never> | null;
                    unitTypeCoverUrl?: string | null;
                    unitTypeId: string;
                    unitTypeName: string;
                    version: number;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_PlanView: {
            data: {
                items: {
                    bookingMode: string;
                    cancellationPolicyVersionId: string;
                    childNightlyAmount: string;
                    id: string;
                    mealPlan: string;
                    occupancyBasis: string;
                    paymentPolicyVersionId: string;
                    status: string;
                    unitTypeId: string;
                    version: number;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_PromotionView: {
            data: {
                items: {
                    capAmount?: string | null;
                    discountPercent: string;
                    endDate?: string | null;
                    id: string;
                    minNights: number;
                    priority: number;
                    ratePlanId?: string | null;
                    redeemed: number;
                    redemptionLimit?: number | null;
                    startDate?: string | null;
                    status: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_PropertyView: {
            data: {
                items: {
                    addressLine: string;
                    amenities: unknown;
                    checkInFrom?: string | null;
                    checkInUntil?: string | null;
                    checkOutBy?: string | null;
                    city: string;
                    coverMediaId?: string | null;
                    coverMediaMime?: string | null;
                    coverMediaVersion?: number | null;
                    createdAt: string;
                    descriptionEn: string;
                    descriptionFr: string;
                    effectiveGuestContactEmail: string;
                    effectiveGuestContactPhone: string;
                    fromAmount?: string | null;
                    guestContactEmail: string;
                    guestContactPhone: string;
                    hostId: string;
                    houseRulesEn: string;
                    houseRulesFr: string;
                    id: string;
                    latitude?: number | null;
                    longitude?: number | null;
                    nameEn: string;
                    nameFr: string;
                    neighborhood: string;
                    openReservations: number;
                    pendingRequests: number;
                    petsPolicy?: string | null;
                    photoCount: number;
                    publishedRevision: number;
                    smokingPolicy?: string | null;
                    status: string;
                    timezone: string;
                    unitCount: number;
                    unitTypeCount: number;
                    updatedAt: string;
                    verificationStatus: string;
                    version: number;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_QueueItem: {
            data: {
                items: (components["schemas"]["PropertyFacts"] & {
                    changedFields: number;
                    city: string;
                    hostId: string;
                    hostName: string;
                    isFirstSubmission: boolean;
                    neighborhood: string;
                    propertyId: string;
                    propertyNameEn: string;
                    reviewStatus: string;
                    revisionId: string;
                    revisionNumber: number;
                    submittedAt?: string | null;
                })[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_RateRow: {
            data: {
                items: {
                    changedByUserId: string;
                    createdAt: string;
                    effectiveFrom: string;
                    id: string;
                    note?: string | null;
                    rateBps: number;
                    reasonCode: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_ReservationRow: {
            data: {
                items: {
                    bookingMode: string;
                    checkIn?: string | null;
                    checkOut?: string | null;
                    city: string;
                    commissionAmount: string;
                    createdAt: string;
                    guestName?: string | null;
                    id: string;
                    nights?: number | null;
                    openCases: number;
                    operatorId: string;
                    operatorName: string;
                    operatorNetAmount: string;
                    propertyId: string;
                    propertyNameEn: string;
                    propertyNameFr: string;
                    reference: string;
                    source: string;
                    state: string;
                    totalAmount: string;
                    unitTypeCoverUrl?: string | null;
                    unitTypeId?: string | null;
                    unitTypeNameEn?: string | null;
                    unitTypeNameFr?: string | null;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_RestrictionView: {
            data: {
                items: {
                    basis: string;
                    createdAt: string;
                    endDate?: string | null;
                    id: string;
                    kind: string;
                    propertyId: string;
                    removedAt?: string | null;
                    startDate?: string | null;
                    unitTypeId?: string | null;
                    value?: number | null;
                    version: number;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_ReviewView: {
            data: {
                items: {
                    comment?: string | null;
                    createdAt: string;
                    id: string;
                    operatorRepliedAt?: string | null;
                    operatorReply?: string | null;
                    propertyId: string;
                    rating: number;
                    status: string;
                    unitTypeCoverUrl?: string | null;
                    unitTypeId?: string | null;
                    unitTypeNameEn?: string | null;
                    unitTypeNameFr?: string | null;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_RevisionSummary: {
            data: {
                items: {
                    authorUserId: string;
                    createdAt: string;
                    id: string;
                    propertyId: string;
                    reviewGuidance?: string | null;
                    reviewReasons: unknown;
                    reviewStatus: string;
                    reviewedAt?: string | null;
                    revisionNumber: number;
                    submittedAt?: string | null;
                    updatedAt: string;
                    version: number;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_RuleView: {
            data: {
                items: {
                    adjustmentType: string;
                    amount: string;
                    endDate?: string | null;
                    id: string;
                    priority: number;
                    ratePlanId: string;
                    startDate?: string | null;
                    weekdays: number[];
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_SavedSearchView: {
            data: {
                items: {
                    alertConsent: boolean;
                    createdAt: string;
                    id: string;
                    parameters: unknown;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_TaskView: {
            data: {
                items: {
                    assigneeUserId?: string | null;
                    checklist: string[];
                    completedAt?: string | null;
                    createdAt: string;
                    id: string;
                    physicalUnitId?: string | null;
                    priority: string;
                    propertyId: string;
                    reservationItemId?: string | null;
                    state: string;
                    taskType: string;
                    updatedAt: string;
                    version: number;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_TemplateView: {
            data: {
                items: {
                    accessibility: unknown;
                    baseNightlyAmount: string;
                    bathroomCount: number;
                    bedCount: number;
                    bedroomCount: number;
                    categoryCode: string;
                    cityId: string;
                    coverUrl?: string | null;
                    descriptionEn: string;
                    descriptionFr: string;
                    hostId: string;
                    id: string;
                    imageCount: number;
                    inventoryModel: string;
                    maxAdults: number;
                    maxChildren: number;
                    nameEn: string;
                    nameFr: string;
                    status: string;
                    unitTypeCount: number;
                    updatedAt: string;
                    version: number;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_UnitTypeView: {
            data: {
                items: {
                    accessibility: unknown;
                    baseNightlyAmount?: string | null;
                    bathroomCount: number;
                    bedCount: number;
                    bedroomCount: number;
                    categoryCode: string;
                    coverUrl?: string | null;
                    descriptionEn: string;
                    descriptionFr: string;
                    id: string;
                    images?: components["schemas"]["ImageView"][];
                    inventoryModel: string;
                    maxAdults: number;
                    maxChildren: number;
                    nameEn: string;
                    nameFr: string;
                    sellableQuantity: number;
                    status: string;
                    templateId?: string | null;
                    version: number;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_WishlistItem: {
            data: {
                items: {
                    addedAt: string;
                    name: string;
                    propertyId: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Page_WishlistView: {
            data: {
                items: {
                    createdAt: string;
                    id: string;
                    itemCount: number;
                    name: string;
                    visibility: string;
                }[];
                page: number;
                pageSize: number;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_PayoutResult: {
            data: {
                caseId: string;
                payoutState: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_PlanView: {
            data: {
                bookingMode: string;
                cancellationPolicyVersionId: string;
                childNightlyAmount: string;
                id: string;
                mealPlan: string;
                occupancyBasis: string;
                paymentPolicyVersionId: string;
                status: string;
                unitTypeId: string;
                version: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_PreferenceList: {
            data: {
                items: components["schemas"]["PreferenceView"][];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_PreferenceView: {
            data: {
                email: boolean;
                eventKey: string;
                inApp: boolean;
                push: boolean;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_PricePreviewView: {
            data: {
                currency: string;
                discount?: null | components["schemas"]["DiscountView"];
                nights: components["schemas"]["PreviewNight"][];
                propertyId: string;
                ratePlanId: string;
                subtotal: string;
                total: string;
                unitTypeId: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_PromotionView: {
            data: {
                capAmount?: string | null;
                discountPercent: string;
                endDate?: string | null;
                id: string;
                minNights: number;
                priority: number;
                ratePlanId?: string | null;
                redeemed: number;
                redemptionLimit?: number | null;
                startDate?: string | null;
                status: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_ProofResponse: {
            data: {
                proof: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_PropertyDetailView: {
            data: {
                amenities: string[];
                city: string;
                description: string;
                hostName?: string | null;
                id: string;
                media: components["schemas"]["MediaView"][];
                name: string;
                ratePlans: components["schemas"]["RatePlanView"][];
                timezone: string;
                unitTypes: components["schemas"]["UnitTypeView"][];
                verificationStatus: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_PropertyMediaContent: {
            data: {
                media: components["schemas"]["PropertyMediaView"];
                url: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_PropertyMediaRemoved: {
            data: {
                mediaId: string;
                status: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_PropertyMediaView: {
            data: {
                altEn: string;
                altFr: string;
                createdAt: string;
                fileName: string;
                id: string;
                isCover: boolean;
                mimeType: string;
                ownerId: string;
                rejectionReason?: string | null;
                sizeBytes: number;
                sortOrder: number;
                status: string;
                updatedAt: string;
                version: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_PropertyView: {
            data: {
                addressLine: string;
                amenities: unknown;
                checkInFrom?: string | null;
                checkInUntil?: string | null;
                checkOutBy?: string | null;
                city: string;
                coverMediaId?: string | null;
                coverMediaMime?: string | null;
                coverMediaVersion?: number | null;
                createdAt: string;
                descriptionEn: string;
                descriptionFr: string;
                effectiveGuestContactEmail: string;
                effectiveGuestContactPhone: string;
                fromAmount?: string | null;
                guestContactEmail: string;
                guestContactPhone: string;
                hostId: string;
                houseRulesEn: string;
                houseRulesFr: string;
                id: string;
                latitude?: number | null;
                longitude?: number | null;
                nameEn: string;
                nameFr: string;
                neighborhood: string;
                openReservations: number;
                pendingRequests: number;
                petsPolicy?: string | null;
                photoCount: number;
                publishedRevision: number;
                smokingPolicy?: string | null;
                status: string;
                timezone: string;
                unitCount: number;
                unitTypeCount: number;
                updatedAt: string;
                verificationStatus: string;
                version: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_QuoteView: {
            data: {
                adultCount: number;
                cancellationDeadline?: string | null;
                checkIn: string;
                checkOut: string;
                childCount: number;
                currency: string;
                discount?: null | components["schemas"]["DiscountView"];
                expired: boolean;
                expiresAt: string;
                id: string;
                nights: components["schemas"]["NightLine"][];
                paymentSchedule: components["schemas"]["PaymentSchedule"];
                policies: Record<string, never>;
                propertyId: string;
                ratePlanId: string;
                subtotal?: string | null;
                total: string;
                unitTypeCoverUrl?: string | null;
                unitTypeId: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_RateSet: {
            data: {
                hostId: string;
                id: string;
                rateBps: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_ReadMessagesResult: {
            data: {
                marked: boolean;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_ReadinessView: {
            data: {
                blockers: string[];
                ready: boolean;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_RemovedRule: {
            data: {
                id: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_ReservationSummary: {
            data: {
                awaitingPayment: number;
                cancelled: number;
                checkedIn: number;
                confirmed: number;
                noShow: number;
                requested: number;
                revenue: string;
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_ReservationView: {
            data: {
                adultCount: number;
                approvalDeadlineAt?: string | null;
                checkIn: string;
                checkOut: string;
                childCount: number;
                currency: string;
                discount?: null | components["schemas"]["DiscountView"];
                guest: components["schemas"]["GuestView"];
                holdExpiresAt?: string | null;
                id: string;
                nights: components["schemas"]["NightLine"][];
                paymentRequired: boolean;
                propertyId: string;
                ratePlanId: string;
                reference: string;
                specialRequest?: string | null;
                specialRequestState: string;
                state: string;
                total: string;
                unitTypeCoverUrl?: string | null;
                unitTypeId: string;
                unitTypeImages: components["schemas"]["TripImageView"][];
                version: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_ReviewView: {
            data: {
                comment?: string | null;
                createdAt: string;
                id: string;
                operatorRepliedAt?: string | null;
                operatorReply?: string | null;
                propertyId: string;
                rating: number;
                status: string;
                unitTypeCoverUrl?: string | null;
                unitTypeId?: string | null;
                unitTypeNameEn?: string | null;
                unitTypeNameFr?: string | null;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_RevisionView: {
            data: components["schemas"]["RevisionSummary"] & {
                content: components["schemas"]["ListingContent"];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_RuleView: {
            data: {
                adjustmentType: string;
                amount: string;
                endDate?: string | null;
                id: string;
                priority: number;
                ratePlanId: string;
                startDate?: string | null;
                weekdays: number[];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_SavedCatalogItem: {
            data: {
                code: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_SavedSearchRemoved: {
            data: {
                removedAt: string;
                savedSearchId: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_SavedSearchView: {
            data: {
                alertConsent: boolean;
                createdAt: string;
                id: string;
                parameters: unknown;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_SearchResponse: {
            data: {
                dataAsOf: string;
                engineVersion: number;
                facets: components["schemas"]["Facets"];
                fingerprint: string;
                items: components["schemas"]["SearchHit"][];
                meta: components["schemas"]["Meta"];
                page: number;
                pageSize: number;
                query: components["schemas"]["SearchQuery"];
                stats: components["schemas"]["Stats"];
                total: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_SessionView: {
            data: {
                hosts: components["schemas"]["SessionHost"][];
                permissions: components["schemas"]["PermissionGrantView"][];
                user: components["schemas"]["SessionUser"];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_SettingsView: {
            data: {
                city: string;
                items: components["schemas"]["SettingView"][];
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_StatementRow: {
            data: {
                body: string;
                createdAt: string;
                id: string;
                party: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_TaskView: {
            data: {
                assigneeUserId?: string | null;
                checklist: string[];
                completedAt?: string | null;
                createdAt: string;
                id: string;
                physicalUnitId?: string | null;
                priority: string;
                propertyId: string;
                reservationItemId?: string | null;
                state: string;
                taskType: string;
                updatedAt: string;
                version: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_TemplateView: {
            data: {
                accessibility: unknown;
                baseNightlyAmount: string;
                bathroomCount: number;
                bedCount: number;
                bedroomCount: number;
                categoryCode: string;
                cityId: string;
                coverUrl?: string | null;
                descriptionEn: string;
                descriptionFr: string;
                hostId: string;
                id: string;
                imageCount: number;
                inventoryModel: string;
                maxAdults: number;
                maxChildren: number;
                nameEn: string;
                nameFr: string;
                status: string;
                unitTypeCount: number;
                updatedAt: string;
                version: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_TokenResponse: {
            data: {
                accessToken: string;
                refreshToken: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_UnitTypeView: {
            data: {
                accessibility: unknown;
                baseNightlyAmount?: string | null;
                bathroomCount: number;
                bedCount: number;
                bedroomCount: number;
                categoryCode: string;
                coverUrl?: string | null;
                descriptionEn: string;
                descriptionFr: string;
                id: string;
                images?: components["schemas"]["ImageView"][];
                inventoryModel: string;
                maxAdults: number;
                maxChildren: number;
                nameEn: string;
                nameFr: string;
                sellableQuantity: number;
                status: string;
                templateId?: string | null;
                version: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_UnreadCount: {
            data: {
                unread: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_UpdatedSetting: {
            data: {
                key: string;
                value: number;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Value: {
            data: unknown;
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Vec_AssignmentView: {
            data: {
                createdAt: string;
                endDate: string;
                id: string;
                physicalUnitId: string;
                reason: string;
                releasedAt?: string | null;
                reservationItemId: string;
                startDate: string;
            }[];
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Vec_CategoryView: {
            data: {
                active: boolean;
                code: string;
                icon: string;
                nameEn: string;
                nameFr: string;
                revision: number;
                sortOrder: number;
                unitTypeCount: number;
                updatedAt: string;
            }[];
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Vec_CityView: {
            data: {
                code: string;
                name: string;
                timezone: string;
            }[];
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Vec_DestinationView: {
            data: {
                city: string;
                propertyCount: number;
            }[];
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Vec_EvidenceView: {
            data: {
                addedByUserId: string;
                caseId: string;
                createdAt: string;
                fileName: string;
                id: string;
                mimeType: string;
                sizeBytes: number;
            }[];
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Vec_PolicyView: {
            data: {
                content: Record<string, never>;
                id: string;
                kind: string;
                versionNumber: number;
            }[];
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Vec_PropertyMediaView: {
            data: {
                altEn: string;
                altFr: string;
                createdAt: string;
                fileName: string;
                id: string;
                isCover: boolean;
                mimeType: string;
                ownerId: string;
                rejectionReason?: string | null;
                sizeBytes: number;
                sortOrder: number;
                status: string;
                updatedAt: string;
                version: number;
            }[];
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Vec_String: {
            data: string[];
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_Vec_UnitOptionView: {
            data: {
                id: string;
                localCode: string;
            }[];
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        Envelope_WishlistView: {
            data: {
                createdAt: string;
                id: string;
                itemCount: number;
                name: string;
                visibility: string;
            };
            metadata: components["schemas"]["Metadata"];
            success: boolean;
        };
        ErrorBody: {
            error: components["schemas"]["ErrorDetail"];
            success: boolean;
        };
        ErrorCode: string;
        ErrorDetail: {
            code: components["schemas"]["ErrorCode"];
            params?: unknown;
        };
        EscalateCaseInput: {
            expectedVersion: number;
            reason: string;
        };
        EvidenceContent: {
            contentBase64: string;
            evidence: components["schemas"]["EvidenceView"];
        };
        EvidenceRow: {
            createdAt: string;
            fileName: string;
            id: string;
            mimeType: string;
            sizeBytes: number;
        };
        EvidenceView: {
            addedByUserId: string;
            caseId: string;
            createdAt: string;
            fileName: string;
            id: string;
            mimeType: string;
            sizeBytes: number;
        };
        FacetValue: {
            count: number;
            value: string;
        };
        Facets: {
            accessibility: components["schemas"]["FacetValue"][];
            amenities: components["schemas"]["FacetValue"][];
            bookingMode: components["schemas"]["FacetValue"][];
            cancellationClass: components["schemas"]["FacetValue"][];
            category: components["schemas"]["FacetValue"][];
            mealPlan: components["schemas"]["FacetValue"][];
            paymentCollection: components["schemas"]["FacetValue"][];
            propertyType: components["schemas"]["FacetValue"][];
            verificationStatus: components["schemas"]["FacetValue"][];
        };
        FeedTokenCreated: {
            feedPath: string;
            tokenId: string;
        };
        FeedTokenRevoked: {
            revoked: boolean;
            tokenId: string;
        };
        FeedTokenView: {
            createdAt: string;
            id: string;
        };
        FundingInput: {
            fundedBy: string;
        };
        GroupWrite: {
            active?: boolean | null;
            nameEn: string;
            nameFr: string;
            sortOrder?: number | null;
        };
        GuestBooking: {
            checkIn?: string | null;
            checkOut?: string | null;
            city: string;
            createdAt: string;
            id: string;
            nights?: number | null;
            operatorName: string;
            propertyNameEn: string;
            propertyNameFr: string;
            reference: string;
            state: string;
            totalAmount: string;
        };
        GuestInput: {
            contactEmail?: string | null;
            contactPhone?: string | null;
            fullName: string;
        };
        GuestItem: {
            activeBookings: number;
            bookings: number;
            cancelledBookings: number;
            contactEmail?: string | null;
            contactPhone?: string | null;
            fullName?: string | null;
            lastBookingAt?: string | null;
            openCases: number;
            registeredAt?: string | null;
            status?: string | null;
            totalSpent: string;
            userId: string;
        };
        GuestView: {
            contactEmail?: string | null;
            contactPhone?: string | null;
            fullName: string;
        };
        Health: {
            database: boolean;
        };
        HealthMetadata: {
            requestId: string;
        };
        HealthResponse: {
            data: components["schemas"]["Health"];
            metadata: components["schemas"]["HealthMetadata"];
            success: boolean;
        };
        HoldRequest: {
            acceptedPolicyVersionIds?: string[];
            guest: components["schemas"]["GuestInput"];
            quoteId: string;
            specialRequest?: string | null;
        };
        HostContact: {
            guestContactEmail: string;
            guestContactPhone: string;
            hostId: string;
        };
        HostDecisionInput: {
            decision: string;
            reasonCode: string;
        };
        HostDocumentView: {
            createdAt: string;
            fileName: string;
            id: string;
            kind: string;
            mimeType: string;
            reasonCode?: string | null;
            sizeBytes: number;
            status: string;
        };
        HostFigures: components["schemas"]["DashboardFigures"] & {
            city: string;
            hostId: string;
            hostName: string;
        };
        HostMembership: {
            displayName: string;
            hostId: string;
            isOwner: boolean;
        };
        HostOverride: {
            effect: components["schemas"]["OverrideEffect"];
            hostId: string;
            id: string;
            permission: string;
            userId: string;
        };
        HostReferenceView: {
            approveReasons: string[];
            documentKinds: string[];
            documentRejectReasons: string[];
            kinds: string[];
            reinstateReasons: string[];
            rejectReasons: string[];
            suspendReasons: string[];
        };
        HostStatusInput: {
            action: string;
            reasonCode: string;
        };
        HostStatusView: {
            hostId: string;
            status: string;
        };
        ImageView: {
            alt: string;
            id: string;
            isCover: boolean;
            url: string;
        };
        InventoryCalendarView: {
            blocks: components["schemas"]["CalendarBlock"][];
            from: string;
            page: number;
            pageSize: number;
            propertyId: string;
            reservations: components["schemas"]["CalendarReservation"][];
            restrictions: components["schemas"]["RestrictionView"][];
            timezone: string;
            to: string;
            totalUnits: number;
            units: components["schemas"]["CalendarUnit"][];
        };
        InventoryDefaultsInput: {
            enabled: boolean;
            expectedVersion: number;
            propertyId: string;
            unitTypeId: string;
        };
        InventoryDefaultsResult: {
            enabled: boolean;
            unitTypeId: string;
            version: number;
        };
        InvitationView: {
            createdAt: string;
            email: string;
            expiresAt: string;
            hostId: string;
            id: string;
            permissions: string[];
            status: string;
        };
        IssueOtpInput: {
            channel: string;
            contact: string;
            language?: string | null;
        };
        LifecycleInput: {
            action: string;
            expectedVersion: number;
        };
        ListingContent: {
            addressLine: string;
            amenities: string[];
            checkInFrom?: string | null;
            checkInUntil?: string | null;
            checkOutBy?: string | null;
            descriptionEn: string;
            descriptionFr: string;
            guestContactEmail: string;
            guestContactPhone: string;
            houseRulesEn: string;
            houseRulesFr: string;
            latitude?: number | null;
            longitude?: number | null;
            nameEn: string;
            nameFr: string;
            neighborhood: string;
            petsPolicy?: string | null;
            smokingPolicy?: string | null;
        };
        ListingDecisionInput: {
            decision: string;
            expectedVersion: number;
            guidance?: string | null;
            reasonCodes?: string[];
        };
        LoginRequest: {
            identifier: string;
            password: string;
        };
        MarkedCount: {
            updated: number;
        };
        MediaContentView: {
            alt: string;
            mediaId: string;
            mimeType: string;
            url: string;
        };
        MediaView: {
            alt: string;
            contentPath: string;
            id: string;
            isCover: boolean;
            url: string;
        };
        MemberOverride: {
            effect: string;
            id: string;
            permission: string;
        };
        MemberView: {
            email?: string | null;
            firstName?: string | null;
            isOwner: boolean;
            lastName?: string | null;
            overrides: components["schemas"]["MemberOverride"][];
            permissions: string[];
            phone?: string | null;
            userId: string;
        };
        MessageResponse: {
            message: string;
        };
        MessageView: {
            body: string;
            createdAt: string;
            id: string;
            senderRole: string;
        };
        Meta: {
            analyticsRecorded: boolean;
            candidatePlans: number;
            pricedPlans: number;
            timingsMs: components["schemas"]["Timings"];
        };
        Metadata: {
            requestId: string;
        };
        MineCover: {
            altEn: string;
            altFr: string;
            url: string;
        };
        MineLedgerRow: {
            amount: string;
            balanceAfter: string;
            createdAt: string;
            entryType: string;
            id: string;
            note?: string | null;
            paymentRef?: string | null;
            reasonCode?: string | null;
            reservationId?: string | null;
            reservationReference?: string | null;
        };
        MineSummary: {
            balance: string;
            enabled: boolean;
            low: boolean;
            lowThreshold: number;
            rateBps: number;
        };
        ModerationInput: {
            expectedVersion: number;
            reasonCode?: string | null;
        };
        MoveUnitInput: {
            fromDate: string;
            physicalUnitId: string;
        };
        MyInvitationView: {
            expiresAt: string;
            hostId: string;
            hostName: string;
            id: string;
            permissions: string[];
        };
        MyReservationCounts: {
            active: number;
            all: number;
            cancelled: number;
            past: number;
            pending: number;
            upcoming: number;
        };
        MyReservationView: {
            adultCount: number;
            approvalDeadlineAt?: string | null;
            checkIn: string;
            checkOut: string;
            childCount: number;
            cover?: null | components["schemas"]["MineCover"];
            currency: string;
            discount?: null | components["schemas"]["DiscountView"];
            id: string;
            nights: components["schemas"]["NightLine"][];
            pendingAmendment?: null | components["schemas"]["PendingAmendmentView"];
            propertyId: string;
            propertyNameEn: string;
            propertyNameFr: string;
            reference: string;
            state: string;
            total: string;
            unitNameEn: string;
            unitNameFr: string;
            unitTypeCoverUrl?: string | null;
            unitTypeId: string;
            version: number;
        };
        MyReservationsPage: {
            counts: components["schemas"]["MyReservationCounts"];
            items: components["schemas"]["MyReservationView"][];
            page: number;
            pageSize: number;
            total: number;
        };
        NightAvailability: {
            date: string;
            remaining: number;
        };
        NightExplanation: {
            baseAmount: string;
            date: string;
            rules: components["schemas"]["RuleUse"][];
        };
        NightLine: {
            amount: string;
            date: string;
        };
        NotificationView: {
            audience?: string | null;
            body: string;
            category?: string | null;
            createdAt: string;
            deepLink?: string | null;
            entityId?: string | null;
            entityType?: string | null;
            eventKey?: string | null;
            id: string;
            params?: unknown;
            priority: string;
            read: boolean;
            readAt?: string | null;
            seenAt?: string | null;
            title: string;
            type: string;
        };
        OfferView: {
            accessibility: string[];
            bathroomCount: number;
            bedCount: number;
            bedroomCount: number;
            bookingMode: string;
            cancellation: components["schemas"]["CancellationView"];
            categoryCode?: string | null;
            mealPlan: string;
            nightly?: string | null;
            occupancyBasis: string;
            paymentCollection: string;
            propertyType: string;
            ratePlanId: string;
            remainingRooms?: number | null;
            total?: string | null;
            unitName: string;
            unitTypeId: string;
        };
        OpenCaseInput: {
            caseType: string;
            claimAmount?: string | null;
            hostId?: string | null;
            reason: string;
            reservationId?: string | null;
        };
        OperationNotice: {
            message: string;
        };
        OperatorCancelInput: {
            expectedVersion: number;
            reason: string;
        };
        OperatorReservationView: {
            adultCount: number;
            allowedActions: string[];
            approvalDeadlineAt?: string | null;
            assignedUnitCode?: string | null;
            assignmentId?: string | null;
            assignmentStart?: string | null;
            checkIn: string;
            checkOut: string;
            childCount: number;
            commissionAmount: string;
            commissionRateBps: number;
            createdAt: string;
            currency: string;
            guest?: null | components["schemas"]["GuestView"];
            id: string;
            itemId: string;
            netAmount: string;
            nights: components["schemas"]["NightLine"][];
            propertyId: string;
            propertyName: string;
            reference: string;
            specialRequest?: string | null;
            specialRequestState: string;
            state: string;
            total: string;
            unitType?: Record<string, never> | null;
            unitTypeCoverUrl?: string | null;
            unitTypeId: string;
            unitTypeName: string;
            version: number;
        };
        Origin: {
            latitude: number;
            longitude: number;
        };
        OtpChannelResponse: {
            channel: string;
        };
        OverrideEffect: "ALLOW" | "DENY";
        Page_Amendment: {
            items: {
                createdAt: string;
                deltaAmount: string;
                expiresAt?: string | null;
                id: string;
                respondedAt?: string | null;
                state: string;
            }[];
            page: number;
            pageSize: number;
            total: number;
        };
        Page_Case: {
            items: {
                caseType: string;
                deadlineAt: string;
                id: string;
                state: string;
            }[];
            page: number;
            pageSize: number;
            total: number;
        };
        Page_Guest: {
            items: {
                contactEmail?: string | null;
                contactPhone?: string | null;
                fullName: string;
                itemId: string;
                role: string;
            }[];
            page: number;
            pageSize: number;
            total: number;
        };
        Page_Image: {
            items: {
                altEn: string;
                altFr: string;
                id: string;
                isCover: boolean;
                url: string;
            }[];
            page: number;
            pageSize: number;
            total: number;
        };
        Page_Item: {
            items: {
                adultCount: number;
                checkIn: string;
                checkOut: string;
                childCount: number;
                id: string;
                quantity: number;
                subtotalAmount: string;
                unitCode?: string | null;
                unitTypeCoverUrl?: string | null;
                unitTypeId: string;
                unitTypeNameEn: string;
                unitTypeNameFr: string;
            }[];
            page: number;
            pageSize: number;
            total: number;
        };
        Page_Timeline: {
            items: {
                action: string;
                createdAt: string;
                id: string;
                reason?: string | null;
            }[];
            page: number;
            pageSize: number;
            total: number;
        };
        PaymentSchedule: {
            dueAtProperty: string;
            dueLater: string;
            dueNow: string;
        };
        PayoutInput: {
            payoutRef?: string | null;
            payoutState: string;
        };
        PayoutResult: {
            caseId: string;
            payoutState: string;
        };
        PendingAmendmentView: {
            checkIn: string;
            checkOut: string;
            delta: string;
            discount?: null | components["schemas"]["DiscountView"];
            expiresAt: string;
            id: string;
            nights: components["schemas"]["NightLine"][];
            total: string;
        };
        PermissionGrantView: {
            permission: string;
            scopeId?: string | null;
            scopeType: string;
        };
        PermissionView: {
            delegable: boolean;
            domain: string;
            name: string;
        };
        PlanView: {
            bookingMode: string;
            cancellationPolicyVersionId: string;
            childNightlyAmount: string;
            id: string;
            mealPlan: string;
            occupancyBasis: string;
            paymentPolicyVersionId: string;
            status: string;
            unitTypeId: string;
            version: number;
        };
        PlatformCancelInput: {
            expectedVersion: number;
            reasonCode: string;
        };
        PolicyView: {
            content: Record<string, never>;
            id: string;
            kind: string;
            versionNumber: number;
        };
        PreferenceList: {
            items: components["schemas"]["PreferenceView"][];
        };
        PreferenceView: {
            email: boolean;
            eventKey: string;
            inApp: boolean;
            push: boolean;
        };
        PresetView: {
            audience: string;
            code: string;
            permissions: string[];
        };
        PreviewNight: {
            amount: string;
            explanation: components["schemas"]["NightExplanation"];
        };
        PricePreviewView: {
            currency: string;
            discount?: null | components["schemas"]["DiscountView"];
            nights: components["schemas"]["PreviewNight"][];
            propertyId: string;
            ratePlanId: string;
            subtotal: string;
            total: string;
            unitTypeId: string;
        };
        PriceStats: {
            average: string;
            currency: string;
            max: string;
            median: string;
            min: string;
        };
        ProfileInput: {
            firstName?: string | null;
            lastName?: string | null;
            preferredLanguage?: string | null;
        };
        PromotionView: {
            capAmount?: string | null;
            discountPercent: string;
            endDate?: string | null;
            id: string;
            minNights: number;
            priority: number;
            ratePlanId?: string | null;
            redeemed: number;
            redemptionLimit?: number | null;
            startDate?: string | null;
            status: string;
        };
        ProofResponse: {
            proof: string;
        };
        PropertyDetailView: {
            amenities: string[];
            city: string;
            description: string;
            hostName?: string | null;
            id: string;
            media: components["schemas"]["MediaView"][];
            name: string;
            ratePlans: components["schemas"]["RatePlanView"][];
            timezone: string;
            unitTypes: components["schemas"]["UnitTypeView"][];
            verificationStatus: string;
        };
        PropertyFacts: {
            coverMediaId?: string | null;
            coverMediaMime?: string | null;
            coverMediaVersion?: number | null;
            fromAmount?: string | null;
            openReservations: number;
            pendingRequests: number;
            photoCount: number;
            unitCount: number;
            unitTypeCount: number;
        };
        PropertyMediaContent: {
            media: components["schemas"]["PropertyMediaView"];
            url: string;
        };
        PropertyMediaRemoved: {
            mediaId: string;
            status: string;
        };
        PropertyMediaView: {
            altEn: string;
            altFr: string;
            createdAt: string;
            fileName: string;
            id: string;
            isCover: boolean;
            mimeType: string;
            ownerId: string;
            rejectionReason?: string | null;
            sizeBytes: number;
            sortOrder: number;
            status: string;
            updatedAt: string;
            version: number;
        };
        PropertyView: {
            addressLine: string;
            amenities: unknown;
            checkInFrom?: string | null;
            checkInUntil?: string | null;
            checkOutBy?: string | null;
            city: string;
            coverMediaId?: string | null;
            coverMediaMime?: string | null;
            coverMediaVersion?: number | null;
            createdAt: string;
            descriptionEn: string;
            descriptionFr: string;
            effectiveGuestContactEmail: string;
            effectiveGuestContactPhone: string;
            fromAmount?: string | null;
            guestContactEmail: string;
            guestContactPhone: string;
            hostId: string;
            houseRulesEn: string;
            houseRulesFr: string;
            id: string;
            latitude?: number | null;
            longitude?: number | null;
            nameEn: string;
            nameFr: string;
            neighborhood: string;
            openReservations: number;
            pendingRequests: number;
            petsPolicy?: string | null;
            photoCount: number;
            publishedRevision: number;
            smokingPolicy?: string | null;
            status: string;
            timezone: string;
            unitCount: number;
            unitTypeCount: number;
            updatedAt: string;
            verificationStatus: string;
            version: number;
        };
        ProposeInput: {
            checkIn: string;
            checkOut: string;
        };
        PushTokenInput: {
            appVersion?: string | null;
            deviceId: string;
            platform: string;
            token: string;
        };
        PushTokenRemoval: {
            token: string;
        };
        QueueItem: components["schemas"]["PropertyFacts"] & {
            changedFields: number;
            city: string;
            hostId: string;
            hostName: string;
            isFirstSubmission: boolean;
            neighborhood: string;
            propertyId: string;
            propertyNameEn: string;
            reviewStatus: string;
            revisionId: string;
            revisionNumber: number;
            submittedAt?: string | null;
        };
        QuoteRequest: {
            adultCount: number;
            checkIn: string;
            checkOut: string;
            childCount?: number;
            propertyId: string;
            ratePlanId: string;
            unitTypeId: string;
        };
        QuoteView: {
            adultCount: number;
            cancellationDeadline?: string | null;
            checkIn: string;
            checkOut: string;
            childCount: number;
            currency: string;
            discount?: null | components["schemas"]["DiscountView"];
            expired: boolean;
            expiresAt: string;
            id: string;
            nights: components["schemas"]["NightLine"][];
            paymentSchedule: components["schemas"]["PaymentSchedule"];
            policies: Record<string, never>;
            propertyId: string;
            ratePlanId: string;
            subtotal?: string | null;
            total: string;
            unitTypeCoverUrl?: string | null;
            unitTypeId: string;
        };
        RateInput: {
            note?: string | null;
            rate: string;
            reasonCode: string;
        };
        RatePlanView: {
            bookingMode: string;
            cancellation?: unknown;
            id: string;
            mealPlan: string;
            nightly?: string | null;
            occupancyBasis: string;
            payment?: unknown;
            unitTypeId: string;
        };
        RateRow: {
            changedByUserId: string;
            createdAt: string;
            effectiveFrom: string;
            id: string;
            note?: string | null;
            rateBps: number;
            reasonCode: string;
        };
        RateSet: {
            hostId: string;
            id: string;
            rateBps: number;
        };
        ReadMessagesInput: {
            messageId: string;
        };
        ReadMessagesResult: {
            marked: boolean;
        };
        ReadinessView: {
            blockers: string[];
            ready: boolean;
        };
        RefreshRequest: {
            refreshToken: string;
        };
        RegisterRequest: {
            channel: string;
            cityCode: string;
            contact: string;
            firstName?: string | null;
            language?: string | null;
            lastName?: string | null;
            password: string;
            preferredLanguage?: string | null;
            proof: string;
        };
        RemoveBlockInput: {
            expectedVersion: number;
        };
        RemoveMediaInput: {
            expectedVersion: number;
        };
        RemovedRule: {
            id: string;
        };
        ReplyReviewInput: {
            reply: string;
        };
        ReservationRow: {
            bookingMode: string;
            checkIn?: string | null;
            checkOut?: string | null;
            city: string;
            commissionAmount: string;
            createdAt: string;
            guestName?: string | null;
            id: string;
            nights?: number | null;
            openCases: number;
            operatorId: string;
            operatorName: string;
            operatorNetAmount: string;
            propertyId: string;
            propertyNameEn: string;
            propertyNameFr: string;
            reference: string;
            source: string;
            state: string;
            totalAmount: string;
            unitTypeCoverUrl?: string | null;
            unitTypeId?: string | null;
            unitTypeNameEn?: string | null;
            unitTypeNameFr?: string | null;
        };
        ReservationSummary: {
            awaitingPayment: number;
            cancelled: number;
            checkedIn: number;
            confirmed: number;
            noShow: number;
            requested: number;
            revenue: string;
            total: number;
        };
        ReservationView: {
            adultCount: number;
            approvalDeadlineAt?: string | null;
            checkIn: string;
            checkOut: string;
            childCount: number;
            currency: string;
            discount?: null | components["schemas"]["DiscountView"];
            guest: components["schemas"]["GuestView"];
            holdExpiresAt?: string | null;
            id: string;
            nights: components["schemas"]["NightLine"][];
            paymentRequired: boolean;
            propertyId: string;
            ratePlanId: string;
            reference: string;
            specialRequest?: string | null;
            specialRequestState: string;
            state: string;
            total: string;
            unitTypeCoverUrl?: string | null;
            unitTypeId: string;
            unitTypeImages: components["schemas"]["TripImageView"][];
            version: number;
        };
        RestrictionOperation: {
            action: string;
            basis?: string | null;
            endDate?: string | null;
            kind?: string | null;
            restrictionId?: string | null;
            startDate?: string | null;
            unitTypeId?: string | null;
            value?: number | null;
        };
        RestrictionView: {
            basis: string;
            createdAt: string;
            endDate?: string | null;
            id: string;
            kind: string;
            propertyId: string;
            removedAt?: string | null;
            startDate?: string | null;
            unitTypeId?: string | null;
            value?: number | null;
            version: number;
        };
        ReviewView: {
            comment?: string | null;
            createdAt: string;
            id: string;
            operatorRepliedAt?: string | null;
            operatorReply?: string | null;
            propertyId: string;
            rating: number;
            status: string;
            unitTypeCoverUrl?: string | null;
            unitTypeId?: string | null;
            unitTypeNameEn?: string | null;
            unitTypeNameFr?: string | null;
        };
        RevisionAction: {
            expectedVersion: number;
        };
        RevisionSummary: {
            authorUserId: string;
            createdAt: string;
            id: string;
            propertyId: string;
            reviewGuidance?: string | null;
            reviewReasons: unknown;
            reviewStatus: string;
            reviewedAt?: string | null;
            revisionNumber: number;
            submittedAt?: string | null;
            updatedAt: string;
            version: number;
        };
        RevisionView: components["schemas"]["RevisionSummary"] & {
            content: components["schemas"]["ListingContent"];
        };
        RuleUse: {
            adjustment: string;
            amount: string;
            priority: number;
            reason?: string | null;
            ruleId: string;
            status: string;
        };
        RuleView: {
            adjustmentType: string;
            amount: string;
            endDate?: string | null;
            id: string;
            priority: number;
            ratePlanId: string;
            startDate?: string | null;
            weekdays: number[];
        };
        SavedCatalogItem: {
            code: string;
        };
        SavedSearchRemoved: {
            removedAt: string;
            savedSearchId: string;
        };
        SavedSearchView: {
            alertConsent: boolean;
            createdAt: string;
            id: string;
            parameters: unknown;
        };
        SearchHit: {
            amenities: string[];
            best: components["schemas"]["OfferView"];
            cover?: null | components["schemas"]["CoverView"];
            distanceKm?: number | null;
            id: string;
            instantBooking: boolean;
            name: string;
            offers: components["schemas"]["OfferView"][];
            reviewCount: number;
            verificationStatus: string;
            verifiedRating?: number | null;
        };
        SearchQuery: {
            accessibility: string[];
            adults: number;
            amenities: string[];
            categories: string[];
            checkIn?: string | null;
            checkOut?: string | null;
            childAges: number[];
            children: number;
            instantBooking: boolean;
            maxDistanceKm?: number | null;
            maxPenaltyPercent?: number | null;
            mealPlans: string[];
            minFreeCancellationHours?: number | null;
            origin?: null | components["schemas"]["Origin"];
            page: number;
            pageSize: number;
            paymentCollections: string[];
            priceMaxMinor?: number | null;
            priceMinMinor?: number | null;
            propertyId?: string | null;
            propertyTypes: string[];
            search?: string | null;
            sort: string;
        };
        SearchResponse: {
            dataAsOf: string;
            engineVersion: number;
            facets: components["schemas"]["Facets"];
            fingerprint: string;
            items: components["schemas"]["SearchHit"][];
            meta: components["schemas"]["Meta"];
            page: number;
            pageSize: number;
            query: components["schemas"]["SearchQuery"];
            stats: components["schemas"]["Stats"];
            total: number;
        };
        SendMessageInput: {
            body: string;
        };
        SessionHost: {
            displayName: string;
            hostId: string;
            isOwner: boolean;
        };
        SessionUser: {
            city: string;
            email?: string | null;
            firstName?: string | null;
            id: string;
            lastName?: string | null;
            phone?: string | null;
            preferredLanguage: string;
            role: string;
        };
        SessionView: {
            hosts: components["schemas"]["SessionHost"][];
            permissions: components["schemas"]["PermissionGrantView"][];
            user: components["schemas"]["SessionUser"];
        };
        SetHostOverride: {
            effect: components["schemas"]["OverrideEffect"];
            permission: string;
        };
        SetMemberPermissions: {
            permissions: string[];
        };
        SettingView: {
            changed: boolean;
            defaultValue: number;
            key: string;
            max: number;
            min: number;
            unit: string;
            updatedAt?: string | null;
            updatedByUserId?: string | null;
            value: number;
        };
        SettingsView: {
            city: string;
            items: components["schemas"]["SettingView"][];
        };
        SpecialRequestDecisionInput: {
            decision: string;
            expectedVersion: number;
        };
        StatementInput: {
            body: string;
            party: string;
        };
        StatementRow: {
            body: string;
            createdAt: string;
            id: string;
            party: string;
        };
        Stats: {
            distance?: null | components["schemas"]["DistanceStats"];
            instantBookingCount: number;
            offerCount: number;
            price?: null | components["schemas"]["PriceStats"];
            pricedCount: number;
            resultCount: number;
        };
        SubmitReviewInput: {
            comment?: string | null;
            rating: number;
        };
        TaskView: {
            assigneeUserId?: string | null;
            checklist: string[];
            completedAt?: string | null;
            createdAt: string;
            id: string;
            physicalUnitId?: string | null;
            priority: string;
            propertyId: string;
            reservationItemId?: string | null;
            state: string;
            taskType: string;
            updatedAt: string;
            version: number;
        };
        TemplateInput: {
            accessibility?: string[];
            baseNightlyAmount: string;
            bathroomCount?: number;
            bedCount?: number;
            bedroomCount?: number;
            categoryCode: string;
            descriptionEn?: string;
            descriptionFr?: string;
            expectedVersion?: number | null;
            hostId: string;
            inventoryModel: string;
            maxAdults: number;
            maxChildren?: number;
            nameEn: string;
            nameFr: string;
            status?: string | null;
        };
        TemplateView: {
            accessibility: unknown;
            baseNightlyAmount: string;
            bathroomCount: number;
            bedCount: number;
            bedroomCount: number;
            categoryCode: string;
            cityId: string;
            coverUrl?: string | null;
            descriptionEn: string;
            descriptionFr: string;
            hostId: string;
            id: string;
            imageCount: number;
            inventoryModel: string;
            maxAdults: number;
            maxChildren: number;
            nameEn: string;
            nameFr: string;
            status: string;
            unitTypeCount: number;
            updatedAt: string;
            version: number;
        };
        Timings: {
            candidates: number;
            pricing: number;
            total: number;
        };
        TokenResponse: {
            accessToken: string;
            refreshToken: string;
        };
        TopupInput: {
            amount: string;
            note?: string | null;
            paymentRef?: string | null;
            reasonCode: string;
        };
        TransitionInput: {
            action: string;
            expectedVersion: number;
        };
        TripImageView: {
            altEn: string;
            altFr: string;
            id: string;
            url: string;
        };
        UnitOptionView: {
            id: string;
            localCode: string;
        };
        UnitType: {
            id: string;
            images: components["schemas"]["Page_Image"];
            nameEn: string;
            nameFr: string;
        };
        UnitTypeView: {
            bathroomCount: number;
            bedCount: number;
            bedroomCount: number;
            coverUrl?: string | null;
            description: string;
            id: string;
            images: components["schemas"]["ImageView"][];
            maxAdults: number;
            maxChildren: number;
            name: string;
        };
        UnreadCount: {
            unread: number;
        };
        UpdateHostContact: {
            guestContactEmail: string;
            guestContactPhone: string;
        };
        UpdateMediaInput: {
            altEn?: string | null;
            altFr?: string | null;
            expectedVersion: number;
            isCover?: boolean | null;
            sortOrder?: number | null;
        };
        UpdatePlanInput: {
            bookingMode?: string | null;
            childNightlyAmount?: string | null;
            expectedVersion: number;
            idempotencyKey?: string | null;
            mealPlan?: string | null;
            status?: string | null;
        };
        UpdatePromotionInput: {
            idempotencyKey?: string | null;
            redemptionLimit?: number | null;
            status?: string | null;
        };
        UpdateRevision: {
            content: components["schemas"]["ListingContent"];
            expectedVersion: number;
        };
        UpdateSettingInput: {
            reasonCode: string;
            value: number;
        };
        UpdateUnitTypeInput: {
            accessibility?: string[];
            bathroomCount?: number;
            bedCount?: number;
            bedroomCount?: number;
            categoryCode: string;
            descriptionEn?: string;
            descriptionFr?: string;
            expectedVersion: number;
            idempotencyKey?: string | null;
            maxAdults: number;
            maxChildren?: number;
            nameEn: string;
            nameFr: string;
            sellableQuantity: number;
        };
        UpdateWishlist: {
            name?: string | null;
            visibility?: string | null;
        };
        UpdatedSetting: {
            key: string;
            value: number;
        };
        VerifyOtpInput: {
            channel: string;
            code: string;
            contact: string;
        };
        VersionedAction: {
            expectedVersion: number;
        };
        Violation: {
            code: string;
            params: unknown;
        };
        WishlistItem: {
            addedAt: string;
            name: string;
            propertyId: string;
        };
        WishlistView: {
            createdAt: string;
            id: string;
            itemCount: number;
            name: string;
            visibility: string;
        };
    };
    responses: never;
    parameters: never;
    requestBodies: never;
    headers: never;
    pathItems: never;
}
export type $defs = Record<string, never>;
export interface operations {
    admin_accommodation_categories_list: {
        parameters: {
            query?: {
                search?: string;
                status?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_CategoryView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_accommodation_categories_create: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CategoryInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CategoryView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_accommodation_categories_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                code: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CategoryInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CategoryView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_accommodation_templates_list: {
        parameters: {
            query?: {
                hostId?: string;
                search?: string;
                status?: string;
                categoryCode?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_TemplateView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_accommodation_templates_create: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["TemplateInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_TemplateView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_accommodation_templates_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                templateId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["TemplateInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_TemplateView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_template_media_list: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                templateId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Vec_PropertyMediaView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_template_media_add: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                templateId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["AddMediaInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_template_media_remove: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                templateId: string;
                mediaId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RemoveMediaInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaRemoved"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_template_media_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                templateId: string;
                mediaId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["UpdateMediaInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_template_media_content: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                templateId: string;
                mediaId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaContent"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_amenities_list: {
        parameters: {
            query?: {
                locale?: string;
                search?: string;
                status?: string;
                groupCode?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_Amenity"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_amenities_create: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["AmenityWrite"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_SavedCatalogItem"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_amenities_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                code: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["AmenityWrite"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_SavedCatalogItem"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_amenity_groups_list: {
        parameters: {
            query?: {
                locale?: string;
                search?: string;
                status?: string;
                groupCode?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_AmenityGroup"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_amenity_groups_create: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["GroupWrite"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_SavedCatalogItem"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_amenity_groups_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                code: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["GroupWrite"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_SavedCatalogItem"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_inventory_blocks_list: {
        parameters: {
            query?: {
                propertyId?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_BlockView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_inventory_blocks_create: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateBlockInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_BlockView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_inventory_blocks_remove: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                blockId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RemoveBlockInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_BlockView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_inventory_calendar_get: {
        parameters: {
            query: {
                propertyId: string;
                from: string;
                to: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_InventoryCalendarView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_cases_list: {
        parameters: {
            query?: {
                state?: string;
                caseType?: string;
                hostId?: string;
                overdue?: boolean;
                escalated?: boolean;
                assignee?: string;
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_AdminCaseItem"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_cases_open: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["AdminOpenCaseInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AdminCaseItem"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_case_assignees_list: {
        parameters: {
            query?: {
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_CaseAssignee"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_cases_evidence: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                evidenceId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AdminEvidenceContent"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_cases_get: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AdminCaseDetail"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_cases_assign: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["AssignCaseInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AdminCaseItem"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_cases_close: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CloseCaseInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AdminCaseItem"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_cases_escalate: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["EscalateCaseInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AdminCaseItem"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_claims_list: {
        parameters: {
            query?: {
                status?: string;
                hostId?: string;
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_ClaimRow"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_claims_summary: {
        parameters: {
            query?: {
                status?: string;
                hostId?: string;
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ClaimSummary"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_claims_get: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ClaimDetail"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_claims_decide: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["DecideInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ClaimDecisionResult"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_claims_payout: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["PayoutInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PayoutResult"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_claims_statement: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["StatementInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_StatementRow"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_commission_rate_current: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                hostId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CurrentRate"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_commission_rate_set: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                hostId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RateInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_RateSet"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_commission_rate_history: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                hostId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_RateRow"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_credit_accounts_list: {
        parameters: {
            query?: {
                hostId?: string;
                status?: string;
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_AccountRow"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_credit_detail: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                hostId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CreditDetail"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_credit_adjust: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                hostId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["AdjustInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_BalanceView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_credit_ledger: {
        parameters: {
            query?: {
                type?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                hostId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_AdminLedgerRow"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_credit_topup: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                hostId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["TopupInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_BalanceView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_credit_summary: {
        parameters: {
            query?: {
                hostId?: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CreditSummary"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_dashboard_summary: {
        parameters: {
            query?: {
                from?: string;
                to?: string;
                hostId?: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_DashboardSummary"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_dashboard_hosts: {
        parameters: {
            query?: {
                from?: string;
                to?: string;
                hostId?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_HostFigures"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_admin_documents: {
        parameters: {
            query?: {
                kind?: string;
                entityId?: string;
                page?: number;
                pageSize?: number;
            };
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_DocumentView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_admin_document_preview: {
        parameters: {
            query?: {
                locale?: string;
            };
            header?: never;
            path: {
                kind: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/pdf": number[];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_admin_document_download: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/pdf": number[];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_guests_list: {
        parameters: {
            query?: {
                search?: string;
                flag?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_GuestItem"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_guest_detail: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                userId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_GuestItem"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_guest_bookings: {
        parameters: {
            query?: {
                city?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                userId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_GuestBooking"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_host_applications_list: {
        parameters: {
            query?: {
                status?: string;
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_AdminApplicationView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_host_application_detail: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                applicationId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ApplicationView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_host_application_decide: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                applicationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["HostDecisionInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ApplicationView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_host_document_review: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                applicationId: string;
                documentId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["DocumentReview"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_HostDocumentView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_hosts_list: {
        parameters: {
            query?: {
                status?: string;
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_AdminHostView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_host_status_change: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                hostId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["HostStatusInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_HostStatusView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_inventory_defaults_update: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["InventoryDefaultsInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_InventoryDefaultsResult"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_listing_queue: {
        parameters: {
            query?: {
                status?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_QueueItem"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_listing_revision_get: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                revisionId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AdminRevisionView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_listing_decide: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                revisionId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ListingDecisionInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_RevisionView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_property_media_content: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                mediaId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaContent"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_admin_preferences: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PreferenceList"];
                };
            };
        };
    };
    notifications_admin_set_preference: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["PreferenceView"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PreferenceView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_admin_list: {
        parameters: {
            query?: {
                locale?: string;
                unreadOnly?: boolean;
                page?: number;
                pageSize?: number;
            };
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_NotificationView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_admin_mark_all_read: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MarkedCount"];
                };
            };
        };
    };
    notifications_admin_unread_count: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_UnreadCount"];
                };
            };
        };
    };
    notifications_admin_mark_read: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path: {
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MarkedCount"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_properties_list: {
        parameters: {
            query?: {
                status?: string;
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_AdminPropertyItem"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_properties_create: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreatePropertyInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_properties_get: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AdminPropertyDetail"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_properties_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["AdminUpdateInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_properties_lifecycle: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["LifecycleInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_property_media_list: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Vec_PropertyMediaView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_property_media_add: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["AddMediaInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_property_media_remove: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                mediaId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RemoveMediaInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaRemoved"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_property_media_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                mediaId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["UpdateMediaInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_price_preview: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["QuoteRequest"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PricePreviewView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_promotions_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_AdminPromotionView"];
                };
            };
        };
    };
    admin_promotions_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                promotionId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["UpdatePromotionInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PromotionView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_promotions_funding_set: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                promotionId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["FundingInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AdminPromotionView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_rate_plans_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_PlanView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_rate_plans_create: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreatePlanInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PlanView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_rate_plans_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                ratePlanId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["UpdatePlanInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PlanView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_rate_rules_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                propertyId: string;
                ratePlanId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_RuleView"];
                };
            };
        };
    };
    admin_rate_rules_create: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                ratePlanId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateRuleInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_RuleView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_rate_rules_remove: {
        parameters: {
            query?: {
                idempotencyKey?: string;
            };
            header?: never;
            path: {
                propertyId: string;
                ratePlanId: string;
                ruleId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_RemovedRule"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_properties_reinstate: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ModerationInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_properties_revisions: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_RevisionSummary"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_properties_suspend: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ModerationInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_inventory_unit_types_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_UnitTypeView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_inventory_unit_types_create: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateUnitTypeInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_UnitTypeView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_inventory_unit_types_archive: {
        parameters: {
            query?: {
                expectedVersion?: number;
            };
            header?: never;
            path: {
                propertyId: string;
                unitTypeId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_UnitTypeView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_inventory_unit_types_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                unitTypeId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["UpdateUnitTypeInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_UnitTypeView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_admin_register_token: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["PushTokenInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MarkedCount"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_admin_remove_token: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["PushTokenRemoval"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MarkedCount"];
                };
            };
        };
    };
    admin_rate_policies_list: {
        parameters: {
            query?: {
                kind?: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Vec_PolicyView"];
                };
            };
        };
    };
    admin_reports_summary: {
        parameters: {
            query?: {
                from?: string;
                to?: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AnalyticsSummary"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_reservations_list: {
        parameters: {
            query?: {
                state?: string;
                hostId?: string;
                propertyId?: string;
                checkInFrom?: string;
                checkInTo?: string;
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_ReservationRow"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_reservations_export: {
        parameters: {
            query?: {
                state?: string;
                hostId?: string;
                propertyId?: string;
                checkInFrom?: string;
                checkInTo?: string;
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_ReservationRow"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_reservations_summary: {
        parameters: {
            query?: {
                state?: string;
                hostId?: string;
                propertyId?: string;
                checkInFrom?: string;
                checkInTo?: string;
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReservationSummary"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_reservations_detail: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Detail"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_reservations_cancel: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["PlatformCancelInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReservationView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_reservations_cancel_preview: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CancelPreview"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_inventory_restrictions_bulk: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["BulkRestrictionInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_BulkRestrictionResult"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reviews_admin_list: {
        parameters: {
            query?: {
                status?: string;
                rating?: number;
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: {
                "x-city"?: string | null;
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_AdminReviewItem"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reviews_admin_queue: {
        parameters: {
            query?: {
                status?: string;
                page?: number;
                pageSize?: number;
            };
            header?: {
                "x-city"?: string | null;
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_ReviewView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reviews_admin_decide: {
        parameters: {
            query?: never;
            header?: {
                "x-city"?: string | null;
                "x-request-id"?: string | null;
            };
            path: {
                reviewId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["DecideReviewInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReviewView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_settings_list: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_SettingsView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_settings_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                key: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["UpdateSettingInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_UpdatedSetting"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_tasks_list: {
        parameters: {
            query?: {
                state?: string;
                taskType?: string;
                priority?: string;
                hostId?: string;
                propertyId?: string;
                assignee?: string;
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_AdminTaskItem"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_tasks_create: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateTaskInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AdminTaskItem"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_tasks_get: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                taskId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AdminTaskItem"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    admin_tasks_transition: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                taskId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["TransitionInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AdminTaskItem"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    amendments_accept: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                amendmentId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReservationView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    amendments_decline: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                amendmentId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReservationView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cases_open_guest: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["OpenCaseInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CaseView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cases_evidence_get_guest: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                evidenceId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_EvidenceContent"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cases_mine: {
        parameters: {
            query?: {
                state?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_CaseView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cases_get_guest: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CaseView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cases_close_guest: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CloseCaseInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CaseView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cases_evidence_list_guest: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Vec_EvidenceView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cases_evidence_add_guest: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["AddEvidenceInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_EvidenceView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_guest_documents: {
        parameters: {
            query?: {
                kind?: string;
                entityId?: string;
                page?: number;
                pageSize?: number;
            };
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_DocumentView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_guest_document_download: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/pdf": number[];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    guest_invitations_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_MyInvitationView"];
                };
            };
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    guest_invitation_accept: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                invitationId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_InvitationView"];
                };
            };
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_guest_preferences: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PreferenceList"];
                };
            };
        };
    };
    notifications_guest_set_preference: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["PreferenceView"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PreferenceView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_guest_list: {
        parameters: {
            query?: {
                locale?: string;
                unreadOnly?: boolean;
                page?: number;
                pageSize?: number;
            };
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_NotificationView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_guest_mark_all_read: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MarkedCount"];
                };
            };
        };
    };
    notifications_guest_unread_count: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_UnreadCount"];
                };
            };
        };
    };
    notifications_guest_mark_read: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path: {
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MarkedCount"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    auth_profile_update: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ProfileInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MessageResponse"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_guest_register_token: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["PushTokenInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MarkedCount"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_guest_remove_token: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["PushTokenRemoval"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MarkedCount"];
                };
            };
        };
    };
    guest_quotes_create: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["QuoteRequest"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_QuoteView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    guest_quotes_get: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                quoteId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_QuoteView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reservations_hold: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["HoldRequest"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReservationView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reservations_mine: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
                tab?: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MyReservationsPage"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reservations_get: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReservationView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reservation_amendments_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_AmendmentListItem"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reservation_amendments_propose: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ProposeInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AmendmentProposal"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reservations_cancel: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CancelStayInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CancelledStayView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reservations_cancellation: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CancellationView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reservation_messages_list_guest: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_MessageView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reservation_messages_send_guest: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["SendMessageInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MessageView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reviews_submit: {
        parameters: {
            query?: never;
            header?: {
                "x-city"?: string | null;
                "x-request-id"?: string | null;
            };
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["SubmitReviewInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReviewView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reservations_withdraw: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["VersionedAction"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReservationView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    guests_saved_searches_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_SavedSearchView"];
                };
            };
        };
    };
    guests_saved_searches_create: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateSavedSearch"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_SavedSearchView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    guests_saved_searches_delete: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                savedSearchId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_SavedSearchRemoved"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    auth_session: {
        parameters: {
            query?: never;
            header?: {
                "x-city"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_SessionView"];
                };
            };
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    auth_logout: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MessageResponse"];
                };
            };
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    guests_wishlists_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_WishlistView"];
                };
            };
        };
    };
    guests_wishlists_create: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateWishlist"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_WishlistView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    guests_wishlists_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                wishlistId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["UpdateWishlist"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_WishlistView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    guests_wishlist_items_list: {
        parameters: {
            query?: {
                propertyIds?: string;
                locale?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                wishlistId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_WishlistItem"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    guests_wishlist_items_add: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                wishlistId: string;
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_WishlistView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    guests_wishlist_items_remove: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                wishlistId: string;
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_WishlistView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_accommodation_templates_list: {
        parameters: {
            query?: {
                hostId?: string;
                search?: string;
                status?: string;
                categoryCode?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_TemplateView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_accommodation_templates_create: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["TemplateInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_TemplateView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_accommodation_templates_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                templateId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["TemplateInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_TemplateView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_template_media_list: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                templateId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Vec_PropertyMediaView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_template_media_add: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                templateId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["AddMediaInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_template_media_remove: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                templateId: string;
                mediaId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RemoveMediaInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaRemoved"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_template_media_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                templateId: string;
                mediaId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["UpdateMediaInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_template_media_content: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                templateId: string;
                mediaId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaContent"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_analytics_summary: {
        parameters: {
            query: {
                hostId: string;
                propertyId?: string;
                from?: string;
                to?: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AnalyticsSummary"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_applications_mine: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_ApplicationView"];
                };
            };
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_application_save: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ApplicationInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ApplicationView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_application_document_add: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                applicationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["AddDocumentInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_HostDocumentView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_application_document_remove: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                applicationId: string;
                documentId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_OperationNotice"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_application_submit: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                applicationId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ApplicationView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_application_withdraw: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                applicationId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ApplicationView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_availability_get: {
        parameters: {
            query: {
                propertyId: string;
                unitTypeId: string;
                checkIn: string;
                checkOut: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AvailabilityView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_blocks_create: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateBlockInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_BlockView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_blocks_get: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                blockId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_BlockView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_blocks_remove: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                blockId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RemoveBlockInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_BlockView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_calendar_get: {
        parameters: {
            query: {
                propertyId: string;
                from: string;
                to: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_InventoryCalendarView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_feed_tokens_list: {
        parameters: {
            query: {
                propertyId: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_FeedTokenView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_feed_tokens_create: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateFeedTokenInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_FeedTokenCreated"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_feed_tokens_revoke: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                tokenId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_FeedTokenRevoked"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_calendar_feed: {
        parameters: {
            query: {
                propertyId: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "text/calendar": string;
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_defaults_update: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["InventoryDefaultsInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_InventoryDefaultsResult"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cases_list_host: {
        parameters: {
            query?: {
                hostId?: string;
                state?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_CaseView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cases_open_host: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["OpenCaseInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CaseView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cases_evidence_get_host: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                evidenceId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_EvidenceContent"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cases_get_host: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CaseView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cases_close_host: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CloseCaseInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CaseView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cases_evidence_list_host: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Vec_EvidenceView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cases_evidence_add_host: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                caseId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["AddEvidenceInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_EvidenceView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_credits_summary: {
        parameters: {
            query: {
                hostId: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MineSummary"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_credits_ledger: {
        parameters: {
            query: {
                hostId: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_MineLedgerRow"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_dashboard_summary: {
        parameters: {
            query: {
                hostId: string;
                from?: string;
                to?: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_DashboardFigures"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_host_documents: {
        parameters: {
            query?: {
                kind?: string;
                entityId?: string;
                page?: number;
                pageSize?: number;
            };
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_DocumentView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_host_document_download: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/pdf": number[];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_hosts_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: {
                "x-city"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_HostMembership"];
                };
            };
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_contact_get: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                hostId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_HostContact"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_contact_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                hostId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["UpdateHostContact"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_HostContact"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_invitations_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                hostId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_InvitationView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_invitation_create: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                hostId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateInvitation"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_InvitationView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_invitation_revoke: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                hostId: string;
                invitationId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MessageResponse"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_members_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                hostId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_MemberView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_member_remove: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                hostId: string;
                userId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MessageResponse"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_member_override_set: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                hostId: string;
                userId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["SetHostOverride"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_HostOverride"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_member_override_clear: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                hostId: string;
                userId: string;
                overrideId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MessageResponse"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_member_permissions_set: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                hostId: string;
                userId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["SetMemberPermissions"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MemberView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_property_media_content: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                mediaId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaContent"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_message_conversations: {
        parameters: {
            query: {
                hostId: string;
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_ConversationView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_host_preferences: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PreferenceList"];
                };
            };
        };
    };
    notifications_host_set_preference: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["PreferenceView"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PreferenceView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_host_list: {
        parameters: {
            query?: {
                locale?: string;
                unreadOnly?: boolean;
                page?: number;
                pageSize?: number;
            };
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_NotificationView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_host_mark_all_read: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MarkedCount"];
                };
            };
        };
    };
    notifications_host_unread_count: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_UnreadCount"];
                };
            };
        };
    };
    notifications_host_mark_read: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path: {
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MarkedCount"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_price_preview: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["QuoteRequest"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PricePreviewView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_properties_list: {
        parameters: {
            query?: {
                status?: string;
                hostId?: string;
                search?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_PropertyView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_properties_create: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreatePropertyInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_properties_get: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_blocks_list: {
        parameters: {
            query?: {
                propertyId?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_BlockView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_property_lifecycle: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["LifecycleInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_property_media_list: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Vec_PropertyMediaView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_property_media_add: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["AddMediaInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_property_media_remove: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                mediaId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RemoveMediaInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaRemoved"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_property_media_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                mediaId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["UpdateMediaInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyMediaView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_promotions_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_PromotionView"];
                };
            };
        };
    };
    host_promotions_create: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreatePromotionInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PromotionView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_promotions_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                promotionId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["UpdatePromotionInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PromotionView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_rate_plans_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_PlanView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_rate_plans_create: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreatePlanInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PlanView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_rate_plans_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                ratePlanId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["UpdatePlanInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PlanView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_rate_rules_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                propertyId: string;
                ratePlanId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_RuleView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_rate_rules_create: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                ratePlanId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateRuleInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_RuleView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_rate_rules_remove: {
        parameters: {
            query?: {
                idempotencyKey?: string;
            };
            header?: never;
            path: {
                propertyId: string;
                ratePlanId: string;
                ruleId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_RemovedRule"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_restrictions_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_RestrictionView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reviews_host_list: {
        parameters: {
            query?: {
                status?: string;
                page?: number;
                pageSize?: number;
            };
            header?: {
                "x-city"?: string | null;
                "x-request-id"?: string | null;
            };
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_ReviewView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_revisions_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_RevisionSummary"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_revisions_create: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateRevision"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_RevisionView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_revisions_get: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                revisionId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_RevisionView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_revisions_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                revisionId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["UpdateRevision"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_RevisionView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_revisions_readiness: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                revisionId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReadinessView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_revisions_submit: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                revisionId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RevisionAction"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_RevisionView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_revisions_withdraw: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                revisionId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RevisionAction"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_RevisionView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_unit_types_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_UnitTypeView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_unit_types_create: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateUnitTypeInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_UnitTypeView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_unit_types_archive: {
        parameters: {
            query?: {
                expectedVersion?: number;
            };
            header?: never;
            path: {
                propertyId: string;
                unitTypeId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_UnitTypeView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_unit_types_update: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                propertyId: string;
                unitTypeId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["UpdateUnitTypeInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_UnitTypeView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_host_register_token: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["PushTokenInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MarkedCount"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    notifications_host_remove_token: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["PushTokenRemoval"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MarkedCount"];
                };
            };
        };
    };
    host_rate_policies_list: {
        parameters: {
            query?: {
                kind?: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Vec_PolicyView"];
                };
            };
        };
    };
    inventory_assignments_history: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                itemId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Vec_AssignmentView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_assignments_options: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                itemId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Vec_UnitOptionView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_reservations_list: {
        parameters: {
            query?: {
                hostId?: string;
                propertyId?: string;
                state?: string;
                search?: string;
                checkInFrom?: string;
                checkInTo?: string;
                locale?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_OperatorReservationView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_reservations_export: {
        parameters: {
            query?: {
                hostId?: string;
                propertyId?: string;
                state?: string;
                search?: string;
                checkInFrom?: string;
                checkInTo?: string;
                locale?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_OperatorReservationView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_reservations_get: {
        parameters: {
            query?: {
                locale?: string;
            };
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_OperatorReservationView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_reservations_amend: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ProposeInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AmendmentProposal"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_reservations_cancel: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["OperatorCancelInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReservationView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_reservations_check_in: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["VersionedAction"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReservationView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_reservations_check_out: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["VersionedAction"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReservationView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_reservations_decide: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["DecisionInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReservationView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reservation_messages_list_host: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_MessageView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reservation_messages_send_host: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["SendMessageInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MessageView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_messages_mark_read: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ReadMessagesInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReadMessagesResult"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_reservations_no_show: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["VersionedAction"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReservationView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    host_reservations_special_request: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                reservationId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["SpecialRequestDecisionInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReservationView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_restrictions_bulk: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["BulkRestrictionInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_BulkRestrictionResult"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reviews_reply: {
        parameters: {
            query?: never;
            header?: {
                "x-city"?: string | null;
                "x-request-id"?: string | null;
            };
            path: {
                reviewId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ReplyReviewInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ReviewView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    operations_tasks_list_host: {
        parameters: {
            query?: {
                propertyId?: string;
                state?: string;
                taskType?: string;
                page?: number;
                pageSize?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_TaskView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    operations_tasks_create_host: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["CreateTaskInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_TaskView"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    operations_tasks_get_host: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                taskId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_TaskView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    operations_tasks_transition_host: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                taskId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["TransitionInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_TaskView"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_assignments_assign: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["AssignUnitInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AssignmentView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_assignments_move: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                assignmentId: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["MoveUnitInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AssignmentView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    public_accommodation_categories: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Vec_CategoryView"];
                };
            };
        };
    };
    amenities_list: {
        parameters: {
            query?: {
                locale?: string;
                groupCode?: string;
                page?: number;
                pageSize?: number;
            };
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_Amenity"];
                };
            };
            400: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            500: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    amenity_groups_list: {
        parameters: {
            query?: {
                locale?: string;
                page?: number;
                pageSize?: number;
            };
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_AmenityGroup"];
                };
            };
            400: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            500: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    auth_login: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["LoginRequest"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_TokenResponse"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    auth_otp_channel: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_OtpChannelResponse"];
                };
            };
        };
    };
    auth_refresh: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RefreshRequest"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_TokenResponse"];
                };
            };
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    auth_register: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RegisterRequest"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_TokenResponse"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    auth_register_otp: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["IssueOtpInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_OtpChannelResponse"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    auth_register_verify: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["VerifyOtpInput"];
            };
        };
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_ProofResponse"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    inventory_feed_by_token: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                city: string;
                token: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "text/calendar": string;
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    cities_list: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Vec_CityView"];
                };
            };
        };
    };
    city_context: {
        parameters: {
            query?: never;
            header?: {
                "x-city"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CityView"];
                };
            };
        };
    };
    guests_destinations_list: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Vec_DestinationView"];
                };
            };
        };
    };
    health_get: {
        parameters: {
            query?: never;
            header?: {
                "x-request-id"?: string | null;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["HealthResponse"];
                };
            };
            503: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["HealthResponse"];
                };
            };
        };
    };
    host_reference: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_HostReferenceView"];
                };
            };
        };
    };
    languages_list: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Vec_String"];
                };
            };
        };
    };
    guests_media_content: {
        parameters: {
            query?: {
                locale?: string;
            };
            header?: never;
            path: {
                mediaId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_MediaContentView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    permission_catalog: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CatalogView"];
                };
            };
        };
    };
    guests_properties_search: {
        parameters: {
            query?: {
                propertyId?: string;
                search?: string;
                checkIn?: string;
                checkOut?: string;
                adults?: string;
                children?: string;
                childAges?: string;
                instantBooking?: string;
                propertyTypes?: string;
                mealPlans?: string;
                paymentCollections?: string;
                accessibility?: string;
                amenities?: string;
                categories?: string;
                priceMin?: string;
                priceMax?: string;
                minFreeCancellationHours?: string;
                maxPenaltyPercent?: string;
                latitude?: string;
                longitude?: string;
                maxDistanceKm?: string;
                sort?: string;
                page?: string;
                pageSize?: string;
                analyticsConsent?: string;
                locale?: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_SearchResponse"];
                };
            };
        };
    };
    guests_property_get: {
        parameters: {
            query?: {
                locale?: string;
            };
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_PropertyDetailView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    guests_property_availability: {
        parameters: {
            query?: {
                checkIn?: string;
                checkOut?: string;
                adults?: string;
                children?: string;
            };
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_AvailabilityView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    guests_property_calendar: {
        parameters: {
            query: {
                unitTypeId: string;
                from: string;
                to: string;
            };
            header?: never;
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_CalendarView"];
                };
            };
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    reviews_public_list: {
        parameters: {
            query?: {
                page?: number;
                pageSize?: number;
            };
            header?: {
                "x-city"?: string | null;
                "x-request-id"?: string | null;
            };
            path: {
                propertyId: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    "x-request-id"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Page_ReviewView"];
                };
            };
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ErrorBody"];
                };
            };
        };
    };
    guests_search_catalog: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Envelope_Value"];
                };
            };
        };
    };
}
