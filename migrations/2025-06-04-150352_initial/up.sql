CREATE TABLE equations
(
    id      SERIAL PRIMARY KEY,
    content VARCHAR(16) NOT NULL,
    answer  REAL        NOT NULL
);