# task2ntfy

task2ntfy exists for an extremely small amount of people, who happen to use [taskwarrior](https://taskwarrior.org/) for task management, but who might appreciate notifications of upcoming pending tasks, but who also think [ntfy.sh](https://ntfy.sh/) is cool. If you fit that description, you might find this extremely specific application useful.

To use task2ntfy, you must first either create a subscription on ntfy.sh to send notifications to or have a subscription setup on your own locally hosted version of ntfy. If you are self-hosting, you'll need to set a path with `--base-url` or `-b`, which defaults to `https://ntfy.sh/` if unset.

## Installation

Since this is just a tiny CLI program made for basically my own needs, there are no real plans to package it beyond providing the source here. So, feel free to build it from source on GitHub with Cargo.

## Usage

`task2ntfy` has a few options you can set for the sake of sanity. The only required setting is `--subscription` or `-s` to tell `task2ntfy` where to send the notifications to. For example:

```bash
task2ntfy -s mytaskwarriornotifications
```

You can also configure how early you want to be notified, which defaults to 9:00AM local time. If you want to be notified later, you can set that time (in hours), with `--earliest` or `-e`:

```bash
task2nty -s mytaskwarriornotifications -e 12
```

task2ntfy will run continuously, checking your pending taskwarrior tasks for one that is within its notification threshold, which defaults to 24 hours. That threshold can be configured (in hours) with `--within` or `-w`:

```bash
task2nty -s mytaskwarriornotifications -e 12 -w 12
```

You can also configure how frequently to recheck taskwarrior tasks, which defaults to every 60 seconds. This time can be set (in seconds), with `--check-every` or `-c`:

```bash
task2nty -s mytaskwarriornotifications -e 12 -w 12 -c 30
```

Lastly, task2ntfy defaults to looping infinitely, but it can be set to run only once (in case you'd prefer to run it as a cron job or something similar). Add the `--once` or `-o` option for task2ntfy to stop after its first loop:

```bash
task2nty -s mytaskwarriornotifications -e 12 -w 12 -c 30 -o
```

Big thanks to the [task-hookrs](https://crates.io/crates/task-hookrs) crate for doing all the hard work and to [ureq](https://crates.io/crates/ureq) for sending the HTTP request to ntfy.
