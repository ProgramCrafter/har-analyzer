Shows the media list in a given webpage, as indicated by its HAR bundle.

```
$ your-favourite-browser --export-har="example.har" --url="https://xkcd.com"
$ cat example.har | cargo run --release
     Running `target/release/har-parser`
The page "https://xkcd.com/" depends on the following images:
1. PNG image of 10.743164 KiB, loaded from https://xkcd.com/s/0b7742.png
2. PNG image of 27.060547 KiB, loaded from https://imgs.xkcd.com/s/temperature.png
3. PNG image of 123.33203 KiB, loaded from https://imgs.xkcd.com/comics/pull_2x.png
```

