Visualize images in an ansi terminal with this tool !

Todo list :
- [X] make sure it works with impair values
- [ ] handle errors when there are different line width -> partially handled but it would be better if all lines with wrong width are displayed at once
- [ ] support transparent pixels
- [ ] make code cleaner
- [ ] display warning when specified height isn't equal to the true height
- [ ] make possible to import any compatible json file with a cli argument and add an argument to dismiss the warning specified on previous checkbox
- [ ] make a tiny pixel art editor to stop having to place all the pixels manually
- [ ] fix bug : when a height is specified (and is odd), if there is another line in the file, it is displayed where it shouldn't be
- [ ] ````handle "thread 'main' panicked at src/main.rs:28:26:  index out of bounds: the len is 17 but the index is 17  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace````   when specified height isn't equal to real one"


## Example with a smiley image :
![smiley image](https://github.com/JustVal-su/tui-images-visualizer/blob/main/Capture%20d%E2%80%99%C3%A9cran%20du%202026-05-15%2013-18-02.png)


## Json file requirement

Coming soon
