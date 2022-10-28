# openapi-yup-generator

**This package is under heavy development and will change some specifications.**

CLI tool for generating yup definitions from openapi3.yaml

# Installation


```sh
curl -sfL https://raw.githubusercontent.com/igtm/openapi-yup-generator/master/install.sh | sudo sh -s -- -b=/usr/local/bin
```

You can also download old version binary from [release](https://github.com/igtm/openapi-yup-generator/releases).

# Usage

```
Usage: openapi-yup-generator [OPTIONS]

Options:
  -f, --file <FILE>      openapi3 yaml or json file name
  -o, --out <OUT>        output file name
  -c, --config <CONFIG>  config file name
  -h, --help             Print help information
  -V, --version          Print version information
```

# Settings

place `openapi-yup-generator-config.jsonc` on your working directory

```jsonc
{
  // "description_as_label": false,
  // "file": "openapi3.yaml",
  // "out": "yup-defs.js",
}
```
