# sled-debug
A program used to reproduce a sled bug

## How to use
1. build with cargo (either in release or debug mode)
2. run the binary in the root directory of this repo
3. observe the phenomenon that `sled::Db::len never` terminates.
