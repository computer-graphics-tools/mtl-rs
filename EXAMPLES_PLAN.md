# Metal Sample Code Library — Rust Port Plan

## Context

The goal is to mirror Apple's [Metal documentation](https://developer.apple.com/documentation/metal?language=objc) structure in mtl-rs:

1. **Automated scraper** (`tools/scraper/`) — a uv-based Python project using Playwright to crawl Apple's Metal documentation, extract all article content, discover sample code download links, and download sample projects with all their resources.
2. **mdBook** (`book/`) — article-style documentation generated from the scraped content, mirroring Apple's Metal doc pages with prose adapted for Rust/mtl-rs.
3. **Examples** (`examples/`) — standalone Rust binaries porting every Apple Metal sample project. Resources from downloaded samples are placed under `examples/resources/`.

This approach is sustainable: re-running the scraper picks up new Apple articles and samples automatically rather than maintaining the list by hand.

---

## 1. Scraper — `tools/scraper/`

A uv-managed Python project that crawls Apple's Metal documentation and downloads all sample projects.

### Project structure

```
tools/scraper/
├── pyproject.toml              # uv project: playwright dependency
├── scrape_docs.py              # Step 1: Crawl doc tree, extract articles
├── download_samples.py         # Step 2: Download sample code ZIPs
└── generate_book.py            # Step 3: Generate mdBook from scraped content
```

### Step 1: `scrape_docs.py` — Crawl documentation tree

Uses Playwright (headless Chromium) to handle Apple's JS-rendered pages.

**Input:** Root URL `https://developer.apple.com/documentation/metal?language=objc`

**Process:**
1. Load the root Metal doc page, extract all section headings and child links
2. For each child page (e.g. `render-passes`, `compute-passes`, `metal-sample-code-library`):
   - Navigate to the page
   - Extract: title, overview text, topics list with descriptions
   - For sample code articles: extract the "Download" button URL if present
   - Record parent-child hierarchy
3. For each sample code article (leaf pages like `performing-calculations-on-a-gpu`):
   - Extract: title, full article prose, code snippets, download URL
   - Record which section/category it belongs to

**Output:** `tools/scraper/output/metal_docs.json`
```json
{
  "title": "Metal",
  "url": "https://developer.apple.com/documentation/metal?language=objc",
  "sections": [
    {
      "title": "Essentials",
      "articles": [
        {
          "title": "Performing calculations on a GPU",
          "slug": "performing-calculations-on-a-gpu",
          "url": "https://developer.apple.com/documentation/metal/performing-calculations-on-a-gpu",
          "overview": "Use Metal to find GPUs and perform calculations on them.",
          "body_markdown": "... full article text ...",
          "download_url": "https://docs-assets.developer.apple.com/...",
          "has_sample_code": true,
          "category": "Essentials"
        }
      ]
    }
  ]
}
```

### Step 2: `download_samples.py` — Download sample projects

**Input:** `output/metal_docs.json`

**Process:**
1. For each article with `has_sample_code: true` and a `download_url`:
   - Download the ZIP file
   - Extract into `output/samples/<slug>/`
2. Catalog all resource files per sample:
   - Scan for `.tga`, `.png`, `.jpg`, `.hdr`, `.obj`, `.usdz`, `.mtl` files
   - Record paths relative to sample root

**Output:**
- `output/samples/<slug>/` — extracted sample project directories
- `output/samples_manifest.json` — manifest listing each sample's source files, shader files, and resource files

```json
{
  "performing-calculations-on-a-gpu": {
    "source_files": ["MetalAdder.m", "MetalAdder.h", "main.m"],
    "shader_files": ["add.metal"],
    "resource_files": [],
    "header_files": ["ShaderTypes.h"]
  },
  "creating-and-sampling-textures": {
    "source_files": ["AAPLRenderer.m", "AAPLImage.m", ...],
    "shader_files": ["AAPLShaders.metal"],
    "resource_files": ["Image.tga"],
    "header_files": ["AAPLShaderTypes.h"]
  }
}
```

### Step 3: `generate_book.py` — Generate mdBook structure

**Input:** `output/metal_docs.json`, `output/samples_manifest.json`

**Process:**
1. Create `book/src/SUMMARY.md` from the doc tree hierarchy
2. For each article, generate a `.md` file:
   - Title and link to Apple docs
   - Article prose (from scraped body)
   - If sample code exists: `cargo run --example <name>` command, link to example source
   - List of resources used
3. Copy resource files from downloaded samples into `examples/resources/<slug>/`

**Output:**
- `book/book.toml`
- `book/src/SUMMARY.md`
- `book/src/**/*.md` — all article pages
- `examples/resources/<slug>/` — resource files ready for Rust examples

### Running the scraper

```bash
cd tools/scraper
uv run python scrape_docs.py        # ~2 min: crawl all doc pages
uv run python download_samples.py   # ~5 min: download all sample ZIPs
uv run python generate_book.py      # ~10 sec: generate mdBook + copy resources
```

### Re-running

The scraper is idempotent. Re-running it:
- Updates articles if Apple changes them
- Discovers new samples added to the documentation
- Downloads only new/changed sample ZIPs (checks file sizes)

---

## 2. Documentation — mdBook

Generated by the scraper (Step 3) but can also be edited manually afterward.

### Structure

```
book/
├── book.toml
└── src/
    ├── SUMMARY.md                           # Auto-generated from doc tree
    ├── introduction.md
    ├── essentials/
    │   ├── performing-calculations-on-a-gpu.md
    │   ├── drawing-a-triangle-with-metal-4.md
    │   └── ...
    ├── sample-code-library/
    │   ├── compute-workflows.md
    │   ├── render-workflows.md
    │   ├── textures.md
    │   └── ...
    ├── gpu-devices/
    ├── command-encoders/
    ├── resources/
    ├── shader-compilation/
    ├── presentation/
    ├── developer-tools/
    └── apple-silicon/
```

### book.toml

```toml
[book]
title = "Metal Programming Guide for Rust"
authors = ["Eugene Bokhan"]
language = "en"
src = "src"

[output.html]
git-repository-url = "https://github.com/computer-graphics-tools/mtl-rs"
```

---

## 3. Examples — Full Port of Apple Sample Code Library

### Structure

```
examples/
├── common/
│   └── mod.rs                           # Shared helpers
├── resources/
│   ├── creating-and-sampling-textures/
│   │   └── image.tga
│   ├── rendering-reflections-in-real-time/
│   │   ├── firetruck.obj
│   │   ├── kloppenheim_06_4k.hdr
│   │   └── ...
│   └── ...                              # Per-sample resource dirs
├── performing_calculations_on_a_gpu.rs
├── drawing_a_triangle_with_metal_4.rs
├── creating_and_sampling_textures.rs
└── ...                                  # One .rs per sample
```

### Current status

| # | Example | Status |
|---|---|---|
| 1 | `performing_calculations_on_a_gpu` | Done |
| 2 | `drawing_a_triangle_with_metal_4` | Done |
| 3 | `creating_and_sampling_textures` | Done |
| 4 | `customizing_render_pass_setup` | Done |
| 5 | `calculating_primitive_visibility_using_depth_testing` | Done |
| 6 | `encoding_indirect_command_buffers_on_the_cpu` | Done |
| 7 | `improving_edge_rendering_quality_with_msaa` | Done |
| 8 | `processing_a_texture_in_a_compute_function` | Done |
| 9 | `streaming_large_images_with_metal_sparse_textures` | Done |
| 10 | `managing_groups_of_resources_with_argument_buffers` | Done |
| 11 | `using_argument_buffers_with_resource_heaps` | Done |
| 12 | `encoding_argument_buffers_on_the_gpu` | Done |
| 13 | `creating_a_metal_dynamic_library` | Done |
| 14 | `using_function_specialization_to_build_pipeline_variants` | Done |
| 15 | `synchronizing_cpu_and_gpu_work` | Done |
| 16 | `accelerating_ray_tracing_using_metal` | Done |
| 17 | `capturing_metal_commands_programmatically` | Done |

Remaining samples will be discovered and cataloged by the scraper.

---

## 4. Implementation Phases

### Phase 1: Foundation — Done

- [x] Rename existing examples to Apple naming convention
- [x] Port `performing_calculations_on_a_gpu` (16M element GPU compute)
- [x] Port `creating_and_sampling_textures` (TGA loading + textured quad)
- [x] Port `drawing_a_triangle_with_metal_4` (windowed rotating triangle)
- [x] Copy `Image.tga` to `examples/resources/`
- [x] Fix `Encoding::Struct` names across the library (`"?"` for anonymous C structs)

### Phase 2: Build the scraper

1. Create `tools/scraper/` uv project with Playwright
2. Implement `scrape_docs.py` — crawl Apple Metal doc tree
3. Implement `download_samples.py` — download all sample code ZIPs
4. Implement `generate_book.py` — generate mdBook + copy resources
5. Run the full pipeline, verify output

### Phase 3: Generate mdBook

1. Run `generate_book.py` to create `book/` structure
2. Review and edit generated articles for Rust/mtl-rs context
3. Add `cargo run --example` invocations and code snippets
4. `mdbook build book/` — verify it builds

### Phase 4: Port remaining examples

Using the downloaded sample projects as reference:
1. For each sample in `output/samples/`:
   - Read the ObjC/Swift source and Metal shaders
   - Write the Rust equivalent using mtl-rs bindings
   - Copy resources to `examples/resources/<slug>/`
2. Prioritize no-resource samples first, then resource-heavy ones

### Phase 5: Ongoing maintenance

- Re-run scraper periodically to pick up new Apple articles/samples
- Diff output against previous run to identify changes
- Port new samples as they appear

---

## 5. Verification

1. `cd tools/scraper && uv run python scrape_docs.py` — crawls without errors
2. `cd tools/scraper && uv run python download_samples.py` — downloads all samples
3. `cd tools/scraper && uv run python generate_book.py` — generates book
4. `mdbook build book/` — book builds without errors
5. `cargo build --examples` — all Rust examples compile
6. `cargo run --example drawing_a_triangle_with_metal_4` — windowed rotating triangle
