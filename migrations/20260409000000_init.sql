-- Schéma initial einvoice-rs
-- Tables : invoices + invoice_events

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- --------------------------------------------------------------------------
-- Table invoices
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS invoices (
    id            UUID PRIMARY KEY,
    number        TEXT NOT NULL,
    issue_date    DATE NOT NULL,
    due_date      DATE,
    currency      CHAR(3) NOT NULL,

    seller_name   TEXT NOT NULL,
    seller_vat    TEXT,
    seller_siret  TEXT,

    buyer_name    TEXT NOT NULL,
    buyer_vat     TEXT,
    buyer_siret   TEXT,

    subtotal      NUMERIC(18, 4) NOT NULL,
    tax_total     NUMERIC(18, 4) NOT NULL,
    total         NUMERIC(18, 4) NOT NULL,

    status        TEXT NOT NULL DEFAULT 'draft',
    payload       JSONB NOT NULL,

    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT invoices_status_check CHECK (
        status IN (
            'draft',
            'issued',
            'sent',
            'received',
            'accepted',
            'rejected',
            'paid',
            'cancelled'
        )
    )
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_invoices_number ON invoices (number);
CREATE INDEX IF NOT EXISTS idx_invoices_issue_date ON invoices (issue_date);
CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices (status);
CREATE INDEX IF NOT EXISTS idx_invoices_buyer_vat ON invoices (buyer_vat);

-- --------------------------------------------------------------------------
-- Table invoice_events : journal d'événements (audit trail)
-- --------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS invoice_events (
    id          BIGSERIAL PRIMARY KEY,
    invoice_id  UUID NOT NULL REFERENCES invoices (id) ON DELETE CASCADE,
    event_type  TEXT NOT NULL,
    payload     JSONB,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_invoice_events_invoice_id ON invoice_events (invoice_id);
CREATE INDEX IF NOT EXISTS idx_invoice_events_created_at ON invoice_events (created_at DESC);
CREATE INDEX IF NOT EXISTS idx_invoice_events_event_type ON invoice_events (event_type);

-- --------------------------------------------------------------------------
-- Trigger: maintient invoices.updated_at
-- --------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_invoices_updated_at ON invoices;
CREATE TRIGGER trg_invoices_updated_at
    BEFORE UPDATE ON invoices
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();
