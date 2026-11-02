# HN jobs parser

Installation:

1. Download the `hnparser` binary from the latest release into `/usr/bin/hnparser`
2. Download `hnparser.service` from the same release into `/lib/systemd/system/hnparser.service`
3. Configure `/etc/hnparser.toml` (see `config.example.toml`)
4. Enable and start the service:

```sh
$ systemctl enable hnparser
$ systemctl start hnparser
```

To view logs:

```sh
$ journalctl -u hnparser
```
