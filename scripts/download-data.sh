#!/bin/sh
mkdir -p data
if [ ! -f data/cities15000.txt ]; then
  echo "Downloading cities15000"
  curl -L --progress-bar http://download.geonames.org/export/dump/cities15000.zip > data/cities15000.zip
  (cd data; unzip -q cities15000.zip; rm cities15000.zip)
fi
if [ ! -f data/countryInfo.txt ]; then
  echo "Downloading CountryInfo"
  curl -L --progress-bar http://download.geonames.org/export/dump/countryInfo.txt > data/countryInfo.txt
fi
