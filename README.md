# film_exif_fixer
Film Scans EXIF Correction

## Development

### Requirements

- Rust >= 1.79
- Perl >= 5
- Git >= 2.27

### For Windows

Make sure you have [chocolatey](https://chocolatey.org/) and install Perl

```bat
choco install strawberryperl
```

### How To Run

This depends on having [`exiftool`](https://exiftool.org/) installed. The following script will
download exiftool.

**For Unix Systems**

```bash
./scripts/unix/install.sh
```

**For Windows**

```bat
.\scripts\windows\install.bat
```