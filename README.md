# compressible

Tells you whether a file's contents (identified by its extension) are worth
gzip-compressing — for example before uploading to a CDN or object store with
`Content-Encoding: gzip`.

Available as a library in two languages, sharing a single curated extension
list (~230 entries) generated from [jshttp/mime-db]:

| Language          | Package                                          |
| ----------------- | ------------------------------------------------ |
| Go                | `github.com/reearth/compressible`                |
| TypeScript / JS   | `@reearth/compressible` (ESM + CJS)              |

## Why not just use mime-db's `compressible` flag?

`mime-db` is a great starting point but ships some `compressible: true`
entries that aren't actually worth gzipping in practice (e.g. VM disk images,
DXT-compressed textures, PSDs with internal compression). It also misses
formats common in 3D / geospatial pipelines (3D Tiles `.b3dm` / `.i3dm` /
`.pnts` / `.cmpt` / `.subtree`, Cesium quantized-mesh `.terrain`).

This library bakes that curation in once so it doesn't have to be
re-implemented in every uploader / CDN tool.

## Install & use

### Go

```bash
go get github.com/reearth/compressible
```

```go
import "github.com/reearth/compressible"

compressible.Ext("js")          // true
compressible.Ext(".JSON")       // true
compressible.Path("a/b/c.html") // true
compressible.Path("img.png")    // false
compressible.Path("archive.tar.gz") // false (only the last ext is considered)
```

### TypeScript / JavaScript

```bash
npm install @reearth/compressible
```

```ts
import { isCompressible, isCompressiblePath, extensions } from "@reearth/compressible";

isCompressible("js");            // true
isCompressible(".JSON");         // true
isCompressiblePath("a/b/c.html") // true
isCompressiblePath("img.png");   // false
```

## Behavior

- The extension may be supplied with or without a leading dot.
- Matching is case-insensitive (lowercased internally).
- For paths, **only the final extension** is considered:
  `archive.tar.gz` → `gz` → not compressible.
- An empty input returns `false`.

## Updating the extension list

The list is regenerated from upstream `mime-db` by a single script at the repo
root, which writes language-specific files into each package:

```bash
go run scripts/gen.go
```

This rewrites:

- `go/extensions.go`
- `js/src/extensions.ts`

CI re-runs the generator and fails on a diff, so the lists stay in sync and
follow upstream `mime-db` updates.

## Curation rules

Encoded in `scripts/gen.go`:

- **Excluded MIME types** even if `compressible: true` upstream:
  `application/octet-stream`, `application/x-tar`, all `application/x-virtualbox-*`,
  `image/vnd.ms-dds`, `image/vnd.adobe.photoshop`.
- **Excluded extensions** (pre-compressed siblings):
  `svgz`, `x3dz`.
- **Extra extensions** added beyond `mime-db`:
  `bin`, 3D Tiles (`b3dm`, `i3dm`, `pnts`, `cmpt`, `subtree`),
  Cesium terrain (`terrain`).

## License

MIT — see [LICENSE](./LICENSE).

[jshttp/mime-db]: https://github.com/jshttp/mime-db
