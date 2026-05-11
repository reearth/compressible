//! Tells you whether a file extension is worth gzip-compressing — for example
//! before uploading to a CDN or object store with `Content-Encoding: gzip`.
//!
//! The extension set is derived from [jshttp/mime-db] with curation for
//! formats that are already-compressed or that don't benefit from gzip in
//! practice. See `scripts/gen.go` in the repository for the generation logic.
//!
//! [jshttp/mime-db]: https://github.com/jshttp/mime-db

mod extensions;

use extensions::EXTENSIONS;

/// Returns `true` if the given file extension benefits from gzip compression.
///
/// The extension may be supplied with or without a leading dot, and is matched
/// case-insensitively. An empty string returns `false`.
pub fn is_compressible(ext: &str) -> bool {
    let n = normalize(ext);
    if n.is_empty() {
        return false;
    }
    EXTENSIONS.binary_search(&n.as_str()).is_ok()
}

/// Returns `true` if the file at `p` benefits from gzip compression, based on
/// its extension. Only the final extension is considered: for `"foo.tar.gz"`,
/// the extension is `"gz"`.
pub fn is_compressible_path(p: &str) -> bool {
    is_compressible(ext_of(p))
}

/// Iterator over all known compressible extensions (lowercased, no leading
/// dot). Yielded in sorted order.
pub fn extensions() -> impl Iterator<Item = &'static str> {
    EXTENSIONS.iter().copied()
}

fn normalize(ext: &str) -> String {
    let trimmed = ext.strip_prefix('.').unwrap_or(ext);
    trimmed.to_ascii_lowercase()
}

fn ext_of(p: &str) -> &str {
    if p.is_empty() {
        return "";
    }
    let slash = p.rfind(['/', '\\']).map(|i| i + 1).unwrap_or(0);
    let base = &p[slash..];
    match base.rfind('.') {
        Some(0) | None => "",
        Some(i) => &base[i + 1..],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_compressible() {
        for e in [
            "js", "css", "html", "json", "svg", "wasm", "b3dm", "i3dm", "pnts", "cmpt", "subtree",
            "terrain", "bin",
        ] {
            assert!(is_compressible(e), "expected {e} to be compressible");
        }
    }

    #[test]
    fn known_not_compressible() {
        for e in [
            "gz", "zip", "7z", "jpg", "jpeg", "png", "mp4", "mp3", "svgz", "psd", "vmdk", "tar",
            "ova",
        ] {
            assert!(!is_compressible(e), "expected {e} to be not compressible");
        }
    }

    #[test]
    fn normalization() {
        for e in [".JS", "JS", ".js", "Js", ".Json"] {
            assert!(is_compressible(e), "expected {e} to be compressible");
        }
    }

    #[test]
    fn empty() {
        assert!(!is_compressible(""));
        assert!(!is_compressible("."));
    }

    #[test]
    fn paths() {
        assert!(is_compressible_path("foo.js"));
        assert!(is_compressible_path("a/b/c.html"));
        assert!(is_compressible_path("/abs/path/file.JSON"));
        assert!(!is_compressible_path("image.png"));
        assert!(!is_compressible_path("archive.tar.gz"));
        assert!(!is_compressible_path("data.json.gz"));
        assert!(!is_compressible_path("noext"));
        assert!(!is_compressible_path(""));
        assert!(!is_compressible_path(".bashrc"));
        assert!(is_compressible_path("C:\\src\\foo.js"));
    }

    #[test]
    fn extensions_sorted_and_unique() {
        let v: Vec<&str> = extensions().collect();
        assert!(!v.is_empty());
        for w in v.windows(2) {
            assert!(w[0] < w[1], "not sorted/unique: {} vs {}", w[0], w[1]);
        }
    }
}
