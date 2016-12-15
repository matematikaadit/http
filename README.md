# http [![Build status](https://travis-ci.org/thecoshman/http.svg?branch=develop)](https://travis-ci.org/thecoshman/http) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE) [![Crates.io version](http://meritbadge.herokuapp.com/https)](https://crates.io/crates/https)
Host These Things Please - a basic HTTP server for hosting a folder fast and simply

# Aims
The idea is to make a program that can compile down to a simple binary that can be used via Linux CLI to quickly take the current directory and server it over HTTP. Everything should have sensible defaults such that you do not *have* to pass parameters like what port to use.

  * [x] Sub directories would be automatically hosted.
  * [ ] Symlinks will not be followed by default (in my opinion, this is more likely to be a problem than an intended thing).
  * [x] Root should not be required.
  * [ ] If an index file isn't provided, one will be generated (in memory, no touching the disk, why would do that you dirty freak you), that will list the current files and folders (and then sub directories will have index files generated as required)
  * [x] Changes made to files should be reflected instantly, as I don't see why anything would be cached... you request a file, a file will be looked for

It's not going to be a 'production ready' tool, it's a quick and dirty way of hosting a folder, so whilst I'll try to make it secure, it is not going to be a serious goal.

# Disclaimer
I mostly just wanted to use this witty name for the project... I'll work on this at some point... maybe... very lazily...
