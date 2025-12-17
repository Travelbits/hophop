<!--
SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
SPDX-License-Identifier: MIT OR Apache-2.0
-->
External relevant documents
===========================

with selective relevant highlights

* Main documents: ETSI TS 103 636-{1,2,3,4,5}

    * [-1](https://www.etsi.org/deliver/etsi_ts/103600_103699/10363601/02.01.01_60/ts_10363601v020101p.pdf): Overview.

        Highlights: Figures 5.3 for topology overview; Figure 6.1-2 for protocol stack; Figure 7.2-2 for onboarding.

    * [-2](https://www.etsi.org/deliver/etsi_ts/103600_103699/10363602/02.01.01_60/ts_10363602v020101p.pdf): Radio.

        Highlihgts: Table 5.4.2-1 for band numbers and absolute channel frequency numbers.

    * [-3](https://www.etsi.org/deliver/etsi_ts/103600_103699/10363603/02.01.01_60/ts_10363603v020101p.pdf): Physical layer.

        Highlights: Sections 4.4 and 5.1 describe the frame and packet structure, which is relevant for timings.

    * [-4](https://www.etsi.org/deliver/etsi_ts/103600_103699/10363604/02.01.01_60/ts_10363604v020101p.pdf): MAC layer.

        Relevant throughout.

    * [-5](https://www.etsi.org/deliver/etsi_ts/103600_103699/10363605/02.01.01_60/ts_10363605v020101p.pdf): DLC and CVG layers.

        Unsure; probably relevant throughout.

* [DECT-2020 NR Endpoint Multiplexing Address Allocation](https://portal.etsi.org/PNNS/Protocol-Specification-Allocation/DECT-2020-NR-Endpoint-Multiplexing-Addresses)

    The best approximation ETSI has of IANA?

* ETSI TS 103 874-{1,2,3}: Profiles

    * [-1](https://www.etsi.org/deliver/etsi_ts/103800_103899/10387401/01.01.01_60/ts_10387401v010101p.pdf): Profile overview.

    * [-2](https://www.etsi.org/deliver/etsi_ts/103800_103899/10387402/01.01.01_60/ts_10387402v010101p.pdf): Smart metering, City, Buildings.

        Not applicable.

    * [-3](https://www.etsi.org/deliver/etsi_ts/103800_103899/10387403/01.01.01_60/ts_10387403v010101p.pdf): IPv6 Profile

        Highlights: 5.6 claims to have all that's needed to match it to 6lo

* [ETSI EN 301 406-2](https://www.etsi.org/deliver/etsi_en/301400_301499/30140602/03.01.01_60/en_30140602v030101p.pdf): Harmonised Standard for access to radio spectrum

    Much of this is covered in the PHY.

    Relevant: eg. in 4.3.2.3 effectively says "use only odd absolute channel numbers for 1.728MHz wide operation", also power limits.
