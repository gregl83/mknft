# mknft

Toolset to create NFTs from Photoshop layer group combinations.

- Attributes are organized by PSD layer groups
- Attribute values comprise of layers in each group
- Combinations of attribute values, random layers from groups, for unique NFTs
- Weighted selection of layers using configuration file
- Exclusion lists prevent incompatible layers from being combined
- PSD groups or layers can be ignored with *_* prefix
- Image probabilities calculated for odds of assembling attributes together (factors weights)
- Sort composite images by probability
- Publish composite images to OpenSea as NFTs

**Status:** Beta Development

## Requirements

- Photoshop PSD (layers in groups)
- Size of output collection cannot be too close to total number of possible combinations
- MetaMask Wallet (for publish command)

// todo

## Limitations

- Photoshop layers **must** not have hidden pixels outside canvas
- Publish command only supports OpenSea marketplace using selenium and is prone to crashing

// todo

## Usage

// todo

## Installation

// todo

## API

// todo

## Example Project

Included in this package is an [example project](example) that is comprised of a configuration JSON file and PSD with three layer groups.

Run the `prepare` command and subsequently the `package` command to observe how `mknft` functions.

## Real-World Example

`mknft` was used to build the [nfshibes NFT project](https://github.com/nfshibes/nfshibes.github.io).

[GitHub Pages website](http://nfshibes.com) hosts NFTs minted with `mknft`.

## License

[MIT](LICENSE)
