CREATE TABLE ads
(
    id                SERIAL PRIMARY KEY,
    url               text NOT NULL,
    num_prepaid_shows INT  NOT NULL,
    categories        text[] NOT NULL DEFAULT '{}'
);