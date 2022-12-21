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
* setup postgresql support
