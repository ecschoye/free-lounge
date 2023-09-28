# Capture The Flag Challenge: Free Lounge
### Part of the Exercises in IDATT2503 - Security in Programming and Cryptography

![index](https://github.com/ecschoye/free-lounge/tree/main/images/image.png)

## Team Members
- Aleksander Olsvik
- Daniel Skymoen
- Edvard Schøyen
- Jarand Romestrand

## Overview

This Capture The Flag (CTF) challenge is designed to test a beginner to intermediate skill set. Named "Free Lounge," it focuses on the real-world scenario of altering a QR code to gain unauthorized access to the SAS Lounge. Players will need skills in QR code manipulation and basic encryption techniques.

## Skill Level
- **Difficulty**: Beginner - Intermediate
- **Skills Required**: QR Code Scanning and Manipulation, Basic Encryption

## Technologies
### Frontend
- Nuxt framework
- Axios for API requests
- vue-qrcode-reader for reading QR codes

### Backend
- Written in Rust
- Actix-web as the web framework
- Base64 for QR code decoding

## Software Requirements
- **Essential**: Docker
- **Recommended**: Make

## Installation
- With Make: `make up` to start and `make down` to stop
- Without Make: `docker compose up` to start and `docker compose down` to stop
