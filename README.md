# Principal Vanity Generator

A high-performance Rust tool for generating Internet Computer Protocol (ICP) vanity principals with custom prefixes. This tool allows you to generate ICP principals that start with your desired characters.

## What is a Vanity Principal?

A vanity principal is an ICP principal that starts with specific characters of your choice. For example, instead of a random principal like `2vxsx-fae`, you might want one that starts with `cool` like `cool-abc-def`.

## Features

- **Multi-threaded Generation**: Utilizes Rayon for parallel processing across all CPU cores
- **BIP39/BIP32 Compliant**: Follows standard cryptographic practices for key derivation
- **ICP-Specific**: Uses the correct derivation path for ICP addresses (m/44'/223'/0'/0/0) - compatible with Plug Wallet
- **Real-time Progress**: Shows generation rate and progress updates
- **Memory Efficient**: Batch processing to optimize memory usage

## Prerequisites

- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning the repository

### Linux Dependencies

If you're on Linux (Ubuntu/Debian), install required packages first:

```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev
```

For other Linux distributions:
- **Fedora/RHEL**: `sudo dnf install gcc pkg-config openssl-devel`
- **Arch**: `sudo pacman -S base-devel pkg-config openssl`

## Installation

1. **Clone the repository**:
```bash
git clone https://github.com/mariode0/Principal-Vanity.git
cd Principal-Vanity
```

2. **Build the project**:
```bash
cargo build --release
```

## How to Use

### Step 1: Configure Your Target Prefix

Open `src/main.rs` and modify the target prefix:

```rust
let target_prefix = "aaaaa"; // Change this to your desired prefix
```

**Important Notes:**
- Use only Base32 characters: lowercase letters (a-z) and numbers (2-7)
- Avoid characters 0, 1, 8, 9 as they are not valid in Base32 encoding
- Avoid uppercase letters as ICP principals are case-sensitive
- Shorter prefixes are much faster to find
- Each additional character increases difficulty exponentially

### Step 2: Run the Generator

```bash
cargo run --release
```

### Step 3: Wait for Results

The tool will:
- Show estimated attempts needed
- Display real-time progress updates
- Print the found address, mnemonic, and generation rate when complete

## Understanding the Output

When a match is found, you'll see:
```
MATCH FOUND after 1234567 iterations!
Time elapsed: 45.23s
Principal : aaaaa-bcdef-ghij-klmn-opqr-stuv-wxyz-2345-6789-abcd-efgh
Mnemonic  : word1 word2 word3 word4 word5 word6 word7 word8 word9 word10 word11 word12
Rate: 27300 attempts/second
```

## Performance Guide

### Expected Generation Times

| Prefix Length | Estimated Attempts | Time (8-core CPU) |
|---------------|-------------------|-------------------|
| 3 characters  | ~32,768           | < 1 second       |
| 4 characters  | ~1,048,576        | ~30 seconds      |
| 5 characters  | ~33,554,432       | ~15 minutes      |
| 6 characters  | ~1,073,741,824    | ~8 hours         |

### Optimization Tips

1. **Use Release Build**: Always use `cargo run --release` for maximum performance
2. **Shorter Prefixes**: Start with 3-4 characters for faster results
3. **CPU Cores**: More CPU cores = faster generation
4. **Background Running**: You can run this in the background while doing other tasks

## Security Considerations

⚠️ **IMPORTANT SECURITY NOTES:**

1. **Keep Mnemonics Safe**: The generated mnemonic phrase is your private key
2. **Never Share**: Never share your mnemonic or private keys
3. **Secure Storage**: Store mnemonics in a secure password manager or hardware wallet
4. **Test First**: Always test with small amounts before using for large transactions

## How It Works

1. **Entropy Generation**: Creates cryptographically secure random entropy
2. **Mnemonic Creation**: Converts entropy to BIP39 mnemonic phrase
3. **Seed Derivation**: Generates seed from mnemonic
4. **Key Derivation**: Uses BIP32 to derive private key with ICP path `m/44'/223'/0'/0/0`
5. **Address Generation**: Creates ICP identity and extracts principal
6. **Pattern Matching**: Checks if address matches desired prefix

### Why This Derivation Path?

The derivation path `m/44'/223'/0'/0/0` follows the BIP44 standard:
- `m/44'` - BIP44 standard for deterministic wallets
- `223'` - ICP coin type (223 is the registered coin type for Internet Computer)
- `0'` - Account number (0 for first account)
- `0` - Change address (0 for receiving addresses)
- `0` - Address index (0 for first address)

This path ensures compatibility with Plug Wallet and other ICP wallets that follow the BIP44 standard.

## Technical Details

- **Cryptographic Standards**: BIP39, BIP32, BIP44
- **Derivation Path**: `m/44'/223'/0'/0/0` (ICP standard, compatible with Plug Wallet)
- **Address Format**: Base32 encoded with CRC32 checksum
- **Threading**: Rayon parallel iterator for multi-core utilization

## Troubleshooting

### Common Issues

1. **Build Errors**: Make sure you have Rust 1.70+ installed
2. **Linux Build Errors**: Install required dependencies:
   ```bash
   sudo apt install -y build-essential pkg-config libssl-dev
   ```
3. **Slow Performance**: Use `--release` flag for optimized builds
4. **Memory Issues**: The tool is memory-efficient, but very long prefixes may take significant time

### Getting Help

- Check the [Issues](https://github.com/mariode0/Principal-Vanity/issues) page
- Create a new issue if you encounter bugs

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This tool is for educational and legitimate use only. Users are responsible for:
- Securing their generated keys
- Complying with local regulations
- Using the tool responsibly

## Acknowledgments

- Built with [Rust](https://rust-lang.org/)
- Uses [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) for mnemonic generation
- Implements [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) for key derivation
- Parallel processing with [Rayon](https://github.com/rayon-rs/rayon)
