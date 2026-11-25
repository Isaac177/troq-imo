ALTER TABLE hosts
    ADD COLUMN guest_contact_phone varchar(32) NOT NULL DEFAULT '',
    ADD COLUMN guest_contact_email varchar(254) NOT NULL DEFAULT '';
