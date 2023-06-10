# HN jobs parser

Installation:

```sh
$ wget https://github.com/iliabylich/hn-parser/releases/download/latest/hnparser_0.1.0_amd64.deb
$ dpkg -i hnparser_0.1.0_amd64.deb
# configure `/etc/hnparser.json`
$ systemctl enable hnparser
$ systemctl start hnparser
```

To uninstall:

```sh
$ dpkg --purge hnparser
```

To view logs:

```sh
$ journalctl -u hnparser
```
