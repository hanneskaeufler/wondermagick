#!/bin/bash

set -euo pipefail

sandbox="$(mktemp -d -t 'mytmpdir')"

cargo run -- identify /Users/hanneskaeufler/Pictures/372CANON/IMG_9863.JPG

cargo run -- manipulate /Users/hanneskaeufler/Pictures/372CANON/IMG_9863.JPG "$sandbox/text-center-pink-very-transparent-full-quality-resized.jpg" --strip-metadata --watermark-text "Center Pink" --watermark-text-color-rgba 255,47,146,100 --watermark-text-gravity center --quality 100 --resize 3240x2160

cargo run -- manipulate /Users/hanneskaeufler/Pictures/372CANON/IMG_9860.JPG "$sandbox/text-northeast-green-med-quality.jpg" --strip-metadata --watermark-text "Northeast Green" --watermark-text-color-rgba 0,225,0,255 --watermark-text-gravity northeast --quality 50

cargo run -- manipulate /Users/hanneskaeufler/Pictures/372CANON/IMG_9860.JPG "$sandbox/text-south-blue-low-quality.jpg" --strip-metadata --watermark-text "South Blue" --watermark-text-color-rgba 0,31,230,255 --watermark-text-gravity south --quality 10

cargo run -- manipulate /Users/hanneskaeufler/Pictures/372CANON/IMG_9860.JPG "$sandbox/text-southwest-red-semitransparent-original-quality.jpg" --strip-metadata --watermark-text "Southwest Red Semitransparent" --watermark-text-color-rgba 128,0,0,128 --watermark-text-gravity southwest

cargo run -- manipulate /Users/hanneskaeufler/Pictures/372CANON/IMG_9860.JPG "$sandbox/text-southwest-red-alpha-255-original-quality.jpg" --strip-metadata --watermark-text "Southwest Red Alpha 255" --watermark-text-color-rgba 255,0,0,255 --watermark-text-gravity southwest

cargo run -- manipulate /Users/hanneskaeufler/Pictures/372CANON/IMG_9860.JPG "$sandbox/text-southwest-red-alpha-0-original-quality.jpg" --strip-metadata --watermark-text "Southwest Red Alpha 0" --watermark-text-color-rgba 255,0,0,0 --watermark-text-gravity southwest

open "$sandbox"
