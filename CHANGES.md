# 0.4.4

- It is now possible to actually set double6 properties.

# 0.4.3 (2021-11-28)

- Upgrade Synthizer to 0.11.6.
- Bind `FastSineBankGenerator`.

# 0.4.2 (2021-11-07)

- Upgrade Synthizer to 0.11.5.
- Add a method to build buffers from a read+seek pair without having to go
  through the entire custom stream infrastructure.
- Add support for [asset_lru](https://docs.rs/asset_lru) via `AssetLruDecoder`
  as an optional feature.

# 0.4.1 (2021-10-24)

- Upgrade Synthizer to 0.11.4
- Bind `syz_bufferGetSizeInBytes`.

# 0.4.0 (2020-10-15)

- Bumped Synthizer to 0.11.3.
- Properties are now accessed like `node.gain().set(0.3)`
- Bound `Automationbatch` for controlling Synthizer automation.
