ALTER TABLE kernel_outbox ADD COLUMN next_attempt_at timestamptz NOT NULL DEFAULT now();

CREATE INDEX kernel_outbox_retry_due_idx
    ON kernel_outbox (next_attempt_at, created_at, id)
    WHERE state IN ('PENDING', 'FAILED');
