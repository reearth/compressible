// Package compressible reports whether a file's contents (identified by its
// extension) are worth gzip-compressing — for example before uploading to a
// CDN or object store with Content-Encoding: gzip.
//
// The extension set is derived from jshttp/mime-db with curation for formats
// that are already-compressed or that don't benefit from gzip in practice.
// See scripts/gen.go for the generation logic.
package compressible

import (
	"path"
	"sort"
	"strings"
)

// Ext reports whether the given file extension benefits from gzip
// compression. The extension may be supplied with or without a leading dot,
// and is matched case-insensitively. An empty string returns false.
func Ext(ext string) bool {
	ext = normalize(ext)
	if ext == "" {
		return false
	}
	_, ok := exts[ext]
	return ok
}

// Path reports whether the file at p benefits from gzip compression, based on
// its extension. Only the final extension is considered: for "foo.tar.gz",
// the extension is "gz".
func Path(p string) bool {
	return Ext(path.Ext(p))
}

// Extensions returns a sorted copy of the known compressible extensions
// (lowercased, without leading dot).
func Extensions() []string {
	out := make([]string, 0, len(exts))
	for e := range exts {
		out = append(out, e)
	}
	sort.Strings(out)
	return out
}

func normalize(ext string) string {
	ext = strings.TrimPrefix(ext, ".")
	return strings.ToLower(ext)
}
