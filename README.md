# søstr
A private nostr relay

# Why
A private relay is a great way to make sure that your notes are always available.

# How
The relay will only save notes from a particular privatekey, and ignore all others,
but let anyone request the notes that it has saved.

The software will accept settings from a file or environmental variables, and be connected to a postgresql database.

# TODO

* data types
* data storage
* pub key config
* WS

# Links

#### nostr

[protocol](https://github.com/nostr-protocol/nips/blob/master/01.md)\
[example](https://github.com/fiatjaf/relayer)\
[lib](https://github.com/fiatjaf/go-nostr)

#### ws

[example](https://stackoverflow.com/questions/65376514/q-rust-how-do-you-use-the-tokio-tungstenite-accept-hdr-async-function-to-get-h)

#### orm

[documentation](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration)
