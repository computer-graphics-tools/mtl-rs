"""Step 3: Generate mdBook structure from scraped Apple Metal documentation.

Reads output/metal_docs.json and output/samples_manifest.json.
Output: book/ directory in the repo root + examples/resources/<slug>/ directories.
"""

import json
import re
import shutil
from pathlib import Path

DOCS_JSON = Path(__file__).parent / "output" / "metal_docs.json"
MANIFEST_JSON = Path(__file__).parent / "output" / "samples_manifest.json"
REPO_ROOT = Path(__file__).parent.parent.parent
BOOK_DIR = REPO_ROOT / "book"
BOOK_SRC = BOOK_DIR / "src"
RESOURCES_DIR = REPO_ROOT / "examples" / "resources"


def slug_to_example_name(slug: str) -> str:
    """Convert an Apple doc slug to a Rust example name."""
    return slug.replace("-", "_")


def slug_to_filename(slug: str) -> str:
    """Convert a slug to a markdown filename."""
    return f"{slug}.md"


def sanitize_body(body: str) -> str:
    """Return body markdown as-is (already properly formatted by scrape_docs.py)."""
    return body.strip()


def collect_all_articles(sections):
    """Flatten all articles from the section tree."""
    articles = []
    for section in sections:
        for article in section.get("articles", []):
            articles.append((section["title"], article))
            for child in article.get("child_articles", []):
                articles.append((article["title"], child))
                for grandchild in child.get("child_articles", []):
                    articles.append((child["title"], grandchild))
    return articles


def section_slug(title: str) -> str:
    """Convert a section title to a directory-friendly slug."""
    return re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")


def main():
    if not DOCS_JSON.exists():
        print(f"Error: {DOCS_JSON} not found. Run scrape_docs.py first.")
        return

    with open(DOCS_JSON) as f:
        docs = json.load(f)

    manifest = {}
    if MANIFEST_JSON.exists():
        with open(MANIFEST_JSON) as f:
            manifest = json.load(f)

    # Create book structure
    BOOK_SRC.mkdir(parents=True, exist_ok=True)

    # Write book.toml
    book_toml = BOOK_DIR / "book.toml"
    book_toml.write_text("""[book]
title = "Metal Programming Guide for Rust"
authors = ["Eugene Bokhan"]
language = "en"
src = "src"

[output.html]
git-repository-url = "https://github.com/computer-graphics-tools/mtl-rs"
""")

    # Write introduction
    intro = BOOK_SRC / "introduction.md"
    intro.write_text("""# Metal Programming Guide for Rust

This book mirrors [Apple's Metal documentation](https://developer.apple.com/documentation/metal?language=objc)
with Rust examples using the [mtl-rs](https://github.com/computer-graphics-tools/mtl-rs) crate.

Each chapter corresponds to an Apple Metal documentation article. Where Apple provides
sample code, we provide a Rust port that you can run with `cargo run --example <name>`.

## Getting Started

```bash
git clone https://github.com/computer-graphics-tools/mtl-rs.git
cd mtl-rs
cargo run --example drawing_a_triangle_with_metal_4
```
""")

    # Build SUMMARY.md and article pages
    summary_lines = ["# Summary\n", "[Introduction](introduction.md)\n"]

    for section in docs["sections"]:
        section_title = section["title"]
        sec_slug = section_slug(section_title)
        sec_dir = BOOK_SRC / sec_slug
        sec_dir.mkdir(parents=True, exist_ok=True)

        # Section README
        readme = sec_dir / "README.md"
        readme.write_text(f"# {section_title}\n")
        summary_lines.append(f"\n# {section_title}\n")
        summary_lines.append(f"- [{section_title}]({sec_slug}/README.md)\n")

        for article in section["articles"]:
            slug = article["slug"]
            title = article["title"]
            apple_url = article["url"]
            example_name = slug_to_example_name(slug)
            md_file = slug_to_filename(slug)

            # Write article page
            article_path = sec_dir / md_file
            content = f"# {title}\n\n"
            content += f"> [Apple Documentation]({apple_url})\n\n"

            if article.get("overview"):
                content += f"{article['overview']}\n\n"

            # Check if we have a sample
            has_sample = slug in manifest
            if has_sample:
                sample_info = manifest[slug]
                content += "## Run the Example\n\n"
                content += f"```bash\ncargo run --example {example_name}\n```\n\n"

                if sample_info.get("shader_files"):
                    content += "## Shader Files\n\n"
                    for sf in sample_info["shader_files"]:
                        content += f"- `{sf}`\n"
                    content += "\n"

                if sample_info.get("resource_files"):
                    content += "## Resources\n\n"
                    for rf in sample_info["resource_files"]:
                        content += f"- `{rf}`\n"
                    content += "\n"

            elif article.get("has_sample_code"):
                content += "## Run the Example\n\n"
                content += f"```bash\ncargo run --example {example_name}\n```\n\n"

            # Add article body (already properly formatted markdown)
            body = sanitize_body(article.get("body_markdown", ""))
            if body:
                content += body + "\n"

            article_path.write_text(content)
            summary_lines.append(f"  - [{title}]({sec_slug}/{md_file})\n")

            # Handle child articles
            for child in article.get("child_articles", []):
                child_slug = child["slug"]
                child_title = child["title"]
                child_url = child["url"]
                child_example = slug_to_example_name(child_slug)
                child_md = slug_to_filename(child_slug)

                child_path = sec_dir / child_md
                child_content = f"# {child_title}\n\n"
                child_content += f"> [Apple Documentation]({child_url})\n\n"

                if child.get("overview"):
                    child_content += f"{child['overview']}\n\n"

                child_has_sample = child_slug in manifest
                if child_has_sample:
                    child_content += "## Run the Example\n\n"
                    child_content += (
                        f"```bash\ncargo run --example {child_example}\n```\n\n"
                    )

                child_body = sanitize_body(child.get("body_markdown", ""))
                if child_body:
                    child_content += child_body + "\n"

                child_path.write_text(child_content)
                summary_lines.append(f"    - [{child_title}]({sec_slug}/{child_md})\n")

    # Write SUMMARY.md
    summary_path = BOOK_SRC / "SUMMARY.md"
    summary_path.write_text("".join(summary_lines))

    # Copy resources from downloaded samples
    if manifest:
        for slug, info in manifest.items():
            if info.get("resource_files"):
                sample_dir = Path(__file__).parent / "output" / "samples" / slug
                dest_dir = RESOURCES_DIR / slug
                dest_dir.mkdir(parents=True, exist_ok=True)

                for rf in info["resource_files"]:
                    src = sample_dir / rf
                    dst = dest_dir / Path(rf).name
                    if src.exists() and not dst.exists():
                        shutil.copy2(src, dst)
                        print(f"  Copied resource: {slug}/{Path(rf).name}")

    # Count stats
    md_count = sum(1 for _ in BOOK_SRC.rglob("*.md"))
    print(f"\nDone! Generated mdBook:")
    print(f"  {md_count} markdown files in book/src/")
    print(f"  SUMMARY.md with {len(summary_lines)} entries")
    print(f"  Output: {BOOK_DIR}")


if __name__ == "__main__":
    main()
