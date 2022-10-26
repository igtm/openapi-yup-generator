# openapi-yup-generator

**This package is under heavy development and will change some specifications.**

CLI tool for generating yup definitions from openapi3.yaml

# Install

//TBD

You can download binary from [release](https://github.com/igtm/openapi-yup-generator/releases).

I will setup easy ways to install...

# Usage

```
Usage: openapi-yup-generator [OPTIONS]

Options:
  -f, --file <FILE>      openapi3 yaml file name
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
