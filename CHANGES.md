# 0.5.3 (2022-11-12)

- Pull in Synthizer 0.111

# 0.5.2 (2022-09-24)

- Bind syz_routingRemoveAllRoutes
- Upgrade Synthizer to 0.11.9

# 0.5.1 (2022-05-28)

- Respect static-crt by making the MSVC C++ runtime match the feature (that is, static CRT is also static C++ runtime).
- Allow directly using sources in routing functions.

# 0.5.0 (2022-05-21)

- Fix a `modle` typo.

# 0.4.7 (2022-04-18)

- Add `TryFrom` for non-reference types.

# 0.4.6 (2022-03-06)

- Upgrade Synthizer to 0.11.7

# 0.4.5 (2021-12-12)

- Fix `Buffer::from_read_seek` on readers that don't always return the exact number of bytes requested.
- Avoid a heap allocation in `Buffer::from_read_seek`.
- Loosen the dependency on `asset_lru`'s version.

# 0.4.4 (2021-12-11)

- It is now possible to actually set double6 properties.

# 0.4.3 (2021-11-28)

- Upgrade Synthizer to 0.11.6.
- Bind `FastSineBankGenerator`.

# 0.4.2 (2021-11-07)

- Upgrade Synthizer to 0.11.5.
- Add a method to build buffers from a read+seek pair without having to go through the entire custom stream
  infrastructure.
- Add support for [asset_lru](https://docs.rs/asset_lru) via `AssetLruDecoder` as an optional feature.

# 0.4.1 (2021-10-24)

- Upgrade Synthizer to 0.11.4
- Bind `syz_bufferGetSizeInBytes`.

# 0.4.0 (2020-10-15)

- Bumped Synthizer to 0.11.3.
- Properties are now accessed like `node.gain().set(0.3)`
- Bound `Automationbatch` for controlling Synthizer automation.
