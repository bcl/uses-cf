Build using [Rust](https://rustup.rs) by running `cargo build --release`

Create a file with a list of domains to check, 1 per line, no https:// just the domain name.

Run it with `./target/release/uses-cf /path/to/domain-list.txt`

Output looks like this:

```
[OK] - netflix.com does not use CloudFlare
[!!] - news.ycombinator.com uses CloudFlare, change your password.
[OK] - bitbucket.org does not use CloudFlare
[OK] - keybase.io does not use CloudFlare
[  ] - Error checking steampowered.com: The OpenSSL library reported an error: The OpenSSL library reported an error: error:14090086:SSL routines:ssl3_get_server_certificate:certificate verify failed
[OK] - lwn.net does not use CloudFlare
```

It may run slowly if you give it nonexistant domains,
[reqwest](https://docs.rs/reqwest/0.4.0/reqwest/index.html) doesn't appear to
support timeouts yet.

