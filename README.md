# mknft

Toolset to create NFTs from Photoshop layer group combinations.

`mknft` was created rather quickly to POC a NFT project. Originally it included tools to deploy NFTs to the OpenSea marketplace but policy changes in their service now prevent that solution from being viable. Features were subsequently removed from `mknft` in order to provide a minimum toolset for generating NFT, images, using Photoshop layer group combinations. There is the possibility of this package including a deployment mechanism to the Polygon network in the future, but that's far from a guarantee.

## Features

- Attributes are organized by PSD layer groups
- Attribute values comprised of layers in each group
- Combinations of attribute values, random layers from groups, for unique NFTs
- Weighted selection of layers using configuration file
- Exclusion lists prevent incompatible layers from being combined
- PSD groups or layers can be ignored with *_* prefix
- Image probabilities calculated for odds of assembling attributes together (factors weights)
- Sort composite images by probability

**Status:** Alpha Experimentation

## Requirements

- Photoshop PSD (layers in groups)
- Rust / Cargo

## Limitations

- Photoshop layers **must** not have hidden pixels outside canvas
- Size of output collection cannot be too close to total number of possible combinations (due to random selection)

## Installation

1. [Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Install `mknft` using Cargo command: `cargo install mknft`

## Usage

`mknft` is a command line interface utility.

### Project States

NFT projects have been broken down into three distinct states.

Each state created by `mknft` produces a new directory with respective configuration and assets. These directories are NOT meant to be the same path for other states. They must be unique to prevent file name collisions.

#### Photoshop Project

This state is not created by `mknft`.

The project directory should contain a structured Photoshop project file (PSD) with a configuration file identifying layers and weights.

#### Prepared Project

This state represents a configuration of the prepared layer images without each individual generated NFT. The preapred project can be used to generate new combinations of NFTs more than once using the same parameters but, due to probablistic outcomes, *should* produce different results each execution.

#### Packaged Project

The final state for making NFTs. The results of this state include a configuration file identifying attributes and name of each NFT. This configuration file can be used to publish NFTs.

This state includes ALL the final NFT files and will consume disk space respectively.

### Command Help

Help menu can be displayed using `--help` or `-h` option with any `mknft` command.

```bash
$ mknft --help
```

OR

```bash
$ mknft prepare --help
```

## Example Project

Included in this package is an [example project](example) that is composed of a configuration JSON file and PSD with three layer groups.

Run the `prepare` command and subsequently the `package` command to observe how `mknft` functions.

## Real-World Example

`mknft` was used to build the [nfshibes NFT project](https://github.com/nfshibes/nfshibes.github.io).

[GitHub Pages website](http://nfshibes.com) hosts NFTs minted with `mknft`.

## License

[MIT](LICENSE)
