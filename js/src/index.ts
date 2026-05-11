// Tells you whether a file extension is worth gzip-compressing.
// The extension set is derived from jshttp/mime-db with curation for formats
// that are already-compressed or that don't benefit from gzip in practice.
// See scripts/gen.go for the generation logic.

import { extensions as _extensions } from "./extensions.js";

/** Set of known compressible extensions (lowercased, no leading dot). */
export const extensions: ReadonlySet<string> = _extensions;

/**
 * Returns true if the given file extension benefits from gzip compression.
 * Accepts the extension with or without a leading dot, in any case.
 */
export function isCompressible(ext: string): boolean {
  const e = normalize(ext);
  if (!e) return false;
  return _extensions.has(e);
}

/**
 * Returns true if the file at `p` benefits from gzip compression, based on
 * its extension. Only the final extension is considered: for "foo.tar.gz",
 * the extension is "gz".
 */
export function isCompressiblePath(p: string): boolean {
  return isCompressible(extOf(p));
}

function normalize(ext: string): string {
  if (!ext) return "";
  if (ext.startsWith(".")) ext = ext.slice(1);
  return ext.toLowerCase();
}

function extOf(p: string): string {
  if (!p) return "";
  // Strip directory components.
  const slash = Math.max(p.lastIndexOf("/"), p.lastIndexOf("\\"));
  const base = slash >= 0 ? p.slice(slash + 1) : p;
  const dot = base.lastIndexOf(".");
  if (dot <= 0) return ""; // no dot, or leading-dot dotfile like ".bashrc"
  return base.slice(dot + 1);
}
