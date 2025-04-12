# **Contributing**

By contributing, you agree to abide by our
[code of conduct](https://github.com/caffeine-addictt/audiopatch?tab=coc-ov-file#readme).

## Prerequisites

- [Rust][Rust install]
- [GNU/Make][]

## Building

You can build with [GNU/Make][],
simply run the following commands:

### Rust

```sh
make build
```

### Docker

```sh
make up
```

## Testing

Running the following will run all tests.

```sh
make test
```

## Creating commits

Commit messages should conform to [Conventional commits][].

We also ask that your code is formatted before committing
by running:

```sh
make lint
make fmt
```

## Submitting a Pull Request

Push your changes to your fork and create a Pull Request
against the main branch. Please include a clear and concise description
of your changes and why they are necessary. Also ensure that your Pull Request
title conforms to [Conventional commits][] and that you have incremented version
numbers according to [SemVer][] by running:

```sh
make bump version=x.x.x
```

## Creating an issue

Please ensure that there is no similar issue already open before
creating a new one.

If not, you can choose a relevant issue template from the [list](https://github.com/caffeine-addictt/waku/issues/new/choose).
Providing as much information as possible will make it easier for us to help
resolve your issue.

[Rust install]: https://www.rust-lang.org/tools/install
[GNU/Make]: https://www.gnu.org/software/make/#download
[Conventional commits]: https://www.conventionalcommits.org
[SemVer]: https://semver.org
