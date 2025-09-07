-- 0001_init.sql  — minimal, reorg-aware schema for CoW Settlement indexing

-- Journal and durability
PRAGMA journal_mode = WAL;        -- concurrent reads during writes
PRAGMA synchronous = NORMAL;      -- good tradeoff for ingest
PRAGMA foreign_keys = ON;
PRAGMA temp_store = MEMORY;
PRAGMA user_version = 1;

-- Canonical blocks you’ve seen (optional but handy for debugging and FK later)
CREATE TABLE blocks (
  number     INTEGER PRIMARY KEY,
  hash       BLOB NOT NULL UNIQUE,   -- 32 bytes
  parent     BLOB NOT NULL,          -- 32 bytes
  ts         INTEGER NOT NULL        -- unix seconds
);

-- Events.  Reorg deletes target block_hash; idempotence via (tx_hash, log_index).
-- Amount-like fields are TEXT (decimal strings) to preserve 256-bit EVM values.

CREATE TABLE cow_trades (
  tx_hash     BLOB NOT NULL,         -- 32 bytes
  log_index   INTEGER NOT NULL,
  block_hash  BLOB NOT NULL,         -- 32 bytes
  order_uid   BLOB NOT NULL,         -- bytes
  owner       BLOB NOT NULL,         -- 20 bytes
  sell_token  BLOB NOT NULL,         -- 20 bytes
  buy_token   BLOB NOT NULL,         -- 20 bytes
  exec_sell   TEXT NOT NULL,         -- uint256 as decimal string
  exec_buy    TEXT NOT NULL,         -- uint256 as decimal string
  fee         TEXT NOT NULL,         -- uint256 as decimal string
  PRIMARY KEY (tx_hash, log_index)
);
CREATE INDEX idx_trades_blockhash  ON cow_trades(block_hash);
CREATE INDEX idx_trades_order_uid  ON cow_trades(order_uid);

CREATE TABLE cow_settlements (
  tx_hash        BLOB PRIMARY KEY,   -- one per tx
  block_hash     BLOB NOT NULL,
  solver         BLOB NOT NULL,      -- 20 bytes
  clearing_prices BLOB               -- opaque bytes
);
CREATE INDEX idx_settle_blockhash ON cow_settlements(block_hash);
CREATE INDEX idx_settle_solver    ON cow_settlements(solver);

CREATE TABLE cow_interactions (
  tx_hash     BLOB NOT NULL,
  log_index   INTEGER NOT NULL,
  block_hash  BLOB NOT NULL,
  target      BLOB NOT NULL,         -- 20 bytes
  calldata    BLOB NOT NULL,
  PRIMARY KEY (tx_hash, log_index)
);
CREATE INDEX idx_interact_blockhash ON cow_interactions(block_hash);

CREATE TABLE cow_presignatures (
  tx_hash     BLOB NOT NULL,
  log_index   INTEGER NOT NULL,
  block_hash  BLOB NOT NULL,
  order_uid   BLOB NOT NULL,
  signed      INTEGER NOT NULL CHECK (signed IN (0,1)),
  PRIMARY KEY (tx_hash, log_index)
);
CREATE INDEX idx_presig_blockhash ON cow_presignatures(block_hash);
CREATE INDEX idx_presig_order_uid ON cow_presignatures(order_uid);

CREATE TABLE cow_invalidations (
  tx_hash     BLOB NOT NULL,
  log_index   INTEGER NOT NULL,
  block_hash  BLOB NOT NULL,
  order_uid   BLOB NOT NULL,
  PRIMARY KEY (tx_hash, log_index)
);
CREATE INDEX idx_inval_blockhash  ON cow_invalidations(block_hash);
CREATE INDEX idx_inval_order_uid  ON cow_invalidations(order_uid);

-- Lightweight cursor state for your pipeline
CREATE TABLE meta (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
INSERT OR IGNORE INTO meta(key,value) VALUES
  ('last_seen_number','0'),
  ('last_seen_hash',''),
  ('last_finalized_number','0');