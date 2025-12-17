<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
Running hophop examples
=======================

For all examples
----------------

* Get an [nRF9151-DK](../doc/hardware.md), connect it and turn it on.

* Ensure you are set up to run Ariel OS examples.

  For the time being, it's easiest to follow the [Getting Started section in the Ariel OS book](https://ariel-os.github.io/ariel-os/dev/docs/book/getting-started.html).

* [Ensure that you have the DECT firmware running](../doc/dect-firmware.md).

* Beware that this is a research example,
  and that depending on your location, regulation on operating these devices does apply,
  especially as some examples emit transmissions controlled by the user alone.

* If ever you have multiple DKs connected,
  probe-rs will give you interactive options.

  You can skip that and statically set the probe to use
  by adding `-- --probe 1366:1059:xxxxxxxxxxxx` after the laze call (or withtout the `--` if you already).

Running the RSSI example
------------------------

* In this directory, run:

    ```console
    $ laze build -b nrf9151-dk run --bin rssi
    ```

    You can add the option `-D LOG=trace` before the `run` for more verbosity.

    The output this produces is a scan of Band 1, which is printed on screen;
    it's best to let that run through.

*   To visualize the output, store it by running

    ```
    $ laze build -b nrf9151-dk run --bin rssi -- --target-output-file=rssi.log
    ```

    and run the visualizer:

    ```
    $ ./show-rssi.py rssi.log
    ```

    (If anything is missing on the Python side, `pipx run ./show…` will fetch any missing components).

Running the RX example
----------------------

```
$ laze build -b nrf9151-dk run --bin rx
```

Note that this only produces output when data is sent on channel 1665;
use `dect ping -c` or `dect mac beacon_start -c 1665`
in the Nordic `dect_shell` example to send data from another boad.

Running the TX example
----------------------

```
$ laze build -b nrf9151-dk run --bin tx
```

This produces a single manually crafted beacon on chnanel 1665
whenever Button 1 is pressed.
Beware that this does not perform LBYT:
The user is expected to monitor that channel in parallel.
