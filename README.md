# Film Flicker

<img src="./img/logo.png" height="150" />

## About

When scanning film negatives, the metadata of the scans does not match with
the settings of the exposure of the film. This tool fixes that by replacing the
EXIF metadata of the scans with the correct information.

## How I use it

Whenever I go out to shoot film, as soon as I shot any picture with my camera,
I take an additional photo with my phone towards my film camera so the settings
are visible, i.e. Shutter Speed & Aperture. Then, at home after the shooting
is done. I write the  information in a spreadsheet, making sure I write down
the time of the shot and other information like ISO of the film, camera, lens,
etc.

Using the spreadsheet as a CSV File, it is possible to replace the values from
the scan with the values in the CSV File.

## How to use

_Note: This is a tool under development_

For now, this tool requires to be built locally. Check the development section
for more information on how to do it. Once it's built then continue on how to
use.

```shell

cargo run -- -s "<PATH_FOR_IMAGES>" -m "<PATH FOR CSV>.csv" -c <CAMERA_MODEL --maker <MAKER> -f <FILM_NAME>

# Example
cargo run -- -s "E:\Photos\Film Scans\2024\06-June" -m "E:\Photos\Film Scans\2024\06-June\metadata.csv" -c Hexar_RF --maker KONICA -f "Kodak Gold 200"
```

An important note: The number of rows of the CSV must match the number of photos
in the given file. Additionally, the images must be in a way that is sortable
by name, like: "1.jpg, 2.jpg ..." or "DSCF2470.RAF, DSCF2471.RAF ..." this way,
the CSV rows will match the images chronologically.

The CSV must be as follows

| no | lens_name              | focal_length | date                      | iso | aperture | shutter_speed | exposure_compensation |
|----|------------------------|--------------|---------------------------|-----|----------|---------------|-----------------------|
| 1  | 7Artisans 35mm f/2 MII | 35           | 2024:06:15 15:39:00+02:00 | 200 | 2.8      | 1/60          | 0.67                  |

An example file can be found under the `fixtures/` path.

### Values in metadata.csv

- **no:** This is not required, I like to use it for an easy management in my shots.
- **lens_name:** String, arbitrary name of your lens.
- **focal_length:** Integer, the focal length of the lens.
- **date:** String, note the format, the yyy-mm-dd might be valid, but I'm not sure, please refer to exiftool documentation.
- **iso:** Integer, ISO of the film (change this if you under or over exposed the film)
- **aperture:** Float, aperture when the shot was made.
- **shutter_speed:** String, use the regular format of 1/60 or 2 (for 2 seconds).
- **exposure_compensation:** String/Float, this is an optional value, it can be left as blank or using any of the following formats:
  - Positive float numbers: 0.33, 0.67, 1.33, etc.
  - Negative float numbers: -0.33, -0.67, -1.33, etc.
  - Positive Fractions: 1/3, 2/3, 1 1/3, etc.
  - Negative fraction numbers -1/3, -2/3, -1 1/3, etc.

## Development

### Requirements

- Rust >= 1.79
- Git >= 2.27

### Installation

This project requires [`exiftool`](https://exiftool.org/), please run the
following script to download it locally.

**Unix Systems**

```bash
./scripts/unix/install.sh
```

**For Windows**

```bat
.\scripts\windows\install.bat
```

### How To Run

Once [`exiftool`](https://exiftool.org/) is installed with the previous script,
run the CLI with cargo.

```shell

cargo run -- -s "<PATH_FOR_IMAGES>" -m "<PATH FOR CSV>.csv" -c <CAMERA_MODEL --maker <MAKER> -f <FILM_NAME>

# Example
cargo run -- -s "E:\Photos\Film Scans\2024\06-June" -m "E:\Photos\Film Scans\2024\06-June\metadata.csv" -c "Hexar RF" --maker KONICA -f "Kodak Gold 200"
```

## Build

### Windows

Windows is able to build and install as a regular program. No need to run it
in development mode.

1. Download the wix v3 toolchain from [here](https://github.com/wixtoolset/wix3/releases/tag/wix3141rtm)

   ```bat
   .\wix314.exe /install /quiet /norestart
   ```

2. Install cargo-wix for building a `.msi`

   ```bat
   cargo install cargo-wix
   ```

3. Re-start the terminal and then you can use the cargo wix command

   ```bat
   cargo wix --install
   
   # Or to debug for errors
   cargo wix --install --nocapture
   ```