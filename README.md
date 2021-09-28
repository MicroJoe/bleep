<p align="center">
    <img src="doc/bleep.png" alt="Bleep logo">
</p>

[![Build](https://github.com/MicroJoe/bleep/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/MicroJoe/bleep/actions/workflows/ci.yml)
[![Latest version](https://img.shields.io/crates/v/bleep.svg)](https://crates.io/crates/bleep)
[![License](https://img.shields.io/crates/l/bleep.svg)](https://crates.io/crates/bleep)

# bleep

Modern beep command replacement.

## Basic usage

Run a command, beep on completion or failure:

```
$ bleep run my-long-command
```

## Run manual beeps

Any beep can be played using the following commands:

```
$ bleep play success  # plays the "success" beep once and exits
$ bleep loop failure  # plays the "failure" beep infinitely in a loop
$ bleep loop -n3 warning  # plays the "warning" beep three times and exits
```

These commands can be combined to build an equivalent of the `bleep run`
command:

```
$ my-long-command && bleep play success || bleep loop failure
```

## Remote server

Start the server on the machine with the audio system — usually your local
development machine:

```
$ bleep server
Listenning on localhost:12345 (bleep TCP)
Listenning on localhost:12346 (beep on TCP connect)
```

Running bleep using the remote server:

```
$ bleep run --remote localhost:12345 my-long-command
```
