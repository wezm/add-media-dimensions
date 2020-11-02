# add-media-dimensions

This is a one-off Rust "script" I built as part of building my
[#100binaries blog post](https://www.wezm.net/v2/posts/2020/100-rust-binaries/).

This is what it does:

1. Reads the JSON document.
2. For each object with a `media` field run `ffprobe` on the file to determine
   its dimensions.
3. Update the JSON object with the width and height.
4. Print out the updated JSON.

Originally I thought that I would need two code paths for videos and images but
it turns out `ffprobe` handles all formats I passed to it:

* MP4
* JPEG
* PNG
* GIF
