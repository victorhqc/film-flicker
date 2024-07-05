@echo off
setlocal
  mkdir .\deps
  cd .\deps

  rmdir /s /q exiftool

  echo "Cloning exiftool@12.70"
  git clone --depth 1 --branch 12.70 https://github.com/exiftool/exiftool.git

  echo "Cleaning up exiftool"

  cd ./exiftool
  rmdir /s /q .git
  rmdir /s /q html
  del README
  del Changes
  del windows_exiftool
  del perl-Image-Exiftool.spec
  del MANIFEST
  del Makefile.PL
  del META.yml
  del META.json

endlocal
