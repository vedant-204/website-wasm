# website-p

Personal site — an n-body orbital map of my work, in Rust + WebAssembly.

## Run it

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk

trunk serve          # http://127.0.0.1:8080
```

## Ship it

```bash
trunk build --release
ls -lh dist/          # keep the .wasm under ~400 KB gzipped
```

Deploy `dist/` to Cloudflare Pages (free, static, no server):

```bash
npx wrangler pages deploy dist --project-name orbital
```

See `CLAUDE.md` for architecture and the rules that keep the concept intact.
