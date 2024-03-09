# CLI

The password generator can be also accessed via the command-line interface (CLI) for general-purpose usage or when you
don't have access to your graphical desktop environment. To view CLI command and options details use `keywich --help`.

> Current version is limited with password generator only. It does not have access to your profile database.

```shell
Usage: keywich generate [OPTIONS] --domain <DOMAIN> --charset <CHARSET> --username <USERNAME>
 --target-length <TARGET_LENGTH>

Options:
  -d, --domain <DOMAIN>                Domain for password
  -c, --charset <CHARSET>              Password character set
  -u, --username <USERNAME>            Username for password
  -t, --target-length <TARGET_LENGTH>  Password target length
  -p, --password <PASSWORD>            Password
  -o, --output-type <OUTPUT_TYPE>      Password output type [default: text] [possible values:
 phc, text, base64, json, qr]
      --revision <REVISION>            Seed number [default: 0]
  -h, --help                           Print help
```

Examples:

```shell
keywich generate -d myserver -u admin -t 6 -c A..Za..z0..9 -p pass12345 -o base64
# a3J5Tmlh

keywich generate -d myserver -u admin -t 6 -c A..Za..z0..9 -p pass12345 -o text
# kryNia

keywich generate -d myserver -u admin -t 6 -c A..Za..z0..9 -p pass12345 -o phc
# $kw_scrypt$v=v1$kryNia
```