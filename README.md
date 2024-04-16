# Rodeo 🦌

![rodeo](./rodeo.jpeg)

Rodeo is a simple HTTP reverse proxy server that can be used to bypass [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) and route incoming HTTP requests to different servers based on the request path

- [Description](#description)
- [Getting Started](#getting-started)
- [Installing](#installing)
- [Executing program](#executing-program)
- [License](#license)

## Description

Rodeo is a straightforward yet powerful HTTP reverse proxy server designed to circumvent Cross-Origin Resource Sharing (CORS) restrictions. Its primary function is to intercept incoming HTTP requests and efficiently route them to various servers based on the specific request path. By doing so, Rodeo enables seamless communication between client-side applications and backend servers, regardless of their origin or domain.

## Installation

Rodeo is currently available as a pre-built binary for Linux and macOS. To install Rodeo, simply download the appropriate binary for your operating system and architecture from the [releases page](https://github.com/rodeo/release)

## Executing program

Get started by creating a new configuration file using the following command:

```sh
rodeo init <config-file-type>
# rodeo init json
```
The config-file-type can be either `json` or `toml`. This command will generate a new configuration file in the current directory with the default settings. You can then modify the configuration file to suit your needs.

Once you have created a configuration file, you can start the Rodeo server by running the following command:

```sh
rodeo run -c rodeo.<config-file-type>
# rodeo run -c rodeo.json
```

## License

This project is licensed under the [NAME HERE] License - see the LICENSE.md file for details

```
