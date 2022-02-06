# mknft

Toolset to create NFTs from Photoshop layer group combinations.

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
- Size of output collection cannot be too close to total number of possible combinations

// todo

## Limitations

- Photoshop layers **must** not have hidden pixels outside canvas

// todo

## Usage

// todo

## Installation

// todo

## API

// todo

## Example Project

Included in this package is an [example project](example) that is composed of a configuration JSON file and PSD with three layer groups.

Run the `prepare` command and subsequently the `package` command to observe how `mknft` functions.

## Real-World Example

`mknft` was used to build the [nfshibes NFT project](https://github.com/nfshibes/nfshibes.github.io).

[GitHub Pages website](http://nfshibes.com) hosts NFTs minted with `mknft`.

## License

[MIT](LICENSE)
