# Auto-Pilot-Computer

This tool utilizes GPT4 Vision's capabilities to function as a virtual operator for your computer. Simply specify your desired task, and it will try to accomplish it on your behalf.

## Table of Contents

- [Demo Videos](#demo)
- [Installation](#installation)
- [Usage](#usage)
- [How it works](#how-it-works)
- [Contributing](#contributing)

## Demo

https://github.com/mostafasadeghi97/auto-pilot-computer/assets/41698808/b9fe7e69-7942-4eb6-838e-b790693057ed

https://github.com/mostafasadeghi97/auto-pilot-computer/assets/41698808/2284bb7d-81a7-4f30-a7b7-262f12439d65

## Installation

If you are using rust and cargo, you can install this project by running:

```bash
cargo install --path .
```

It is also available on crates.io, so you can install it with:

```bash
cargo install auto-pilot
```

If you want to use prebuilt binaries, you can download them from the [releases](https://github.com/mostafasadeghi97/auto-pilot-computer/releases) page.

you can build this project by running:

```bash
cargo build --release
```

## Usage

Make sure that your terminal has access to run operations on your computer. In MacOS, you need to enable terminal access in System Settings > Privacy & Security > Accessibility.

Export your openai api key as an environment variable:

```bash
export OPENAI_API_KEY=<your api key>
```

After installing the program, you can run the cli by running:

```bash
auto-pilot
```

Optional Arguments:

```bash
Usage: auto-pilot [OPTIONS]

Options:
  -o, --objective <OBJECTIVE>
          The objective you want to achieve with the computer

  -g, --grid-interval <GRID_INTERVAL>
          The grid interval to use when capturing the screen. Default is 300. The smaller the number, more number of lines will be drawn. (closer to pixel level)

          [default: 300]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## How it works

When the program starts, it prompts you to input an objective. Together with a screen capture featuring a guidance grid, this objective is sent to the GPT-4 Vision model. The model then selects the most effective action to fulfill the specified goal, and the program carries out this action.
The actions are:

```
- CLICK <X> <Y> (estimates the coordinates of the mouse cursor as a percentage of the screen size and clicks on that point)
- TYPE <TEXT> (types the specified text)
- SEARCH <TEXT> (searches the specified app in the computer and opens it)
- DONE (if the objective is achieved)
```

The program will keep executing the actions until the objective is achieved or reaches a maximum number of iterations (which is 15 by default).

Here is a screenshot of the screen, which includes a grid overlay:

![screenshot_with_grid](https://github.com/mostafasadeghi97/auto-pilot-computer/assets/41698808/bc028fc5-e375-4585-b361-96490a2d66a7)

Here is a sample output of the program:

```bash
~ auto-pilot -o "write a short story in a blank document in google docs"
[Auto-Pilot-Computer] [Act] SEARCH COMPLETE Open program: Google Chrome
[Auto-Pilot-Computer] [Act] CLICK COMPLETE Click: x: 50%, y: 9%, description: "Click: Address Bar", reason: "To type in the URL for Google Docs and start a new document for the short story"
[Auto-Pilot-Computer] [Act] TYPE COMPLETE Type: docs.new
[Auto-Pilot-Computer] [Act] TYPE COMPLETE Type: Once upon a time in a tranquil valley hidden among the sprawling mountains, there lived an unusual creature known as Aloysius. Unlike the other denizens of the valley, Aloysius was not a fearsome dragon nor a majestic griffin. In fact, Aloysius was a small, unassuming pangolin with scales that shimmered like opals under the moonlight...
[Auto-Pilot-Computer] [Act] TYPE COMPLETE Type:  His home was a cosy burrow lined with soft leaves and the rarest of feathers, which he had collected on his numerous adventures across the various terrains of the valley. Despite his diminutive stature, Aloysius was a pangolin of great curiosity and even greater courage. One day, Aloysius set out earlier than usual, determined to explore a mysterious cave that had long been the subject of many a valley whisper. The cave was said to hold ancient secrets and untold treasures, guarded by enigmas as old as the mountains themselves. With his trusty satchel slung over his shoulder and his spirit alight with excitement, Aloysius began his ascent towards the fabled cavern...
```

## Contributing

Feel free to contribute to this project by opening a pull request or issue.
