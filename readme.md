# Wheel

The journey.

![Wheel in field](./marketing/screenshot-mt.jpg)

## Running Wheel

You **must** run as release to apply the optimizations that give adequate performance.

```cli
cargo run --release
```

or

To perform profiling using [Tracy](https://github.com/bevyengine/bevy/blob/main/docs/profiling.md#tracy-profiler)

```cli
tracy
```

```cli
cargo run --release --features bevy/trace_tracy
```

or

To process assets

```cli
cargo run --features bevy/file_watcher bevy/asset_processor
```
*Note that on first build you may need to run the asset processor without the file watcher feature flag in the command.*

## Disclaimer

This project is not intended as social or political commentary. For any assumed association that is made based on visual elements of the work with symbolism or imagery external to the work, the association is not intended. The visual elements within this work were chosen exclusively for style and simplicity and not for commentary or art.

## Resources & Attribution

- [Make transform match timing](https://github.com/bevyengine/bevy/blob/latest/examples/transforms/3d_rotation.rs)
    - This removes most jitter where frames are inconsistent, keeping the motion in sync with the frame.
- [Terrain Generation with Dynamic Grass](https://dev.to/mikeam565/rust-game-dev-log-5-improved-terrain-generation-dynamic-grass-in-an-endless-world-291i)
    - [Mike's First Game](https://github.com/mikeam565/first-game)
    - Mike's projects' code was taken for the dynamic terrain, grass, and more!
