-- Add up migration script here
CREATE TABLE mastodon_app(
  id INTEGER UNSIGNED NOT NULL PRIMARY KEY,
  mastodon_url TEXT NOT NULL,
  client_id TEXT NOT NULL,
  client_secret TEXT NOT NULL
);

CREATE UNIQUE INDEX mastodon_app_url_idx ON mastodon_app (mastodon_url);
