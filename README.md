[![crates.io](https://img.shields.io/crates/v/gstats.svg)](https://crates.io/crates/gstats/)

# gstats

Simple Rust tool to get quick summary info on git repos showing latest tag, branch, state. I implemented this to help me work with a the not too small
number of repositories as a result of Microservices.

It basically shows information similar to those running "gst" on zsh using the git plugin, just for all immediate sub dirs.

The secondary reason is of course for me to try out Rust. So, please do critique or suggest improvement for the code.

## Legend

**** Has uncommited changes.

?!?! Has no origin/out-of-sync with origin.


## Installation

Install using [Rust Cargo](https://doc.rust-lang.org/cargo/commands/cargo-install.html).

```cargo install gstats```

### Sample

```
~/Apps ⌚ 12:22:14
$ time gstats
Base Dir: .
Stats of ./noiseclean [master @ N/A]: 
Stats of ./systemd-vmware [master @ N/A]: 
Stats of ./jEdit-Rust [master @ N/A]: 
Stats of ./WSWireLib [master @ N/A]: 
Stats of ./signal-reset [master @ N/A]: 
Stats of ./li3_loggr [master @ N/A]: 
Stats of ./li3_partials [master @ N/A]: 
Stats of ./KickassTorrentsAPI [master @ N/A]: 
Stats of ./convert-document [master @ 1.4.0]: 
Stats of ./baidupan [master @ N/A]: 
Stats of ./mimipenguin [master @ N/A]: 
Stats of ./khttp [master @ 0.1.0]: 
Stats of ./DeepFaceLab [master @ N/A]: 
Stats of ./sidewinderd [master @ 0.4.1]: 
Stats of ./json [master @ 9.0.6]: 
Stats of ./micropolis [master @ N/A]: 
Stats of ./Pyrit [master @ v0.5.0]: 
Stats of ./parallel [master @ 0.11.3]: 
Stats of ./Exposed [master @ 0.13.6]: 
Stats of ./miraclecast [master @ v1.0]: 
Stats of ./nimble [master @ v0.9.0]: 
Stats of ./gradle-appengine-plugin [master @ gradle-appengine-plugin-1.9.59]: 
**** Stats of ./OpenCL-Gaussian-Blur [master @ N/A]: 
**** Stats of ./SolidOak [master @ 0.1.3]: 
Stats of ./grive2 [master @ v0.5.0]: 
**** Stats of ./openfortigui [master @ v0.7.3.1]: 
Stats of ./ripgrep [master @ 11.0.1]: 
Stats of ./getting-started-java [master @ N/A]: 
Stats of ./spring-data-jpa [master @ 2.2.0.M3]: 
Stats of ./bdr-plugin [bdr-plugin/REL0_9_STABLE @ bdr-plugin/0.9.3-2]: 
Stats of ./dex2jar [2.x @ 2.1-nightly-28]: 
Stats of ./Aporia [master @ v0.4.2]: 
Stats of ./http4k [master @ N/A]: 
Stats of ./spring-data-commons [master @ 2.2.0.M3]: 
**** Stats of ./Nim [version-1-0 @ v1.0.0]: 
Stats of ./spring-security-oauth [master @ 2.3.5.RELEASE]: 
Stats of ./karate [master @ v0.9.2]: 
?!?! Stats of ./liteide [master @ x30.2]: 
Stats of ./youtube-dl [master @ 2019.09.01]: 
Stats of ./nerd-fonts [master @ v2.0.0]: 
Stats of ./neovim [master @ nightly]: 
Stats of ./hibernate-orm [master @ 5.4.2]: 
Stats of ./postgresql-bdr [bdr-pg/REL9_4_STABLE @ bdr-pg/REL9_4_6-1]: 
Stats of ./openapi-generator [master @ v4.1.1]: 
gstats  1.02s user 0.42s system 286% cpu 0.502 total

```
