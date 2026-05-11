# @reearth/compressible

Tells you whether a file's contents (identified by its extension) are worth
gzip-compressing — for example before uploading to a CDN or object store with
`Content-Encoding: gzip`.

The extension list (~230 entries) is curated from [jshttp/mime-db]:
already-compressed and large-binary formats are dropped, and 3D Tiles /
Cesium quantized-mesh terrain extensions common in geospatial pipelines are
added in.

## Install

```bash
npm install @reearth/compressible
```

ESM and CJS are both shipped, with type definitions.

## Usage

```ts
import {
  isCompressible,
  isCompressiblePath,
  extensions,
} from "@reearth/compressible";

isCompressible("js");                // true
isCompressible(".JSON");             // true — leading dot and case are normalized
isCompressible("png");               // false

isCompressiblePath("a/b/c.html");    // true
isCompressiblePath("img.png");       // false
isCompressiblePath("archive.tar.gz") // false — only the last extension is considered

// Raw access to the underlying set if you need it.
extensions.has("wasm");              // true
```

## Behavior

- The extension may be supplied with or without a leading dot.
- Matching is case-insensitive.
- For paths, **only the final extension** is considered:
  `archive.tar.gz` → `gz` → not compressible.
- An empty input returns `false`.

## Why not just use mime-db's `compressible` flag?

`mime-db` ships some `compressible: true` entries that aren't actually worth
gzipping in practice (VM disk images, DXT-compressed textures, PSDs with
internal compression). It also misses formats common in 3D / geospatial
pipelines (3D Tiles `.b3dm` / `.i3dm` / `.pnts` / `.cmpt` / `.subtree`,
Cesium quantized-mesh `.terrain`).

This library bakes that curation in once so it doesn't have to be
re-implemented in every uploader / CDN tool.

## Provenance

Releases are published from GitHub Actions with [npm provenance][provenance],
so you can verify which commit and workflow run produced each tarball:

```bash
npm view @reearth/compressible
```

## License

MIT — see the [repository][repo] for details.

Also available for [Go][go-pkg].

[jshttp/mime-db]: https://github.com/jshttp/mime-db
[provenance]: https://docs.npmjs.com/generating-provenance-statements
[repo]: https://github.com/reearth/compressible
[go-pkg]: https://pkg.go.dev/github.com/reearth/compressible/go
