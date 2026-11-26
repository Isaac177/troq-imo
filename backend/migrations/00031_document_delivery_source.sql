ALTER TABLE messaging_document ADD COLUMN source_delivery_id text;

CREATE UNIQUE INDEX messaging_document_source_delivery_idx
    ON messaging_document (source_delivery_id) WHERE source_delivery_id IS NOT NULL;
