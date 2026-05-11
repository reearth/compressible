import { describe, expect, it } from "vitest";
import { extensions, isCompressible, isCompressiblePath } from "../src/index.js";

describe("isCompressible", () => {
  it("returns true for known compressible extensions", () => {
    for (const e of ["js", "css", "html", "json", "svg", "wasm", "b3dm", "i3dm", "pnts", "cmpt", "subtree", "terrain", "bin"]) {
      expect(isCompressible(e), e).toBe(true);
    }
  });

  it("returns false for already-compressed / binary formats", () => {
    for (const e of ["gz", "zip", "7z", "jpg", "jpeg", "png", "mp4", "mp3", "svgz", "psd", "vmdk", "tar", "ova"]) {
      expect(isCompressible(e), e).toBe(false);
    }
  });

  it("normalizes case and leading dot", () => {
    for (const e of [".JS", "JS", ".js", "Js", ".Json"]) {
      expect(isCompressible(e), e).toBe(true);
    }
  });

  it("returns false for empty input", () => {
    expect(isCompressible("")).toBe(false);
    expect(isCompressible(".")).toBe(false);
  });
});

describe("isCompressiblePath", () => {
  it("inspects only the last extension", () => {
    expect(isCompressiblePath("foo.js")).toBe(true);
    expect(isCompressiblePath("a/b/c.html")).toBe(true);
    expect(isCompressiblePath("/abs/path/file.JSON")).toBe(true);
    expect(isCompressiblePath("image.png")).toBe(false);
    expect(isCompressiblePath("archive.tar.gz")).toBe(false);
    expect(isCompressiblePath("data.json.gz")).toBe(false);
    expect(isCompressiblePath("noext")).toBe(false);
    expect(isCompressiblePath("")).toBe(false);
    expect(isCompressiblePath(".bashrc")).toBe(false);
  });

  it("handles Windows-style separators", () => {
    expect(isCompressiblePath("C:\\src\\foo.js")).toBe(true);
  });
});

describe("extensions", () => {
  it("is a non-empty set", () => {
    expect(extensions.size).toBeGreaterThan(0);
  });
});
