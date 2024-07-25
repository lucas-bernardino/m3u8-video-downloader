# m3u8-video-downloader

A CLI script to help you download videos in websites that use m3u8 extension. Written in Rust, btw.

## Building

You need to have the Rust ecosystem installed on your machine to run this application. If you're using Linux, you can install it by simply running
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After having installed Rust, clone this repository and enter its folder
```
git clone https://github.com/lucas-bernardino/m3u8-video-downloader.git
cd m3u8-video-downloader
```

You can then build this application by running
```
cargo build -r 
```

## Usage

In order to be able to run the application, you need to have a m3u8 file containing the pieces of the video. You can see an example of how it should be like in the file called ***input_example.m3u8***

To use, you can simple run
```
cargo run -r -- -s <SOURCE> -o <OUTPUT>
```
In this case, <SOURCE> is the path to your .m3u8 file and <OUTPUT> is where you want to save the video. 
With the example file, you could get the video by running
```
cargo run -r -- -s input_example.m3u8 -o video.mp4
```