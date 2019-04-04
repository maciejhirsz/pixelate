![Pixelate](pixelate.png)

## This logo is rendered with this crate

It's only 861 bytes for a 720x128 image. [Check out the code](tests/lib.rs).

## Why is this even a thing?!

Rendering up-scaled pixelated PNG files can be quite useful for things like:
+ QR codes.
+ Identicons like [Blockies](https://crates.io/crates/blockies).
+ Pixel Art?

Usually a PNG image data, before compression, is a bitmap that packs 3 or 4 bytes per pixel, depending whether transparency is present or not. For all of the cases above this is way more information than necessary.

PNG supports an **indexed palette** format. Using the indexing palette makes it possible not only to pack a single pixel into a single byte, but for small palettes **a pixel can be 4, 2, or even just 1 _bit_**. The logo here is using 3 colors (black, transparent background and shadow), which allows Pixelate to produce a bitmap where each pixel takes only 2 bits.

Not only does this produce smaller images after compression, smaller bitmap is also much faster to compress, as much as 10x for small palettes. This makes it ideal for rendering QR codes or identicons on the fly.

### License

Ramhorns is free software, and is released under the terms of the GNU General Public
License version 3. See [LICENSE](LICENSE).
