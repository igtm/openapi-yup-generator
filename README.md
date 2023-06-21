# openapi-yup-generator

CLI tool for generating [yup](https://github.com/jquense/yup) definitions from openapi3.yaml

Supports Openapi v3.0.x ([depends on openapiv3 library](https://github.com/glademiller/openapiv3#openapi-v3-))

# Installation

### curl

```sh
sudo curl -sfL https://raw.githubusercontent.com/igtm/openapi-yup-generator/master/install.sh | sudo sh -s -- -b=/usr/local/bin
```

if you want to download old version, pass `-v` argument.

```sh
sudo curl -sfL https://raw.githubusercontent.com/igtm/openapi-yup-generator/master/install.sh | sudo sh -s -- -b=/usr/local/bin -v=v0.0.7
```

### brew

```sh
brew install igtm/tap/openapi-yup-generator
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
  // "case": "snake" // or "camel"
}
```

# Note

currently not all yup definitions are supported. feel free to send PR or issue :)
