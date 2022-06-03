# søstr
A private nostr relay

# Why
A private relay is a great way to make sure that your notes are always available.

# How
The relay will only save notes from a particular privatekey, and ignore all others,
but let anyone request the notes that it has saved.

The software will accept settings from a file or environmental variables, and be connected to a postgresql database.

# TODO

* filter on e and p tags

# Links

#### nostr

[protocol](https://github.com/nostr-protocol/nips/blob/master/01.md)\
[example](https://github.com/fiatjaf/relayer)\
[lib](https://github.com/fiatjaf/go-nostr)

#### orm

[documentation](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration)
