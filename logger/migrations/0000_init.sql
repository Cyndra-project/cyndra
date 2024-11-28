CREATE TABLE IF NOT EXISTS logs (
    deployment_id TEXT,        -- The deployment that this log line pertains to.
    cyndra_service_name TEXT, -- The cyndra service which created this log.
    tx_timestamp DATETIME,      -- Unix epoch timestamp.
    data BLOB                  -- Log fields object.
);
