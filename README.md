# safe_launcher

[![](https://img.shields.io/badge/Project%20SAFE-Approved-green.svg)](http://maidsafe.net/applications) [![](https://img.shields.io/badge/License-GPL3-green.svg)](https://github.com/maidsafe/crust/blob/master/COPYING)


**Primary Maintainer:**     Spandan Sharma (spandan.sharma@maidsafe.net)

**Secondary Maintainer:**   Krishna Kumar (krishna.kumar@maidsafe.net)

|Crate|Linux/OS X|Windows|Coverage|Issues|
|:---:|:--------:|:-----:|:------:|:----:|
|[![](http://meritbadge.herokuapp.com/safe_launcher)](https://crates.io/crates/safe_launcher)|[![Build Status](https://travis-ci.org/maidsafe/safe_launcher.svg?branch=master)](https://travis-ci.org/maidsafe/safe_launcher)|[![Build status](https://ci.appveyor.com/api/projects/status/xnsjhx27snoh4lmy/branch/master?svg=true)](https://ci.appveyor.com/project/MaidSafe-QA/safe-launcher/branch/master)|[![Coverage Status](https://coveralls.io/repos/maidsafe/safe_launcher/badge.svg?branch=master)](https://coveralls.io/r/maidsafe/safe_launcher?branch=master)|[![Stories in Ready](https://badge.waffle.io/maidsafe/safe_launcher.png?label=ready&title=Ready)](https://waffle.io/maidsafe/safe_launcher)|

| [API Documentation - master branch](http://maidsafe.net/safe_launcher/master) | [SAFE Network System Documentation](http://systemdocs.maidsafe.net) | [MaidSafe website](http://maidsafe.net) | [SAFE Network Forum](https://forum.safenetwork.io) |
|:------:|:-------:|:-------:|:-------:|

## Prerequisite

[libsodium](https://github.com/jedisct1/libsodium) is a native dependency, and can be installed by following the instructions [for Windows](https://github.com/maidsafe/QA/blob/master/Documentation/Install%20libsodium%20for%20Windows.md) or [for OS X and Linux](https://github.com/maidsafe/QA/blob/master/Documentation/Install%20libsodium%20for%20OS%20X%20or%20Linux.md).

## Build Instructions

`safe_launcher` can interface conditionally against either the routing crate or a mock used for local testing.

To use it with the Mock:
```
cargo build --features "use-mock-routing"
cargo test --features "use-mock-routing"
```

To interface it with actual routing (default):
```
cargo build
cargo test
```

## TODO
### [0.1.0]
- [ ] [MAID-1401](https://maidsafe.atlassian.net/browse/MAID-1401) Launcher Start-up
  - [ ] [MAID-1402](https://maidsafe.atlassian.net/browse/MAID-1402) Self-Authenticate
  - [ ] [MAID-1403](https://maidsafe.atlassian.net/browse/MAID-1403) Handle Session Packet
  - [X] [MAID-1404](https://maidsafe.atlassian.net/browse/MAID-1404) Threaded TCP server, Design
  - [ ] [MAID-1405](https://maidsafe.atlassian.net/browse/MAID-1405) Implement Threaded TCP server
  - [ ] [MAID-1406](https://maidsafe.atlassian.net/browse/MAID-1406) Test Async TCP server by writing a mock app client
- [ ] [MAID-1407](https://maidsafe.atlassian.net/browse/MAID-1407) Launcher App Handling
  - [ ] [MAID-1408](https://maidsafe.atlassian.net/browse/MAID-1408) Handle Adding of App
  - [ ] [MAID-1409](https://maidsafe.atlassian.net/browse/MAID-1409) App authentication
  - [ ] [MAID-1410](https://maidsafe.atlassian.net/browse/MAID-1410) App Removal
- [ ] [MAID-1411](https://maidsafe.atlassian.net/browse/MAID-1411) Launcher provided services to apps
  - [ ] [MAID-1412](https://maidsafe.atlassian.net/browse/MAID-1412) Design JSON dispatcher
  - [ ] [MAID-1413](https://maidsafe.atlassian.net/browse/MAID-1413) Create Directory
  - [ ] [MAID-1414](https://maidsafe.atlassian.net/browse/MAID-1414) Delete Directory
  - [ ] [MAID-1415](https://maidsafe.atlassian.net/browse/MAID-1415) Modify Directory
  - [ ] [MAID-1416](https://maidsafe.atlassian.net/browse/MAID-1416) Get Directory
  - [ ] [MAID-1417](https://maidsafe.atlassian.net/browse/MAID-1417) Create File
  - [ ] [MAID-1418](https://maidsafe.atlassian.net/browse/MAID-1418) Delete File
  - [ ] [MAID-1419](https://maidsafe.atlassian.net/browse/MAID-1419) Modify File
  - [ ] [MAID-1420](https://maidsafe.atlassian.net/browse/MAID-1420) Get File
  - [ ] [MAID-1421](https://maidsafe.atlassian.net/browse/MAID-1421) Register DNS
  - [ ] [MAID-1422](https://maidsafe.atlassian.net/browse/MAID-1422) Add Service to registered DNS
- [ ] [MAID-1440](https://maidsafe.atlassian.net/browse/MAID-1440) Launcher CLI Example
  - [ ] [MAID-1441](https://maidsafe.atlassian.net/browse/MAID-1441)
  - [ ] [MAID-1442](https://maidsafe.atlassian.net/browse/MAID-1442)
  - [ ] [MAID-1443](https://maidsafe.atlassian.net/browse/MAID-1443)
  - [ ] [MAID-1444](https://maidsafe.atlassian.net/browse/MAID-1444)
  - [ ] [MAID-1445](https://maidsafe.atlassian.net/browse/MAID-1445)
  - [ ] [MAID-1446](https://maidsafe.atlassian.net/browse/MAID-1446)
  - [ ] [MAID-1447](https://maidsafe.atlassian.net/browse/MAID-1447)