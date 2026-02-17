"""Step 2: Download Apple Metal sample code projects.

Reads output/metal_docs.json and downloads all sample code ZIPs.
Output: output/samples/<slug>/ directories + output/samples_manifest.json
"""

import json
import os
import shutil
import urllib.request
import zipfile
from pathlib import Path

DOCS_JSON = Path(__file__).parent / "output" / "metal_docs.json"
SAMPLES_DIR = Path(__file__).parent / "output" / "samples"
MANIFEST_PATH = Path(__file__).parent / "output" / "samples_manifest.json"

RESOURCE_EXTENSIONS = {
    ".tga",
    ".png",
    ".jpg",
    ".jpeg",
    ".hdr",
    ".exr",
    ".obj",
    ".mtl",
    ".usdz",
    ".usd",
    ".fbx",
    ".gltf",
    ".glb",
    ".dae",
    ".stl",
    ".ply",
    ".ktx",
}
SHADER_EXTENSIONS = {".metal"}
SOURCE_EXTENSIONS = {".m", ".mm", ".swift", ".c", ".cpp"}
HEADER_EXTENSIONS = {".h", ".hpp"}


def collect_articles(sections):
    """Recursively collect all articles from the doc tree."""
    articles = []
    for section in sections:
        for article in section.get("articles", []):
            articles.append(article)
            # Recurse into child articles
            if article.get("child_articles"):
                articles.extend(collect_articles_flat(article["child_articles"]))
    return articles


def collect_articles_flat(article_list):
    """Flatten a list of articles including children."""
    result = []
    for article in article_list:
        result.append(article)
        if article.get("child_articles"):
            result.extend(collect_articles_flat(article["child_articles"]))
    return result


def catalog_sample(sample_dir: Path) -> dict:
    """Catalog the files in a downloaded sample project."""
    result = {
        "source_files": [],
        "shader_files": [],
        "resource_files": [],
        "header_files": [],
        "all_files": [],
    }

    for root, dirs, files in os.walk(sample_dir):
        # Skip hidden dirs, build artifacts, xcodeproj internals
        dirs[:] = [
            d
            for d in dirs
            if not d.startswith(".")
            and d != "build"
            and not d.endswith(".xcodeproj")
            and not d.endswith(".xcworkspace")
        ]

        rel_root = Path(root).relative_to(sample_dir)
        for f in files:
            if f.startswith("."):
                continue
            ext = Path(f).suffix.lower()
            rel_path = str(rel_root / f)
            result["all_files"].append(rel_path)

            if ext in SOURCE_EXTENSIONS:
                result["source_files"].append(rel_path)
            elif ext in HEADER_EXTENSIONS:
                result["header_files"].append(rel_path)
            elif ext in SHADER_EXTENSIONS:
                result["shader_files"].append(rel_path)
            elif ext in RESOURCE_EXTENSIONS:
                result["resource_files"].append(rel_path)

    return result


def download_sample(url: str, slug: str) -> bool:
    """Download and extract a sample code ZIP."""
    dest = SAMPLES_DIR / slug
    if dest.exists() and any(dest.iterdir()):
        print(f"    Already downloaded, skipping")
        return True

    dest.mkdir(parents=True, exist_ok=True)
    zip_path = SAMPLES_DIR / f"{slug}.zip"

    try:
        print(f"    Downloading...")
        urllib.request.urlretrieve(url, zip_path)

        print(f"    Extracting...")
        with zipfile.ZipFile(zip_path) as zf:
            zf.extractall(dest)

        # If the ZIP extracted into a single subdirectory, move contents up
        entries = list(dest.iterdir())
        non_zip = [e for e in entries if e.suffix != ".zip"]
        if len(non_zip) == 1 and non_zip[0].is_dir():
            subdir = non_zip[0]
            for item in subdir.iterdir():
                shutil.move(str(item), str(dest / item.name))
            subdir.rmdir()

        zip_path.unlink()
        return True

    except Exception as e:
        print(f"    ERROR downloading: {e}")
        if zip_path.exists():
            zip_path.unlink()
        return False


def main():
    if not DOCS_JSON.exists():
        print(f"Error: {DOCS_JSON} not found. Run scrape_docs.py first.")
        return

    with open(DOCS_JSON) as f:
        docs = json.load(f)

    articles = collect_articles(docs["sections"])
    samples = [a for a in articles if a.get("download_url")]

    print(f"Found {len(samples)} articles with sample code downloads")
    SAMPLES_DIR.mkdir(parents=True, exist_ok=True)

    manifest = {}
    for i, article in enumerate(samples):
        slug = article["slug"]
        url = article["download_url"]
        print(f"[{i + 1}/{len(samples)}] {slug}")
        print(f"    URL: {url}")

        if download_sample(url, slug):
            sample_dir = SAMPLES_DIR / slug
            catalog = catalog_sample(sample_dir)
            catalog["title"] = article["title"]
            catalog["apple_url"] = article["url"]
            catalog["download_url"] = url
            manifest[slug] = catalog

            src_count = len(catalog["source_files"])
            shader_count = len(catalog["shader_files"])
            resource_count = len(catalog["resource_files"])
            print(
                f"    Files: {src_count} source, {shader_count} shader, {resource_count} resource"
            )

            # Write article content as README.md in the sample directory
            body_md = article.get("body_markdown", "")
            if body_md:
                overview = article.get("overview", "")
                readme = f"# {article['title']}\n\n"
                if overview:
                    readme += f"{overview}\n\n"
                readme += f"> Source: [Apple Documentation]({article['url']})\n\n"
                readme += body_md + "\n"

                readme_path = sample_dir / "README.md"
                readme_path.write_text(readme)
                print(f"    Wrote README.md ({len(readme)} chars)")

    with open(MANIFEST_PATH, "w") as f:
        json.dump(manifest, f, indent=2)

    print(f"\nDone! Downloaded {len(manifest)} samples")
    print(f"Manifest: {MANIFEST_PATH}")


if __name__ == "__main__":
    main()
