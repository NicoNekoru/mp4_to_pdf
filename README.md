Extracts the frames of a video and puts all of them into a pdf.

Requires `ffmpeg` to be installed

Usage:
```
> cargo run -- <file.mp4> <pages per second>
```

if `<pages per second>` is omitted, all frames are used.