# Changelog

### [0.4.1](https://www.github.com/nfejzic/capillary/compare/v0.4.0...v0.4.1) (2023-08-15)


### CI

* add waitable PR runs (maybe?) ([fd5f435](https://www.github.com/nfejzic/capillary/commit/fd5f435d7b0565b90f1aa490a1c5ac6c5725c344))
* echo a message when PR app build does not run. ([76138e2](https://www.github.com/nfejzic/capillary/commit/76138e20fb4ea4fe3d49e2db57c0de9d63f13b44))
* run workflow on 'deploy dev' comment on PR ([89c3f02](https://www.github.com/nfejzic/capillary/commit/89c3f02a1491c0a781c0f33acdbcf018a2a8b1f0))
* update ci ([55f7cc1](https://www.github.com/nfejzic/capillary/commit/55f7cc1ae9ea4206062bcb7ed7d9a9d3cde74b14))
* update ci to report build ok if label not present on a PR ([04039ea](https://www.github.com/nfejzic/capillary/commit/04039ead4e5a6a6808e3ecab055a6505cdf8ed73))
* update conditional ci to run on label 'custom-label' ([f152510](https://www.github.com/nfejzic/capillary/commit/f152510bfc84ff59479223bef8cd986e9e0ee342))

## [0.4.0](https://www.github.com/nfejzic/capillary/compare/v0.3.0...v0.4.0) (2022-07-12)


### Features

* reimplement `Dictionary` with graphs ([563b542](https://www.github.com/nfejzic/capillary/commit/563b542e96fbb8c7f0625e23e75c3cd1ef1656a6))
* replace `Dictionary` implementation ([23e80c0](https://www.github.com/nfejzic/capillary/commit/23e80c0a948e861de428ea4a1660ab2612548ddc))

## [0.3.0](https://www.github.com/nfejzic/capillary/compare/v0.2.0...v0.3.0) (2022-07-10)


### Features

* add `get` and `get_ref` API ([149d376](https://www.github.com/nfejzic/capillary/commit/149d37669c5f3127b1ede0d1a4446009f4bbf7a6))
* relax trait constraints on `Dictionary` ([4c91794](https://www.github.com/nfejzic/capillary/commit/4c91794bb3425e24007b971c1e41427cab07503c))


### Bug Fixes

* make constraints on key more unified ([c23d377](https://www.github.com/nfejzic/capillary/commit/c23d37767f2956fe50b3079aefaac85ff9d60fbf))


### Testing

* update tests to conform to the new API ([ad314d7](https://www.github.com/nfejzic/capillary/commit/ad314d7b8904f5232ab384dd4f89a8646aadc87a))

## [0.2.0](https://www.github.com/nfejzic/capillary/compare/v0.1.0...v0.2.0) (2022-06-30)


### Features

* implement basic `Dictionary` with `&str` ([8145688](https://www.github.com/nfejzic/capillary/commit/81456884f5d413021e26ae02783125dc48715136))
* introduce generics to `Dictionary` ([71df6ca](https://www.github.com/nfejzic/capillary/commit/71df6cafff049d48929f955a64c949bdbf554d36))


### CI

* add rust github actions ([18ecf21](https://www.github.com/nfejzic/capillary/commit/18ecf2149470c9b2d5af31e38538715015f2d82d))
* rename rust actions ([4ce5892](https://www.github.com/nfejzic/capillary/commit/4ce58925b031ea41641e1da5fb79be41c0382056))

## 0.1.0 (2022-06-30)


### Features

* implement basic `Dictionary` with `&str` ([8145688](https://www.github.com/nfejzic/capillary/commit/81456884f5d413021e26ae02783125dc48715136))


### CI

* add rust github actions ([18ecf21](https://www.github.com/nfejzic/capillary/commit/18ecf2149470c9b2d5af31e38538715015f2d82d))
* rename rust actions ([4ce5892](https://www.github.com/nfejzic/capillary/commit/4ce58925b031ea41641e1da5fb79be41c0382056))
