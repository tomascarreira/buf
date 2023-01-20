# BUF (BackUp File)

A cli tool to backup a single file

## Instalation

Clone this repository.

Change directory to the cloned one.

Build the program `cargo build`

## Usage

`cargo run -- <FILE>` to create a backup of FILE named FILE.backup.

## Features planned

Restore the file to the backup (Copy FILE.backup to FILE and delete FILE.backup)

Swap the FILE and FILE.backup
