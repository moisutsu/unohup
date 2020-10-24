<h1 align="center">Welcome to unohup 👋</h1>
<p>
  <img alt="Version" src="https://img.shields.io/badge/version-0.1.0-blue.svg?cacheSeconds=2592000" />
  <a href="https://github.com/moisutsu/unohup/blob/master/LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
  <a href="https://twitter.com/moisutsu" target="_blank">
    <img alt="Twitter: moisutsu" src="https://img.shields.io/twitter/follow/moisutsu.svg?style=social" />
  </a>
</p>

> A useful wrapper library for nohup

## Install

```sh
cargo install --path .
```

## Usage

```sh
unohup <commands>
```

| Option         | description             |
| -------------- | ----------------------- |
| -l, --log-file | Specify a log file.     |
| -n, --nice     | Specify a `nice` value. |

The log is written to `/dev/null` by default, and the default value of `nice` is `0`.

## Author

👤 **moisutsu**

* Twitter: [@moisutsu](https://twitter.com/moisutsu)
* Github: [@moisutsu](https://github.com/moisutsu)

## Show your support

Give a ⭐️ if this project helped you!

## 📝 License

Copyright © 2020 [moisutsu](https://github.com/moisutsu).<br />
This project is [MIT](https://github.com/moisutsu/unohup/blob/master/LICENSE) licensed.

***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_
