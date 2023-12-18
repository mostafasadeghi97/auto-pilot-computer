# Auto-Pilot-Computer

This is a tool that uses GPT4 Vision to operate your computer.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Installation

If you are using rust and cargo, you can build this project by running:

```bash
cargo build --release
```

It is also available on crates.io, so you can install it with:

```bash
cargo install auto-pilot
```

If you want to use prebuilt binaries, you can download them from the [releases](https://github.com/mostafasadeghi97/auto-pilot-computer/releases)
page.

## Usage

export your openai api key as an environment variable:

```bash
export OPENAI_API_KEY=<your api key>
```

After installing the program, you can run the cli by running:

```bash
auto-pilot
```

Optional Arguments:

```bash
auto-pilot -o <objective> -g <grid_interval>
```

## Contributing

Feel free to contribute to this project by opening a pull request or issue.
