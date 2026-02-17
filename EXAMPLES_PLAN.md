# Metal Sample Code Library — Rust Port Plan

## Context

The goal is to mirror Apple's [Metal documentation](https://developer.apple.com/documentation/metal?language=objc) structure in mtl-rs:

1. **Automated scraper** (`tools/scraper/`) — a uv-based Python project using Playwright to crawl Apple's Metal documentation, extract article content as properly formatted markdown, discover sample code download links, and download sample projects with all their resources.
2. **mdBook** (`book/`) — article-style documentation generated from the scraped content, mirroring Apple's Metal doc pages with prose adapted for Rust/mtl-rs.
3. **Examples** (`examples/`) — standalone Rust binaries porting every Apple Metal sample project. Resources from downloaded samples are placed under `examples/resources/`.

This approach is sustainable: re-running the scraper picks up new Apple articles and samples automatically rather than maintaining the list by hand.

### Current statistics

- 489 total articles in Apple's Metal documentation
- **45 articles have sample code** with download URLs — these are the focus
- 48 sample projects downloaded and extracted in `output/samples/`
- All 48 samples include Apple-provided README.md files
- 10 sections contain sample-code articles: Essentials (7), GPU devices (1), Command encoders (9), Textures (9), Presentation (3), Developer tools (2), Compute workflows (3), Render workflows (7), Argument buffers (3), Lighting techniques (1)

---

## 1. Scraper — `tools/scraper/`

A uv-managed Python project that crawls Apple's Metal documentation and downloads all sample projects.

### Project structure

```
tools/scraper/
├── pyproject.toml              # uv project: playwright, markdownify dependencies
├── scrape_docs.py              # Step 1: Crawl doc tree, extract articles as markdown
├── download_samples.py         # Step 2: Download sample code ZIPs, write article READMEs
└── generate_book.py            # Step 3: Generate mdBook from scraped content
```

### Step 1: `scrape_docs.py` — Crawl documentation tree

Uses Playwright (headless Chromium) to handle Apple's JS-rendered pages. Uses `markdownify` with custom converters to transform Apple's HTML into well-formatted markdown.

**Input:** Root URL `https://developer.apple.com/documentation/metal?language=objc`

**Scope:** Only articles with sample code (~45 articles, not all 489).

**Process:**
1. Load the root Metal doc page + sample code library page, extract all section headings and child links
2. Visit each article page:
   - Detect download URL presence (sample code indicator)
   - For sample-code articles only: extract full HTML from `.primary-content` and `.documentation-hero__content .abstract`
   - Convert HTML to markdown using custom `markdownify.MarkdownConverter` subclass
   - Extract: title (from `<h1>`), abstract, formatted body, download URL
3. Skip articles without sample code (no body extraction needed)

**HTML → Markdown conversion:**

Apple's documentation uses Vue.js-rendered pages with specific HTML patterns. The custom converter handles:

| HTML Pattern | Markdown Output |
|---|---|
| `<h2 id="overview"><a ...>Overview<svg...></a></h2>` | `## Overview` |
| `<h3 id="Find-a-GPU"><a ...>Find a GPU<svg...></a></h3>` | `### Find a GPU` |
| `<p>text with <em>emphasis</em> and <code>code</code></p>` | `text with *emphasis* and `` `code` `` |
| `<div class="code-listing"><pre><code data-syntax="c">...</code></pre></div>` | Fenced code block with language hint |
| `<div class="TabNavigator">` with `.tabnav-link` filename | Bold filename label before code block |
| `<code>` containing `<wbr>` tags | Strip `<wbr>`, wrap in backticks |
| `<ul><li><p>text</p></li></ul>` | `- text` |
| `<a class="inline-link" href="/documentation/metal/...">` | `[text](https://developer.apple.com/documentation/metal/...)` |
| `<aside>` / note blocks | `> **Note:** text` |
| `<img src="..." alt="...">` | `![alt](url)` with absolute URLs |

**Output:** `tools/scraper/output/metal_docs.json`
```json
{
  "title": "Metal",
  "url": "https://developer.apple.com/documentation/metal?language=objc",
  "scraped_at": "2026-02-16T19:36:38Z",
  "sections": [
    {
      "title": "Essentials",
      "articles": [
        {
          "title": "Performing calculations on a GPU",
          "slug": "performing-calculations-on-a-gpu",
          "url": "https://developer.apple.com/documentation/metal/performing-calculations-on-a-gpu?language=objc",
          "overview": "Use Metal to find GPUs and perform calculations on them.",
          "body_markdown": "## Overview\n\nThis sample demonstrates...\n\n### Write a GPU function\n\n```c\nvoid add_arrays(...)\n```\n...",
          "download_url": "https://docs-assets.developer.apple.com/...",
          "has_sample_code": true
        }
      ]
    }
  ],
  "total_articles": 45,
  "total_samples": 45
}
```

### Step 2: `download_samples.py` — Download sample projects & write READMEs

**Input:** `output/metal_docs.json`

**Process:**
1. For each article with `has_sample_code: true` and a `download_url`:
   - Download the ZIP file
   - Extract into `output/samples/<slug>/`
   - Unwrap single top-level directory if present
2. Catalog all files per sample by type
3. **Write article README.md** into each sample directory containing the properly formatted article content

**Output:**
- `output/samples/<slug>/` — extracted sample project directories with article README.md
- `output/samples_manifest.json` — manifest listing each sample's files

```json
{
  "performing-calculations-on-a-gpu": {
    "title": "Performing calculations on a GPU",
    "apple_url": "https://developer.apple.com/documentation/metal/performing-calculations-on-a-gpu?language=objc",
    "download_url": "https://docs-assets.developer.apple.com/...",
    "source_files": ["MetalAdder.m", "main.m"],
    "shader_files": ["add.metal"],
    "resource_files": [],
    "header_files": ["MetalAdder.h"]
  }
}
```

### Step 3: `generate_book.py` — Generate mdBook structure

**Input:** `output/metal_docs.json`, `output/samples_manifest.json`

**Process:**
1. Create `book/src/SUMMARY.md` from the doc tree hierarchy
2. For each article, generate a `.md` file:
   - Title and link to Apple docs
   - Article body markdown (properly formatted, used directly)
   - If sample code exists: `cargo run --example <name>` command
   - List of shader files and resources
3. Copy resource files from downloaded samples into `examples/resources/<slug>/`

**Output:**
- `book/book.toml`
- `book/src/SUMMARY.md`
- `book/src/**/*.md` — all article pages
- `examples/resources/<slug>/` — resource files ready for Rust examples

### Running the scraper

```bash
cd tools/scraper
uv run python scrape_docs.py        # ~1 min: crawl sample-code article pages only
uv run python download_samples.py   # ~5 min: download all sample ZIPs
uv run python generate_book.py      # ~10 sec: generate mdBook + copy resources
```

### Re-running

The scraper is idempotent. Re-running it:
- Updates articles if Apple changes them
- Discovers new samples added to the documentation
- Downloads only new/changed sample ZIPs (skips existing)

---

## 2. Documentation — mdBook

Generated by the scraper (Step 3) but can also be edited manually afterward.

### Structure

```
book/
├── book.toml
└── src/
    ├── SUMMARY.md
    ├── introduction.md
    ├── essentials/
    │   ├── performing-calculations-on-a-gpu.md
    │   ├── drawing-a-triangle-with-metal-4.md
    │   └── ...
    ├── gpu-devices/
    ├── command-encoders/
    ├── textures/
    ├── presentation/
    ├── developer-tools/
    ├── compute-workflows/
    ├── render-workflows/
    ├── argument-buffers/
    └── lighting-techniques/
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

### Complete sample inventory

#### Essentials (7 samples)

| # | Example | Sources | Shaders | Resources | Status |
|---|---|---|---|---|---|
| 1 | `drawing_a_triangle_with_metal_4` | 12 | 1 | 2 | Done |
| 2 | `performing_calculations_on_a_gpu` | 2 | 1 | 2 | Done |
| 3 | `using_metal_to_draw_a_views_contents` | 4 | 0 | 0 | TODO |
| 4 | `processing_a_texture_in_a_compute_function` | 5 | 1 | 2 | Done |
| 5 | `customizing_shaders_using_function_pointers_and_stitching` | — | — | — | TODO |
| 6 | `creating_a_metal_dynamic_library` | 7 | 3 | 1 | Done |
| 7 | `combining_blit_and_compute_operations_in_a_single_pass` | 5 | 1 | 2 | TODO |

#### GPU devices (1 sample)

| # | Example | Sources | Shaders | Resources | Status |
|---|---|---|---|---|---|
| 8 | `detecting_gpu_features_and_metal_software_versions` | — | — | — | TODO |

#### Command encoders (9 samples)

| # | Example | Sources | Shaders | Resources | Status |
|---|---|---|---|---|---|
| 9 | `calculating_primitive_visibility_using_depth_testing` | 4 | 1 | 3 | Done |
| 10 | `rendering_to_multiple_viewports_in_a_draw_command` | — | — | — | TODO |
| 11 | `customizing_render_pass_setup` | 4 | 1 | 4 | Done |
| 12 | `encoding_indirect_command_buffers_on_the_cpu` | 4 | 1 | 3 | Done |
| 13 | `encoding_indirect_command_buffers_on_the_gpu` | 4 | 1 | 2 | TODO |
| 14 | `accelerating_ray_tracing_using_metal` | 6 | 1 | 0 | Done |
| 15 | `control_the_ray_tracing_process_using_intersection_queries` | 6 | 1 | 0 | TODO |
| 16 | `rendering_reflections_in_real_time_using_ray_tracing` | 8 | 1 | 72 | TODO |
| 17 | `rendering_a_curve_primitive_in_a_ray_tracing_scene` | 6 | 1 | 0 | TODO |

#### Textures (9 samples)

| # | Example | Sources | Shaders | Resources | Status |
|---|---|---|---|---|---|
| 18 | `reading_pixel_data_from_a_drawable_texture` | 5 | 1 | 0 | TODO |
| 19 | `creating_and_sampling_textures` | 5 | 1 | 3 | Done |
| 20 | `streaming_large_images_with_metal_sparse_textures` | 7 | 1 | 12 | Done |
| 21 | `using_argument_buffers_with_resource_heaps` | 4 | 1 | 33 | Done |
| 22 | `implementing_a_multistage_image_filter_using_heaps_and_events` | 6 | 2 | 10 | TODO |
| 23 | `implementing_a_multistage_image_filter_using_heaps_and_fences` | 5 | 2 | 11 | TODO |
| 24 | `synchronizing_cpu_and_gpu_work` | 5 | 1 | 3 | Done |
| 25 | `mtlfence` | — | — | — | TODO |
| 26 | `using_function_specialization_to_build_pipeline_variants` | 6 | 1 | 68 | Done |

#### Presentation (3 samples)

| # | Example | Sources | Shaders | Resources | Status |
|---|---|---|---|---|---|
| 27 | `creating_a_custom_metal_view` | 7 | 1 | 0 | TODO |
| 28 | `achieving_smooth_frame_rates_with_a_metal_display_link` | 10 | 1 | 1 | TODO |
| 29 | `processing_hdr_images_with_metal` | 9 | 1 | 1 | TODO |

#### Developer tools (2 samples)

| # | Example | Sources | Shaders | Resources | Status |
|---|---|---|---|---|---|
| 30 | `supporting_simulator_in_a_metal_app` | 9 | 2 | 2 | TODO |
| 31 | `capturing_metal_commands_programmatically` | 4 | 1 | 1 | Done |

#### Compute workflows (3 samples)

| # | Example | Sources | Shaders | Resources | Status |
|---|---|---|---|---|---|
| 32 | `selecting_device_objects_for_compute_processing` | 5 | 2 | 4 | TODO |
| 33 | `customizing_a_tensorflow_operation` | 0 | 1 | 0 | TODO |
| 34 | `customizing_a_pytorch_operation` | 1 | 0 | 0 | TODO |

#### Render workflows (7 samples)

| # | Example | Sources | Shaders | Resources | Status |
|---|---|---|---|---|---|
| 35 | `selecting_device_objects_for_graphics_rendering` | 4 | 1 | 5 | TODO |
| 36 | `implementing_order_independent_transparency_with_image_blocks` | 6 | 1 | 3 | TODO |
| 37 | `loading_textures_and_models_using_metal_fast_resource_loading` | 5 | 1 | 12 | TODO |
| 38 | `adjusting_the_level_of_detail_using_metal_mesh_shaders` | 5 | 1 | 1 | TODO |
| 39 | `creating_a_3d_application_with_hydra_rendering` | 5 | 1 | 0 | TODO |
| 40 | `culling_occluded_geometry_using_the_visibility_result_buffer` | 4 | 1 | 2 | TODO |
| 41 | `improving_edge_rendering_quality_with_msaa` | 5 | 4 | 2 | Done |

#### Argument buffers (3 samples)

| # | Example | Sources | Shaders | Resources | Status |
|---|---|---|---|---|---|
| 42 | `managing_groups_of_resources_with_argument_buffers` | 4 | 1 | 5 | Done |
| 43 | `encoding_argument_buffers_on_the_gpu` | 4 | 1 | 69 | Done |
| 44 | `rendering_terrain_dynamically_with_argument_buffers` | 12 | 7 | 35 | TODO |

#### Lighting techniques (1 sample)

| # | Example | Sources | Shaders | Resources | Status |
|---|---|---|---|---|---|
| 45 | `rendering_a_scene_with_forward_plus_lighting_using_tile_shaders` | 6 | 4 | 8 | TODO |

**Summary:** 17 of 45 examples ported. 28 remaining.

---

## 4. Implementation Phases

### Phase 1: Foundation — Done

- [x] Rename existing examples to Apple naming convention
- [x] Port `performing_calculations_on_a_gpu` (16M element GPU compute)
- [x] Port `creating_and_sampling_textures` (TGA loading + textured quad)
- [x] Port `drawing_a_triangle_with_metal_4` (windowed rotating triangle)
- [x] Copy `Image.tga` to `examples/resources/`
- [x] Fix `Encoding::Struct` names across the library (`"?"` for anonymous C structs)

### Phase 2: Build the scraper — In Progress

- [x] Create `tools/scraper/` uv project with Playwright
- [x] Implement `scrape_docs.py` — crawl Apple Metal doc tree (basic version)
- [x] Implement `download_samples.py` — download all sample code ZIPs
- [x] Implement `generate_book.py` — generate mdBook + copy resources
- [ ] Fix `scrape_docs.py` — replace `innerText` with HTML→markdown conversion using `markdownify`
- [ ] Add `markdownify` dependency to `pyproject.toml`
- [ ] Filter scraping to sample-code-only articles (~45 instead of 489)
- [ ] Write article content as README.md in each sample directory
- [ ] Run the full pipeline, verify markdown quality

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

1. `cd tools/scraper && uv run python scrape_docs.py` — crawls without errors, ~45 articles with proper markdown
2. `cd tools/scraper && uv run python download_samples.py` — downloads all samples, writes article READMEs
3. `cd tools/scraper && uv run python generate_book.py` — generates book with formatted content
4. `mdbook build book/` — book builds without errors
5. `cargo build --examples` — all Rust examples compile
6. `cargo run --example drawing_a_triangle_with_metal_4` — windowed rotating triangle
