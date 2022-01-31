# czen

* Simple CLI tool to interactively create conventional commits [specification][1] for git commits.
* Instead of running `git commit ...`, run czen and answer the questions presented to create the commit.
* Make sure the git command is installed on your system before running.

## Installation

You can download prebuilt binaries for macOS, Linux and Windows [here][2].

If you're a [Homebrew][3] user, you can install it using homebrew taps.

```shell
brew tap manojkarthick/tap
brew install czen
```

## Troubleshooting

On macOS, if you encounter the error `"czen" cannot be opened because the developer cannot be verified`, run the following:

```shell
xattr -d com.apple.quarantine ./amalgam
```

## Usage

[![asciicast](https://asciinema.org/a/Ib0dG5jrecHyz04T2hJnK9hSJ.svg)](https://asciinema.org/a/Ib0dG5jrecHyz04T2hJnK9hSJ)

[1]: https://www.conventionalcommits.org/en/v1.0.0-beta.2/
[2]: https://github.com/manojkarthick/czen/releases/latest
[3]: https://brew.sh/
