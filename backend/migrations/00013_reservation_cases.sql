CREATE TABLE stay_cases (
    id text NOT NULL,
    city_id text NOT NULL,
    "operatorId" text NOT NULL,
    "reservationId" text,
    "caseType" character varying(24) NOT NULL,
    reason text NOT NULL DEFAULT '',
    parties jsonb NOT NULL,
    "evidenceRefs" jsonb NOT NULL DEFAULT '[]'::jsonb,
    state character varying(24) NOT NULL DEFAULT 'OPEN',
    "deadlineAt" timestamp with time zone NOT NULL,
    resolution jsonb,
    "claimAmount" numeric(14,2),
    "assignedToUserId" text,
    "assignedAt" timestamp with time zone,
    "escalatedAt" timestamp with time zone,
    "escalatedByUserId" text,
    "escalationReason" text,
    version integer NOT NULL DEFAULT 1,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    "updatedAt" timestamp with time zone NOT NULL DEFAULT now(),
    "closedAt" timestamp with time zone,
    CONSTRAINT stay_cases_pkey PRIMARY KEY (id),
    CONSTRAINT stay_cases_operator_fkey FOREIGN KEY ("operatorId") REFERENCES hosts (id),
    CONSTRAINT stay_cases_reservation_fkey FOREIGN KEY ("reservationId") REFERENCES stay_reservations (id),
    CONSTRAINT stay_cases_type_check CHECK ("caseType" IN (
        'GUEST_COMPLAINT', 'OPERATOR_COMPLAINT', 'DAMAGE_CLAIM', 'REFUND_DISPUTE', 'NO_SHOW_DISPUTE', 'GENERAL'
    )),
    CONSTRAINT stay_cases_state_check CHECK (state IN ('OPEN', 'AWAITING_RESPONSE', 'RESOLVED', 'CLOSED')),
    CONSTRAINT stay_cases_closed_check CHECK ((state IN ('RESOLVED', 'CLOSED')) = ("closedAt" IS NOT NULL)),
    CONSTRAINT stay_cases_version_check CHECK (version >= 1),
    CONSTRAINT stay_cases_claim_amount_check CHECK ("claimAmount" IS NULL OR "claimAmount" > 0),
    CONSTRAINT stay_cases_assignment_check CHECK (("assignedToUserId" IS NULL) = ("assignedAt" IS NULL)),
    CONSTRAINT stay_cases_escalation_check CHECK (("escalatedAt" IS NULL) = ("escalatedByUserId" IS NULL))
);

CREATE INDEX stay_cases_operator_state_idx ON stay_cases ("operatorId", state, "deadlineAt");
CREATE INDEX stay_cases_assignee_idx ON stay_cases ("assignedToUserId") WHERE "assignedToUserId" IS NOT NULL;
CREATE INDEX stay_cases_city_state_idx ON stay_cases (city_id, state, "deadlineAt");

CREATE TABLE stay_case_evidence (
    id text NOT NULL,
    "caseId" text NOT NULL,
    "fileName" character varying(200) NOT NULL,
    "mimeType" character varying(64) NOT NULL,
    "sizeBytes" integer NOT NULL,
    content bytea NOT NULL,
    "addedByUserId" text NOT NULL,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_case_evidence_pkey PRIMARY KEY (id),
    CONSTRAINT stay_case_evidence_case_fkey FOREIGN KEY ("caseId") REFERENCES stay_cases (id),
    CONSTRAINT stay_case_evidence_mime_check CHECK ("mimeType" IN ('application/pdf', 'image/png', 'image/jpeg')),
    CONSTRAINT stay_case_evidence_size_check CHECK ("sizeBytes" BETWEEN 1 AND 5242880)
);

CREATE INDEX stay_case_evidence_case_idx ON stay_case_evidence ("caseId", "createdAt");

CREATE TABLE stay_claim_statements (
    id text NOT NULL,
    "caseId" text NOT NULL,
    party character varying(8) NOT NULL,
    body text NOT NULL,
    "recordedByUserId" text NOT NULL,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_claim_statements_pkey PRIMARY KEY (id),
    CONSTRAINT stay_claim_statements_case_fkey FOREIGN KEY ("caseId") REFERENCES stay_cases (id),
    CONSTRAINT stay_claim_statements_party_check CHECK (party IN ('HOST', 'GUEST')),
    CONSTRAINT stay_claim_statements_body_check CHECK (char_length(body) BETWEEN 1 AND 2000)
);

CREATE INDEX stay_claim_statements_case_idx ON stay_claim_statements ("caseId", "createdAt");

CREATE TABLE stay_claim_decisions (
    "caseId" text NOT NULL,
    decision character varying(16) NOT NULL,
    "approvedAmount" numeric(14,2) NOT NULL,
    "reasonCode" character varying(48) NOT NULL,
    "decidedByUserId" text NOT NULL,
    "decidedAt" timestamp with time zone NOT NULL DEFAULT now(),
    "payoutState" character varying(16) NOT NULL,
    "payoutRef" text,
    "payoutUpdatedAt" timestamp with time zone,
    CONSTRAINT stay_claim_decisions_pkey PRIMARY KEY ("caseId"),
    CONSTRAINT stay_claim_decisions_case_fkey FOREIGN KEY ("caseId") REFERENCES stay_cases (id),
    CONSTRAINT stay_claim_decisions_decision_check CHECK (decision IN ('APPROVED', 'PARTIAL', 'REJECTED')),
    CONSTRAINT stay_claim_decisions_amount_check CHECK ("approvedAmount" >= 0 AND ((decision = 'REJECTED') = ("approvedAmount" = 0))),
    CONSTRAINT stay_claim_decisions_payout_check CHECK ("payoutState" IN ('NOT_DUE', 'PENDING', 'PAID', 'FAILED')),
    CONSTRAINT stay_claim_decisions_payout_due_check CHECK ((decision = 'REJECTED') = ("payoutState" = 'NOT_DUE'))
);
