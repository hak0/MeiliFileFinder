# MeiliFileFinder

![1e89440b8e37ac3e98777fd3d4df17c4_2](https://github.com/user-attachments/assets/4c3460ff-6b11-441b-b7e9-3f1bbe99e8a9)

A simple WIP file indexer for NAS, inspired by Everything, diskover, and sist2. The main goal is to have an everything-like user experience on the web with low resource usage.

It has 3 parts:
* a rust indexer to scan the filesystem and send basic metadata to the meilisearch
* a meilisearch instance
* a vue-based frontend to query files from meilisearch

Right now, it needs to manually start a meilisearch instance and assign the fixed master key. Each time the indexer is executed, it will scan the current folder and its subfolders. It is not very convenient, nor is it safe.

I will add functionalities in the future:
- [x] indexer
  - [x] add a config file for tasks with different scan endpoints and specify crontab / file ignore rules / whether to follow symlinks respectively
  - [x] add a scheduler to schedule the scan according to the config file
  - [ ] ~~instead of "cleanup and full-reindex", come up with some incremental indexing. Maybe we can use the tree structure to record folder size, newest member modification time, folder path, uuid, and only re-index changed folders. But this requires another file to store these states.~~ This should not reduce file IO so I'm not gonna add the complexity here
  - [ ] ~~scan file data, maybe similar to sist2 but simpler(text files and maybe some image metadata), but no need to scan compression files~~
  - [ ] ~~also add thumbnail and text preview~~
- [x] meilisearch
  - [ ] ~~configure master key by environment variable~~ It should be configured by the config file
  - [x] configure whether to send telemetry info by environment variable
- [ ] frontend
  - [x] a basic authentication using a master key.
  - [x] a "remember me" option to store the query tokens in the cookies
  - [x] a "logout" button(by changing the key stored in the browser local storage)
  - [ ] ~~add an option to match exactly, including symbols like "."~~ The method doesn't works well, and the stop words caused more irrelevant results to be searched
  - [ ] add filters to filter by file type, file size, and file modification time
  - [x] add a "sort by" option
  - [ ] parse file types from the extension and add filetype icons for each entry
  - [x] make the UI more user-friendly, and fit mobile devices as well
  - [ ] maybe an extra setup to configure a webdav root path, so that the files can be downloaded, making it more close to Everything
- [ ] misc
  - [ ] pack a single docker image, maybe based on the meilisearch image(alpine) for both x86-64 and arm64
  - [x] aggregate frontend and meilisearch to use the same port
