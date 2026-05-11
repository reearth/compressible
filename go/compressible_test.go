package compressible

import "testing"

func TestExt_Compressible(t *testing.T) {
	cases := []string{
		"js", "css", "html", "json", "svg", "wasm",
		"b3dm", "i3dm", "pnts", "cmpt", "subtree", "terrain", "bin",
	}
	for _, c := range cases {
		if !Ext(c) {
			t.Errorf("Ext(%q) = false, want true", c)
		}
	}
}

func TestExt_NotCompressible(t *testing.T) {
	cases := []string{
		"gz", "zip", "7z", "jpg", "jpeg", "png", "mp4", "mp3",
		"svgz", "psd", "vmdk", "tar", "ova",
	}
	for _, c := range cases {
		if Ext(c) {
			t.Errorf("Ext(%q) = true, want false", c)
		}
	}
}

func TestExt_Normalization(t *testing.T) {
	cases := []string{".JS", "JS", ".js", "js", "Js", ".Json"}
	for _, c := range cases {
		if !Ext(c) {
			t.Errorf("Ext(%q) = false, want true", c)
		}
	}
}

func TestExt_Empty(t *testing.T) {
	if Ext("") {
		t.Error("Ext(\"\") = true, want false")
	}
	if Ext(".") {
		t.Error("Ext(\".\") = true, want false")
	}
}

func TestPath(t *testing.T) {
	tt := []struct {
		in   string
		want bool
	}{
		{"foo.js", true},
		{"a/b/c.html", true},
		{"/abs/path/file.JSON", true},
		{"image.png", false},
		// Only the last extension matters.
		{"archive.tar.gz", false},
		{"data.json.gz", false},
		{"noext", false},
		{"", false},
	}
	for _, tc := range tt {
		if got := Path(tc.in); got != tc.want {
			t.Errorf("Path(%q) = %v, want %v", tc.in, got, tc.want)
		}
	}
}

func TestExtensions_SortedAndCopy(t *testing.T) {
	a := Extensions()
	if len(a) == 0 {
		t.Fatal("Extensions() empty")
	}
	for i := 1; i < len(a); i++ {
		if a[i-1] >= a[i] {
			t.Fatalf("Extensions() not sorted at %d: %q >= %q", i, a[i-1], a[i])
		}
	}
	// Mutating the result must not affect subsequent calls.
	a[0] = "ZZZZ"
	b := Extensions()
	if b[0] == "ZZZZ" {
		t.Error("Extensions() returned shared slice")
	}
}
