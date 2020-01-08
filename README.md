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
Received from ./noiseclean [master @ N/A]: 
Received from ./systemd-vmware [master @ N/A]: 
Received from ./jEdit-Rust [master @ N/A]: 
Received from ./WSWireLib [master @ N/A]: 
Received from ./signal-reset [master @ N/A]: 
Received from ./li3_loggr [master @ N/A]: 
Received from ./li3_partials [master @ N/A]: 
Received from ./KickassTorrentsAPI [master @ N/A]: 
Received from ./convert-document [master @ 1.4.0]: 
Received from ./baidupan [master @ N/A]: 
Received from ./mimipenguin [master @ N/A]: 
Received from ./khttp [master @ 0.1.0]: 
Received from ./DeepFaceLab [master @ N/A]: 
Received from ./sidewinderd [master @ 0.4.1]: 
Received from ./json [master @ 9.0.6]: 
Received from ./micropolis [master @ N/A]: 
Received from ./Pyrit [master @ v0.5.0]: 
Received from ./parallel [master @ 0.11.3]: 
Received from ./Exposed [master @ 0.13.6]: 
Received from ./miraclecast [master @ v1.0]: 
Received from ./nimble [master @ v0.9.0]: 
Received from ./gradle-appengine-plugin [master @ gradle-appengine-plugin-1.9.59]: 
**** Received from ./OpenCL-Gaussian-Blur [master @ N/A]: 
**** Received from ./SolidOak [master @ 0.1.3]: 
Received from ./grive2 [master @ v0.5.0]: 
**** Received from ./openfortigui [master @ v0.7.3.1]: 
Received from ./ripgrep [master @ 11.0.1]: 
Received from ./getting-started-java [master @ N/A]: 
Received from ./spring-data-jpa [master @ 2.2.0.M3]: 
Received from ./bdr-plugin [bdr-plugin/REL0_9_STABLE @ bdr-plugin/0.9.3-2]: 
Received from ./dex2jar [2.x @ 2.1-nightly-28]: 
Received from ./Aporia [master @ v0.4.2]: 
Received from ./http4k [master @ N/A]: 
Received from ./spring-data-commons [master @ 2.2.0.M3]: 
**** Received from ./Nim [version-1-0 @ v1.0.0]: 
Received from ./spring-security-oauth [master @ 2.3.5.RELEASE]: 
Received from ./karate [master @ v0.9.2]: 
?!?! Received from ./liteide [master @ x30.2]: 
Received from ./youtube-dl [master @ 2019.09.01]: 
Received from ./nerd-fonts [master @ v2.0.0]: 
Received from ./neovim [master @ nightly]: 
Received from ./hibernate-orm [master @ 5.4.2]: 
Received from ./postgresql-bdr [bdr-pg/REL9_4_STABLE @ bdr-pg/REL9_4_6-1]: 
Received from ./openapi-generator [master @ v4.1.1]: 
gstats  1.02s user 0.42s system 286% cpu 0.502 total

```
