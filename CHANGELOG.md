# Changelog

## 0.9.0 (2022-11-11)
- Add BigInt support to Lisp ([#415](https://github.com/vinc/moros/pull/415))
- Add Conway's Game of Life ([#409](https://github.com/vinc/moros/pull/409))
- Add RTC device file ([#400](https://github.com/vinc/moros/pull/400))
- Add TCO to Lisp ([#420](https://github.com/vinc/moros/pull/420))
- Add apply to Lisp ([#410](https://github.com/vinc/moros/pull/410))
- Add cache to AtaBlockDevice ([#396](https://github.com/vinc/moros/pull/396))
- Add compilation option to set memory size ([#433](https://github.com/vinc/moros/pull/433))
- Add file append ([#387](https://github.com/vinc/moros/pull/387))
- Add if and while to Lisp ([#418](https://github.com/vinc/moros/pull/418))
- Add macro to Lisp ([#425](https://github.com/vinc/moros/pull/425))
- Add makefile arg for qemu audio ([#393](https://github.com/vinc/moros/pull/393))
- Add more bytes to RNG seed when RDRAND is not available ([#401](https://github.com/vinc/moros/pull/401))
- Add new forms to lisp ([#385](https://github.com/vinc/moros/pull/385))
- Add size unit options to commands ([#434](https://github.com/vinc/moros/pull/434))
- Add support for closing connections in request to HTTP server ([#406](https://github.com/vinc/moros/pull/406))
- Exit from alloc errors after page fault in userspace ([#404](https://github.com/vinc/moros/pull/404))
- Extend and refactor Lisp implementation ([#412](https://github.com/vinc/moros/pull/412))
- Fix editor delete ([#429](https://github.com/vinc/moros/pull/429))
- Fix makefile variables ([#394](https://github.com/vinc/moros/pull/394))
- Fix process table panic ([#435](https://github.com/vinc/moros/pull/435))
- Fix SATA LBA2 wrong address ([#388](https://github.com/vinc/moros/pull/388))
- Improve HTTP server ([#391](https://github.com/vinc/moros/pull/391))
- Improve Lisp forms ([#402](https://github.com/vinc/moros/pull/402))
- Improve QEMU options ([#384](https://github.com/vinc/moros/pull/384))
- Improve http timeout ([#397](https://github.com/vinc/moros/pull/397))
- Improve shell ([#405](https://github.com/vinc/moros/pull/405))
- Move /ini/lisp to /lib/lisp ([#398](https://github.com/vinc/moros/pull/398))
- Reduce filesizes ([#430](https://github.com/vinc/moros/pull/430))
- Refactor Lisp ([#417](https://github.com/vinc/moros/pull/417))
- Run clippy ([#424](https://github.com/vinc/moros/pull/424))
- Run clippy ([#438](https://github.com/vinc/moros/pull/438))
- Switch HTTP server to HTTP 1.1 ([#395](https://github.com/vinc/moros/pull/395))
- Update lisp doc and examples ([#428](https://github.com/vinc/moros/pull/428))
- Update pc-keyboard from 0.5.1 to 0.6.1 ([#423](https://github.com/vinc/moros/pull/423))
- Update rust version ([#432](https://github.com/vinc/moros/pull/432))
- Update shell redirections ([#399](https://github.com/vinc/moros/pull/399))
- Update users file ([#389](https://github.com/vinc/moros/pull/389))
- Use git describe to display version ([#437](https://github.com/vinc/moros/pull/437))
- Bump base64 from 0.13.0 to 0.13.1 ([#422](https://github.com/vinc/moros/pull/422))
- Bump libm from 0.2.3 to 0.2.5 ([#386](https://github.com/vinc/moros/pull/386))
- Bump libm from 0.2.5 to 0.2.6 ([#436](https://github.com/vinc/moros/pull/436))
- Bump linked_list_allocator from 0.10.1 to 0.10.3 ([#408](https://github.com/vinc/moros/pull/408))
- Bump linked_list_allocator from 0.10.3 to 0.10.4 ([#416](https://github.com/vinc/moros/pull/416))
- Bump raw-cpuid from 10.4.0 to 10.5.0 ([#390](https://github.com/vinc/moros/pull/390))
- Bump raw-cpuid from 10.5.0 to 10.6.0 ([#411](https://github.com/vinc/moros/pull/411))
- Bump sha2 from 0.10.2 to 0.10.3 ([#403](https://github.com/vinc/moros/pull/403))
- Bump sha2 from 0.10.3 to 0.10.5 ([#407](https://github.com/vinc/moros/pull/407))
- Bump sha2 from 0.10.5 to 0.10.6 ([#413](https://github.com/vinc/moros/pull/413))

## 0.8.0 (2022-08-05)
- Add Brautigan poem ([#373](https://github.com/vinc/moros/pull/373))
- Add CSI sequence for enabling or disabling echo to userspace ([#333](https://github.com/vinc/moros/pull/333))
- Add a reboot command ([#328](https://github.com/vinc/moros/pull/328))
- Add binaries ([#350](https://github.com/vinc/moros/pull/350))
- Add lazy allocation ([#275](https://github.com/vinc/moros/pull/275))
- Add shell aliases ([#357](https://github.com/vinc/moros/pull/357))
- Add shell globbing ([#352](https://github.com/vinc/moros/pull/352))
- Add shell variables ([#348](https://github.com/vinc/moros/pull/348))
- Add socket command ([#341](https://github.com/vinc/moros/pull/341))
- Add tilde expansion to shell ([#367](https://github.com/vinc/moros/pull/367))
- Add time command ([#346](https://github.com/vinc/moros/pull/346))
- Add userspace entry point macro ([#354](https://github.com/vinc/moros/pull/354))
- Build only moros image ([#340](https://github.com/vinc/moros/pull/340))
- Executable loading ([#349](https://github.com/vinc/moros/pull/349))
- Fix args ptr alignment ([#359](https://github.com/vinc/moros/pull/359))
- Fix device reading ([#329](https://github.com/vinc/moros/pull/329))
- Fix issues with process alloc ([#327](https://github.com/vinc/moros/pull/327))
- Fix variables expansion ([#370](https://github.com/vinc/moros/pull/370))
- Improve FUSE driver with write and delete ([#292](https://github.com/vinc/moros/pull/292))
- Improve Lisp ([#344](https://github.com/vinc/moros/pull/344))
- Improve dhcp command ([#335](https://github.com/vinc/moros/pull/335))
- Improve http command ([#365](https://github.com/vinc/moros/pull/365))
- Improve lisp ([#362](https://github.com/vinc/moros/pull/362))
- Improve regex for redirections ([#356](https://github.com/vinc/moros/pull/356))
- Read command line args from userspace programs ([#351](https://github.com/vinc/moros/pull/351))
- Refactor code ([#330](https://github.com/vinc/moros/pull/330))
- Refactor serial ([#336](https://github.com/vinc/moros/pull/336))
- Remove superfluous use of lazy static ([#364](https://github.com/vinc/moros/pull/364))
- Replace ChaCha RNG by HC-128 ([#338](https://github.com/vinc/moros/pull/338))
- Replace clock syscalls with device files ([#345](https://github.com/vinc/moros/pull/345))
- Rewrite network interface ([#334](https://github.com/vinc/moros/pull/334))
- Store current direction in DIR shell variable ([#355](https://github.com/vinc/moros/pull/355))
- Upgrade linked list allocator ([#363](https://github.com/vinc/moros/pull/363))
- Use exit code to set status var ([#360](https://github.com/vinc/moros/pull/360))
- Bump libm from 0.2.2 to 0.2.3 ([#372](https://github.com/vinc/moros/pull/372))
- Bump object from 0.28.3 to 0.28.4 ([#339](https://github.com/vinc/moros/pull/339))
- Bump object from 0.28.4 to 0.29.0 ([#358](https://github.com/vinc/moros/pull/358))
- Bump raw-cpuid from 10.3.0 to 10.4.0 ([#375](https://github.com/vinc/moros/pull/375))
- Bump smoltcp from 0.8.0 to 0.8.1 ([#342](https://github.com/vinc/moros/pull/342))
- Bump spin from 0.9.3 to 0.9.4 ([#369](https://github.com/vinc/moros/pull/369))
- Bump vte from 0.10.1 to 0.11.0 ([#371](https://github.com/vinc/moros/pull/371))
- Bump x86_64 from 0.14.9 to 0.14.10 ([#368](https://github.com/vinc/moros/pull/368))

## 0.7.1 (2022-04-10)
- Add 2048 game ([#295](https://github.com/vinc/moros/pull/295))
- Add Box to process data ([#306](https://github.com/vinc/moros/pull/306))
- Add directory support to userspace ([#303](https://github.com/vinc/moros/pull/303))
- Autocomplete binary path ([#324](https://github.com/vinc/moros/pull/324))
- Display RTC during boot ([#298](https://github.com/vinc/moros/pull/298))
- Fix various issues while reading files ([#307](https://github.com/vinc/moros/pull/307))
- Handle backtab key for backward autocompletion ([#321](https://github.com/vinc/moros/pull/321))
- Improve ATA driver ([#286](https://github.com/vinc/moros/pull/286))
- Improve documentation ([#294](https://github.com/vinc/moros/pull/294))
- Improve file reading ([#296](https://github.com/vinc/moros/pull/296))
- Improve help ([#291](https://github.com/vinc/moros/pull/291))
- Pin rustc version ([#287](https://github.com/vinc/moros/pull/287))
- Refactor code ([#288](https://github.com/vinc/moros/pull/288))
- Refactor network commands ([#322](https://github.com/vinc/moros/pull/322))
- Update rust toolchain ([#320](https://github.com/vinc/moros/pull/320))
- Upgrade smoltcp ([#293](https://github.com/vinc/moros/pull/293))
- Bump aml from 0.16.0 to 0.16.1 ([#297](https://github.com/vinc/moros/pull/297))
- Bump bootloader from 0.9.19 to 0.9.20 ([#290](https://github.com/vinc/moros/pull/290))
- Bump bootloader from 0.9.20 to 0.9.21 ([#302](https://github.com/vinc/moros/pull/302))
- Bump hmac from 0.12.0 to 0.12.1 ([#313](https://github.com/vinc/moros/pull/313))
- Bump libm from 0.2.1 to 0.2.2 ([#309](https://github.com/vinc/moros/pull/309))
- Bump nom from 7.1.0 to 7.1.1 ([#315](https://github.com/vinc/moros/pull/315))
- Bump object from 0.27.1 to 0.28.3 ([#305](https://github.com/vinc/moros/pull/305))
- Bump pbkdf2 from 0.10.0 to 0.11.0 ([#316](https://github.com/vinc/moros/pull/316))
- Bump rand from 0.8.4 to 0.8.5 ([#310](https://github.com/vinc/moros/pull/310))
- Bump rand_chacha from 0.3.0 to 0.3.1 ([#175](https://github.com/vinc/moros/pull/175))
- Bump raw-cpuid from 10.2.0 to 10.3.0 ([#317](https://github.com/vinc/moros/pull/317))
- Bump sha2 from 0.10.0 to 0.10.1 ([#299](https://github.com/vinc/moros/pull/299))
- Bump sha2 from 0.10.1 to 0.10.2 ([#312](https://github.com/vinc/moros/pull/312))
- Bump uart_16550 from 0.2.15 to 0.2.16 ([#301](https://github.com/vinc/moros/pull/301))
- Bump uart_16550 from 0.2.16 to 0.2.17 ([#318](https://github.com/vinc/moros/pull/318))
- Bump x86_64 from 0.14.6 to 0.14.7 ([#289](https://github.com/vinc/moros/pull/289))
- Bump x86_64 from 0.14.7 to 0.14.8 ([#308](https://github.com/vinc/moros/pull/308))
- Bump x86_64 from 0.14.8 to 0.14.9 ([#319](https://github.com/vinc/moros/pull/319))


## 0.7.0 (2021-12-12)
- Add ELF loader ([#248](https://github.com/vinc/moros/pull/248))
- Add basic userspace ([#228](https://github.com/vinc/moros/pull/228))
- Add calc command ([#263](https://github.com/vinc/moros/pull/263))
- Add dynamical disk information ([#252](https://github.com/vinc/moros/pull/252))
- Add file syscalls ([#242](https://github.com/vinc/moros/pull/242))
- Add pci command and switch IDE controllers to compatible mode ([#276](https://github.com/vinc/moros/pull/276))
- Add process table and exit syscall ([#268](https://github.com/vinc/moros/pull/268))
- Add partial rust binaries support ([#255](https://github.com/vinc/moros/pull/255))
- Add shell redirections ([#262](https://github.com/vinc/moros/pull/262))
- Add spawn syscall ([#251](https://github.com/vinc/moros/pull/251))
- Add time module to API ([#284](https://github.com/vinc/moros/pull/284))
- Add website ([#261](https://github.com/vinc/moros/pull/261))
- Extend lisp language ([#278](https://github.com/vinc/moros/pull/278))
- Fix DNS address ([#279](https://github.com/vinc/moros/pull/279))
- Fix VGA issues with real hardware ([#258](https://github.com/vinc/moros/pull/258))
- Improve UTF-8 support ([#267](https://github.com/vinc/moros/pull/267))
- Improve user experience ([#274](https://github.com/vinc/moros/pull/274))
- Remove array-macro dependency ([#253](https://github.com/vinc/moros/pull/253))
- Rewrite Lisp parser with Nom ([#277](https://github.com/vinc/moros/pull/277))
- Bump acpi from 3.1.0 to 4.0.0 ([#243](https://github.com/vinc/moros/pull/243))
- Bump acpi from 4.0.0 to 4.1.0 ([#265](https://github.com/vinc/moros/pull/265))
- Bump aml from 0.14.0 to 0.15.0 ([#236](https://github.com/vinc/moros/pull/236))
- Bump aml from 0.15.0 to 0.16.0 ([#241](https://github.com/vinc/moros/pull/241))
- Bump linked_list_allocator from 0.9.0 to 0.9.1 ([#256](https://github.com/vinc/moros/pull/256))
- Bump pbkdf2 from 0.8.0 to 0.9.0 ([#239](https://github.com/vinc/moros/pull/239))
- Bump pic8259 from 0.10.1 to 0.10.2 ([#235](https://github.com/vinc/moros/pull/235))
- Bump sha2 from 0.9.5 to 0.9.6 ([#240](https://github.com/vinc/moros/pull/240))
- Bump sha2 from 0.9.6 to 0.9.8 ([#244](https://github.com/vinc/moros/pull/244))
- Bump x86_64 from 0.14.4 to 0.14.5 ([#245](https://github.com/vinc/moros/pull/245))
- Bump x86_64 from 0.14.5 to 0.14.6 ([#247](https://github.com/vinc/moros/pull/247))

## 0.6.0 (2021-08-21)
- Add beep command ([#234](https://github.com/vinc/moros/pull/234))
- Add Lisp interpreter ([#207](https://github.com/vinc/moros/pull/207))
- Add VGA font loader ([#201](https://github.com/vinc/moros/pull/201))
- Add VGA palette loader ([#203](https://github.com/vinc/moros/pull/203))
- Add chess game ([#230](https://github.com/vinc/moros/pull/230))
- Add file offset ([#206](https://github.com/vinc/moros/pull/206))
- Add keyboard layout change at runtime ([#226](https://github.com/vinc/moros/pull/226))
- Add regular expression engine ([#222](https://github.com/vinc/moros/pull/222))
- Add syscalls ([#196](https://github.com/vinc/moros/pull/196))
- Add time to dir entry ([#215](https://github.com/vinc/moros/pull/215))
- Improve baremetal experience ([#232](https://github.com/vinc/moros/pull/232))
- Fix clippy warnings ([#214](https://github.com/vinc/moros/pull/2154))
- Move kernel code to api ([#204](https://github.com/vinc/moros/pull/204))
- Refactor editor ([#221](https://github.com/vinc/moros/pull/221))
- Refactor filesystem ([#225](https://github.com/vinc/moros/pull/225))
- Refactor line editing ([#212](https://github.com/vinc/moros/pull/212))
- Refactor print macros ([#208](https://github.com/vinc/moros/pull/208))
- Remove volatile crate ([#219](https://github.com/vinc/moros/pull/219))
- Update acpi crate from v2.3.1 to v3.1.0 ([#218](https://github.com/vinc/moros/pull/218))
- Update crypto crates ([#216](https://github.com/vinc/moros/pull/216))
- Update raw-cpuid from v9.0.0 to v10.0.0 ([#220](https://github.com/vinc/moros/pull/220))
- Use CSI for key events ([#210](https://github.com/vinc/moros/pull/210))
- Bump aml from 0.13.0 to 0.14.0 ([#227](https://github.com/vinc/moros/pull/227))
- Bump bootloader from 0.9.18 to 0.9.19 ([#233](https://github.com/vinc/moros/pull/233))
- Bump raw-cpuid from 10.0.0 to 10.2.0 ([#224](https://github.com/vinc/moros/pull/224))
- Bump spin from 0.9.1 to 0.9.2 ([#202](https://github.com/vinc/moros/pull/202))
- Bump x86_64 from 0.14.3 to 0.14.4 ([#209](https://github.com/vinc/moros/pull/209))

## 0.5.1 (2021-06-27)
- Add missing RX stats to PCNET driver ([#124](https://github.com/vinc/moros/pull/124))
- Disable `rand_chacha` with `debug_assertions` ([#120](https://github.com/vinc/moros/pull/120))
- Fix PCNET BCNT computation ([#122](https://github.com/vinc/moros/pull/122))
- Fix compilation errors ([#184](https://github.com/vinc/moros/pull/184))
- Migrate from TravisCI to GitHub Actions ([#131](https://github.com/vinc/moros/pull/131))
- Update aml crate ([#195](https://github.com/vinc/moros/pull/195))
- Update smoltcp crate ([#194](https://github.com/vinc/moros/pull/194))
- Bump acpi from 2.2.0 to 2.3.1 ([#180](https://github.com/vinc/moros/pull/180))
- Bump array-macro from 1.0.5 to 2.1.0 ([#188](https://github.com/vinc/moros/pull/188))
- Bump rand from 0.8.3 to 0.8.4 ([#176](https://github.com/vinc/moros/pull/176))
- Bump rand_core from 0.6.1 to 0.6.3 ([#185](https://github.com/vinc/moros/pull/185))
- Bump raw-cpuid from 8.1.2 to 9.0.0 ([#191](https://github.com/vinc/moros/pull/191))
- Bump spin from 0.7.1 to 0.9.1 ([#181](https://github.com/vinc/moros/pull/181))
- Bump time from 0.2.25 to 0.2.27 ([#186](https://github.com/vinc/moros/pull/186))
- Bump vte from 0.10.0 to 0.10.1 ([#174](https://github.com/vinc/moros/pull/174))

## 0.5.0 (2020-11-15)
- Add ACPI shutdown ([#111](https://github.com/vinc/moros/pull/111))
- Add a web server ([#114](https://github.com/vinc/moros/pull/114))
- Add nanowait busy loop with nanoseconds precision ([#78](https://github.com/vinc/moros/pull/78))
- Add new `date` and `env` commands ([#112](https://github.com/vinc/moros/pull/112))
- Add new `mem` command ([#113](https://github.com/vinc/moros/pull/113))
- Add pcnet driver ([#82](https://github.com/vinc/moros/pull/82))
- Add tests ([#118](https://github.com/vinc/moros/pull/118))
- Improve text editor ([#109](https://github.com/vinc/moros/pull/109))
- Remove cargo xbuild ([#83](https://github.com/vinc/moros/pull/83))
- Remove dependency on `rlibc` ([#115](https://github.com/vinc/moros/pull/115))
- Use ChaCha20 RNG ([#116](https://github.com/vinc/moros/pull/116))

## 0.4.0 (2020-07-29)
- Add ANSI Style type ([#76](https://github.com/vinc/moros/pull/76))
- Colorize user interface ([#69](https://github.com/vinc/moros/pull/69))
- Fix ATA busy loop hang
- Fix detection of magic superblock
- Handle RTC interrupts ([#71](https://github.com/vinc/moros/pull/71))
- Improve ATA reset
- Improve console ([#74](https://github.com/vinc/moros/pull/74))
- Improve editor ([#77](https://github.com/vinc/moros/pull/77))
- Improve installation and documentation ([#73](https://github.com/vinc/moros/pull/73))
- Optimize shell printing ([#75](https://github.com/vinc/moros/pull/75))
- Update dependencies ([#70](https://github.com/vinc/moros/pull/70))

## 0.3.1 (2020-04-13)
- Update ATA driver ([#41](https://github.com/vinc/moros/pull/41))
- Update dependencies ([#42](https://github.com/vinc/moros/pull/42))

## 0.3.0 (2020-02-16)
- Add PhysBuf for DMA ([#16](https://github.com/vinc/moros/pull/16))
- Add geotime command ([#14](https://github.com/vinc/moros/pull/14))
- Add process struct ([#19](https://github.com/vinc/moros/pull/19))
- Add tcp command ([#17](https://github.com/vinc/moros/pull/17))
- Improve filesystem ([#24](https://github.com/vinc/moros/pull/24))
- Improve shell history ([#18](https://github.com/vinc/moros/pull/18))
- Use VGA color palette ([#15](https://github.com/vinc/moros/pull/15))

## 0.2.0 (2020-02-02)
- Add autocompletion to shell
- Add heap allocation
- Add network stack

## 0.1.0 (2020-01-18)
- Add ATA PIO mode
- Add PCI enumeration
- Add RTC clock
- Add editor
- Add filesystem
- Add shell

## 0.0.0 (2019-12-28)
- Start MOROS project
