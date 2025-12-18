#!/usr/bin/env python3
# /// script
# dependencies = ["matplotlib"]
# ///

import re
import sys

import matplotlib.pyplot as plt
import numpy as np

RECEIVED = re.compile("Received at ([0-9]+):")
SENDER_TIMESTAMP = re.compile("Sender time stamp: ([0-9]+)")

direction_points = {}

for direction_file in sys.argv[1:3]:
    points = []
    last_received = None
    for line in open(direction_file):
        if received := RECEIVED.search(line):
            last_received = int(received.groups()[0])
        if sender := SENDER_TIMESTAMP.search(line):
            sent = int(sender.groups()[0])

            points.append((sent, last_received))

    direction_points[direction_file] = points

for (direction, points) in direction_points.items():
    print(f"Analyzing {direction}")
    last_delta = None
    last_sent = None
    for (sent, received) in points:
        delta = received - sent
        print(sent, received, delta)
        if last_delta is not None:
            drift = last_delta - delta
            elapsed = sent - last_sent
            print(f"Drift {drift} ticks over {elapsed/69120000:.2f} s, {1_000_000 * drift / elapsed:.3f} ppm")
        last_delta = delta
        last_sent = sent

fig, ax = plt.subplots()
for (swap, (direction, points)) in enumerate(direction_points.items()):
    consensus_time = [(received + sent) / 2 for (sent, received) in points]
    time_delta = [sent - received if swap else received - sent for (sent, received) in points]
    color = {0: 'r+', 1: 'b+'}[swap]
    ax.plot(consensus_time, time_delta, color)
    # Not attempting anything smarter with full-file regressions -- the smart
    # thing would be to work locally.
    poly = np.polyfit(consensus_time, time_delta, deg=1)
    minmax = np.linspace(min(consensus_time), max(consensus_time), 100)
    ax.plot(minmax, np.polyval(poly, minmax), color="k")
plt.show()
