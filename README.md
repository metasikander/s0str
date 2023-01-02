# søstr
A private nostr relay

# Why
A private relay is a great way to make sure that your notes are always available.

# How
The relay will only save notes from a specified privatekey, and ignore all others,
but let anyone request the notes that it has saved.

The software will accept settings from a file or environmental variables, and be connected to a postgresql database (support pending).

# TODO

* filter on e and p tags
* error response message when non pubkey user tries to post

## Nice to have
* [Friend-of-a-Friend support](https://github.com/metasikander/s0str/issues/3) Save notes from more pubkeys, based on connection to main pubkey.

# Config

Configuration is read from environment and `.env` file.

* `debug_level`: `ERROR`, `WARN`, `INFO`(default), `DEBUG`, `TRACE`
* `pg_host`: Postgres endpoint
* `pg_pass`: Postgres password
* `pg_user`: Postgres user
* `pubkey`: The pubkey of the nostr user that can post (required)
* `ws_ip`: IP interface to host ws on (default: 0.0.0.0)
* `ws_port`: Port to host ws on (default: 8080)

Note that non persistent in memory sqlite will be used unless postgres configurations is set.
