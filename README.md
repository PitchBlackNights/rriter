# rriter

A TUI markdown editor made with rust

## Features/TODO

In order of priority (Greatest to Least)

- [x] Quitting  (CTRL+[q]x2)
- [ ] Text Input
- [ ] Markdown Syntax
- [ ] Settings
- [ ] Serialization system
  - [ ] Save  (CTRL+s)
  - [ ] Load  (CTRL+l)
  - [ ] Export As  (CTRL+ALT+s)
- [ ] Copy System
  - [ ] Copy  (CTRL+c)
  - [ ] Copy/Paste History  (CTRL+ALT+v)
  - [ ] Paste  (CTRL+v)
  - [ ] Cut  (CTRL+x)
- [ ] Tabs
- [ ] Update Warning
- [ ] Counters
  - [ ] Word Counter
  - [ ] Character Counter
  - [ ] Reading Time
  - [ ] Goal Setting
- [ ] Text finding
  - [ ] Find  (CTRL+f)
  - [ ] Replace
  - [ ] Pattern Find (regex)
- [ ] Themes
  - [ ] Built-in Themes
  - [ ] Built-in Font Color
  - [ ] Custom Themes
  - [ ] Custom Font Color
- [ ] Spell Checker
- [ ] Popup Toolbar  (CTRL+t)
- [ ] Printing

## Licenses

This project is licensed under GNU AGPL (Affero General Public License) (see `LICENSE.md`)

`/src/text_input/handler.rs` contains code from `/src/backend/crossterm.rs` in [tui-input](https://github.com/sayanarijit/tui-input/blob/710017a4c29bbffffe1b04548c2bf2756f427023/src/input.rs) (Licensed under the MIT License)

```Text
MIT License

Copyright (c) 2021 Arijit Basu

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

`/src/{app.rs, event.rs, handler.rs, lib.rs, main.rs, tui.rs, ui.rs}` contains code from `/src/{app.rs, event.rs, handler.rs, lib.rs, main.rs, tui.rs, ui.rs}` in [rust-tui-template](https://github.com/ratatui-org/rust-tui-template/tree/33bde3d0a53732147a6cda4be17db0ac1dc10d8b/src) (Licensed under MIT License)

```Text
Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
```
