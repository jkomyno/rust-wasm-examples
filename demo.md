## Before the Presentation

- Create 3 terminals:
  - Terminal 1 (zsh): `cd rust; clear`
  - Terminal 2 (tmux):
    - Split pane vertically with `(Ctrl+shift %)`
    - `cd nodejs; clear` in both panes
  - Terminal 3 (zsh): `cd nodejs; clear`

### Terminal 3

Left pane:

```
npx ts-node ./src/demo-serde-wasm.ts
```

Right pane:

```
- npx ts-node ./src/demo-tsify-wasm.ts
```
