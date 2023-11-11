CGGTTS 
======

Rust package to parse and generate CGGTTS data.

[![crates.io](https://img.shields.io/crates/v/cggtts.svg)](https://crates.io/crates/cggtts)
[![Rust](https://github.com/gwbres/cggtts/actions/workflows/rust.yml/badge.svg)](https://github.com/gwbres/cggtts/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/cggtts/badge.svg)](https://docs.rs/cggtts/badge.svg)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/cggtts/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/cggtts/blob/main/LICENSE-MIT) 
[![crates.io](https://img.shields.io/crates/d/cggtts.svg)](https://crates.io/crates/cggtts)    

CGGTTS is a file format to describe a local clock behavior against a single or the combination of clocks embedded in Satellite Vehicles (SV).  
Exchanging CGGTTS files enables so called "Common View" Time Transfer.

CGGTTS is specified by the Bureau International des Poids & des Mesures (BIPM):
[CGGTTS 2E specifications](https://www.bipm.org/documents/20126/52718503/G1-2015.pdf/f49995a3-970b-a6a5-9124-cc0568f85450)

This library only supports revision **2E**, and will _reject_ other revisions.

## Ecosystem

`CGGTTS` heavily relies on `Hifitime` for accurate _Epoch_ representation.  
Check out Christopher's amazing libraries [right here](https://github.com/nyx-space/hifitime).

The [RNX2CGGTTS application](https://github.com/georust/rinex) is the "goto" application when it comes
to generate CGTTTS files. Use it to generate such files from coherent RINEX contexts.
This application is part of the GeoRust toolsuite and is the combination of this crate
and the [GNSS-RTK solver](https://github.com/rtk-rs/gnss-rtk).

## Crate achitecture

* [CGGTTS](doc/cggtts.md) is the main structure, it comprises measurement system
information and measurement data. It is a file parser (reader) and producer
(data generator).
* a CGGTTS file contains several [Tracks](doc/track.md) 
* The number of tracks in a CGGTTS is defined by the tracking specification
* CGGTTS requires accurate [delay](doc/delay.md) specifications and compensation,
because it targets 0.1 ns residual errors. 

## CGGTTS track scheduling

If you compiled the crate with the _scheduler_ feature, you can take advantage of the
`Scheduler` structure that will help you generate your synchronous CGGTTS.

Synchronous CGGTTS is convenient because it allows direct exchange of CGGTTS files
and therefore, direct remote clocks comparison.

The `Scheduler` structure works according to the BIPM definitions but we allow for a different
tracking duration. The default being 980s, you can use shorter tracking duration and faster
CGGTTS generation. You can only modify the tracking duration if you can do so on both remote clocks,
so they share the same production parameters at all times.

## System Time delays

A built in API allows accurate system delay description as defined in CGGTTS.

## CGGTTS-CLI

[A command line application](gnss_cli/README.md) is developed to process one or two CGGTTS file for clock comparison.
