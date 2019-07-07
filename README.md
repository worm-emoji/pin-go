# pin-go

pin-go is a tiny Rust app that acts as a URL shortener using
[Pinboard](https://pinboard.in) as a source of truth.

`pin-go` looks for Pinboard items with the tag `go` and a secondary tag that describes the location, in the format `go:linkname`. Thanks to custom DNS mappings, I use it as a personal "golinks" service.

An example Pinboard bookmark looks like this:

![](https://lukemil.es/Files/pin-example.png)

This creates the link: [go/op-1](https://teenage.engineering/guides/op-1).

## How to use

pin-go requires the `PINBOARD` environment variable to be set to a Pinboard API
token. If you don't have a Pinboard token, you can get one at
`https://pinboard.in/settings/password`.

Use Cargo to compile the program:

```
cargo build --release
```

You'll now have a binary located at `./target/release/pin-go`.

Running the binary will start a [Rocket](https://rocket.rs) http service at `http://localhost:8000`. If you want to change the port that the program runs on, check the [Rocket configuration docs](https://rocket.rs/v0.4/guide/configuration/#environment-variables).

### Deployment and setup

To make deploying pin-go easy, we've included a Dockerfile in this repo. Once you have it deployed somewhere, I recommend creating a DNS mapping.

This can be as simple as adding an entry to `go` (or whatever short word you prefer) in your `/etc/hosts`. To let me use my golinks on the go, I've set up a mapping at [NextDNS](https://nextdns.io) and use their iOS app.

### Reloading the configuration

`pin-go` keeps the mapping of golinks in memory in the interest in returning a response as quickly as possible.

If you make changes to your Pinboard golink mappings, simply hit the service at `/reload`. The new configuration will be fetched from Pinboard and replaced in memory.

## License

MIT
