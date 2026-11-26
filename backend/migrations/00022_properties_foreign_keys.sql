ALTER TABLE stay_unit_types
    ADD CONSTRAINT stay_unit_types_property_fkey FOREIGN KEY ("propertyId") REFERENCES stay_properties (id),
    ADD CONSTRAINT stay_unit_types_category_fkey FOREIGN KEY ("categoryCode") REFERENCES stay_accommodation_categories (code),
    ADD CONSTRAINT stay_unit_types_template_fkey FOREIGN KEY ("templateId") REFERENCES stay_accommodation_templates (id);

ALTER TABLE stay_physical_units
    ADD CONSTRAINT stay_physical_units_property_fkey FOREIGN KEY ("propertyId") REFERENCES stay_properties (id);

ALTER TABLE stay_inventory_pools
    ADD CONSTRAINT stay_inventory_pools_property_fkey FOREIGN KEY ("propertyId") REFERENCES stay_properties (id);

ALTER TABLE stay_availability_blocks
    ADD CONSTRAINT stay_availability_blocks_property_fkey FOREIGN KEY ("propertyId") REFERENCES stay_properties (id);

ALTER TABLE stay_restrictions
    ADD CONSTRAINT stay_restrictions_property_fkey FOREIGN KEY ("propertyId") REFERENCES stay_properties (id);

ALTER TABLE stay_calendar_feed_tokens
    ADD CONSTRAINT stay_calendar_feed_tokens_property_fkey FOREIGN KEY ("propertyId") REFERENCES stay_properties (id);

ALTER TABLE stay_wishlist_items
    ADD CONSTRAINT stay_wishlist_items_property_fkey FOREIGN KEY ("propertyId") REFERENCES stay_properties (id);

ALTER TABLE stay_rate_plans
    ADD CONSTRAINT stay_rate_plans_property_fkey FOREIGN KEY ("propertyId") REFERENCES stay_properties (id);

ALTER TABLE stay_reviews
    ADD CONSTRAINT stay_reviews_property_fkey FOREIGN KEY ("propertyId") REFERENCES stay_properties (id);
