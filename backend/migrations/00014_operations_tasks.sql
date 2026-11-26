CREATE TABLE stay_operations_tasks (
    id text NOT NULL,
    city_id text NOT NULL,
    "propertyId" text NOT NULL,
    "physicalUnitId" text,
    "reservationItemId" text,
    "taskType" character varying(24) NOT NULL,
    "assigneeUserId" text,
    state character varying(16) NOT NULL DEFAULT 'OPEN',
    priority character varying(16) NOT NULL DEFAULT 'NORMAL',
    checklist jsonb NOT NULL DEFAULT '[]'::jsonb,
    attachments jsonb NOT NULL DEFAULT '[]'::jsonb,
    version integer NOT NULL DEFAULT 1,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    "updatedAt" timestamp with time zone NOT NULL DEFAULT now(),
    "completedAt" timestamp with time zone,
    CONSTRAINT stay_operations_tasks_pkey PRIMARY KEY (id),
    CONSTRAINT stay_operations_tasks_item_fkey FOREIGN KEY ("reservationItemId") REFERENCES stay_reservation_items (id),
    CONSTRAINT stay_operations_tasks_type_check CHECK ("taskType" IN (
        'CHECK_IN', 'CHECK_OUT', 'HOUSEKEEPING', 'MAINTENANCE', 'SERVICE_REQUEST'
    )),
    CONSTRAINT stay_operations_tasks_state_check CHECK (state IN ('OPEN', 'IN_PROGRESS', 'DONE', 'CANCELLED')),
    CONSTRAINT stay_operations_tasks_priority_check CHECK (priority IN ('LOW', 'NORMAL', 'HIGH', 'URGENT')),
    CONSTRAINT stay_operations_tasks_completed_check CHECK ((state IN ('DONE', 'CANCELLED')) = ("completedAt" IS NOT NULL)),
    CONSTRAINT stay_operations_tasks_version_check CHECK (version >= 1)
);

CREATE INDEX stay_operations_tasks_property_state_idx ON stay_operations_tasks ("propertyId", state, priority);
CREATE INDEX stay_operations_tasks_assignee_idx ON stay_operations_tasks ("assigneeUserId") WHERE "assigneeUserId" IS NOT NULL;
