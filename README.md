# qr-paste

CLI tool that decodes QR codes from your clipboard or an image file and prints the content to stdout.

## Usage

Copy any image containing a QR code to your clipboard, then run:

```
qr-paste
```

Or pass an image file directly:

```
qr-paste path/to/image.png
```

Multiple QR codes in a single image are each printed on their own line. Exits with code 1 if no QR codes are found.

## Install

```
cargo install --path .
```

## Supported formats

PNG, JPEG, WebP, TIFF, BMP, GIF — and stylized/designer QR codes with rounded or circular dots.
