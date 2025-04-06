-- Your SQL goes here
create table transactions  (
      id INTEGER NOT NULL PRIMARY KEY ,
      amount INTEGER NOT NULL,
      description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      budget_id INTEGER NOT NULL REFERENCES budget(id)
)