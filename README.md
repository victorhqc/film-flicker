# Film Flicker

<img src="./img/logo.png" height="150" />

## About

When scanning film negatives, the scan metadata does not match the original
exposure settings. This tool fixes that by replacing the EXIF metadata of the
scans with the correct information.

## How I Use It

When I shoot film, I take a photo with my phone of my camera's settings (shutter
speed and aperture) after each shot. At home, I record this information in a
spreadsheet along with the shot time, film ISO, camera, lens, and other details.

Using the spreadsheet as a CSV file, you can replace the scan values with the
correct exposure data.

## How to Use

_Note: This tool is under development._

Build the tool locally. Check the Development section for instructions.

```shell
cargo run -- from-csv -s "<PATH_FOR_IMAGES>" -m "<PATH_FOR_CSV>.csv" -c <CAMERA_MODEL> --maker <MAKER> -f <FILM_NAME>

# Example
cargo run -- from-csv -s "E:\Photos\Film Scans\2024\06-June" -m "E:\Photos\Film Scans\2024\06-June\metadata.csv" -c Hexar_RF --maker KONICA -f "Kodak Gold 200"
```

**Important:** The number of CSV rows must match the number of photos in the
directory. Images must be sortable by name (e.g., "1.jpg, 2.jpg..." or
"DSCF2470.RAF, DSCF2471.RAF...") so CSV rows match images chronologically.

The CSV format:

| no  | camera_maker | camera_name | lens_maker  | lens_name               | focal_length | date                      | iso | aperture | shutter_speed | exposure_compensation |
| --- | ------------ | ----------- | ----------- | ----------------------- | ------------ | ------------------------- | --- | -------- | ------------- | --------------------- |
| 1   | LEICA        | M6          | VOIGTLANDER | NOKTON 35mm F1.5        | 35           | 2024:06:15 15:39:00+02:00 | 200 | 2.8      | 1/60          | 0.67                  |
| 2   | LEICA        | M6          | LEICA       | APO SUMMICRON 50mm F2.0 | 50           | 2024:06:15 15:52:00+02:00 | 200 | 4.0      | 1/60          | 0                     |

An example file is in the `fixtures/` directory.

### Values in metadata.csv

- **no:** Optional. For tracking shots.
- **camera_maker:** String. Camera maker.
- **camera_name:** String. Camera name.
- **lens_maker:** String. Lens maker.
- **lens_name:** String. Lens name.
- **focal_length:** Integer. Lens focal length.
- **date:** String. Format: `YYYY:MM:DD HH:MM:SS+TZ`. Refer to exiftool documentation for alternatives.
- **iso:** Integer. Film ISO (adjust for exposure compensation).
- **aperture:** Float. Aperture value.
- **shutter_speed:** String. Format: `1/60` or `2` (for 2 seconds).
- **exposure_compensation:** String/Float. Optional. Formats:
  - Positive: `0.33`, `0.67`, `1.33`
  - Negative: `-0.33`, `-0.67`, `-1.33`
  - Fractions: `1/3`, `2/3`, `1 1/3`, `-1/3`, `-2/3`, `-1 1/3`

## Development

### Requirements

- Rust >= 1.79
- Git >= 2.27

### Installation

This project requires [`exiftool`](https://exiftool.org/). Run the following script to download it locally.

**Unix Systems**

```bash
./scripts/unix/install.sh
```

**Windows**

```bat
.\scripts\windows\install.bat
```

### How to Run

After installing [`exiftool`](https://exiftool.org/), run the CLI with cargo:

```shell
cargo run -- from-csv -s "<PATH_FOR_IMAGES>" -m "<PATH_FOR_CSV>.csv" -c <CAMERA_MODEL> --maker <MAKER> -f <FILM_NAME>

# Example
cargo run -- from-csv -s "E:\Photos\Film Scans\2024\06-June" -m "E:\Photos\Film Scans\2024\06-June\metadata.csv" -c "Hexar RF" --maker KONICA -f "Kodak Gold 200"
```

## Build

### Windows

Windows can build and install as a regular program.

1. Download the WiX v3 toolchain from [here](https://github.com/wixtoolset/wix3/releases/tag/wix3141rtm):

   ```bat
   .\wix314.exe /install /quiet /norestart
   ```

2. Install cargo-wix:

   ```bat
   cargo install cargo-wix
   ```

3. Restart the terminal and build:

   ```bat
   cargo wix --install

   # Or to debug errors
   cargo wix --install --nocapture
   ```
