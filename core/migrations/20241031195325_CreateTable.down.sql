-- Add down migration script here
DROP INDEX mastodon_app_url_idx;
DROP TABLE mastodon_app;
