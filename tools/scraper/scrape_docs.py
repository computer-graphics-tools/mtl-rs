"""Step 1: Crawl Apple's Metal documentation tree and extract article content.

Only scrapes article/guide pages (not API reference types like MTLDevice).
Output: output/metal_docs.json
"""

import json
import time
from pathlib import Path
from playwright.sync_api import sync_playwright, Page

ROOT_URL = "https://developer.apple.com/documentation/metal?language=objc"
LANG = "?language=objc"
OUTPUT = Path(__file__).parent / "output" / "metal_docs.json"


def slug_from_url(url: str) -> str:
    path = url.split("/documentation/metal/")[-1]
    return path.split("?")[0].split("#")[0].rstrip("/")


def is_api_reference(slug: str) -> bool:
    """API reference pages start with 'mtl' (lowercase) and have no dashes."""
    lower = slug.lower()
    return (lower.startswith("mtl") and "-" not in slug) or "/" in slug


def extract_links_and_text(page: Page) -> dict:
    page.wait_for_timeout(4000)
    return page.evaluate("""
    () => {
        const links = [];
        const seen = new Set();
        document.querySelectorAll('a').forEach(a => {
            if (a.href && a.href.includes('/documentation/metal/') && a.textContent.trim()) {
                const href = a.href.split('#')[0];
                if (!seen.has(href)) {
                    seen.add(href);
                    const title = a.textContent.trim().replace(/^API Reference/, '').trim();
                    if (title && title.length < 200 && !title.includes('Skip') && !title.includes('Global Nav'))
                        links.push({ title, href });
                }
            }
        });
        const main = document.querySelector('main, #app, .content');
        return { links, text: (main || document.body).innerText.substring(0, 50000) };
    }
    """)


def extract_article_detail(page: Page, url: str) -> dict:
    page.goto(url, wait_until="networkidle", timeout=60000)
    page.wait_for_timeout(2000)
    return page.evaluate("""
    () => {
        let download_url = null;
        document.querySelectorAll('a').forEach(a => {
            if (a.href && (a.href.includes('docs-assets.developer.apple.com') ||
                (a.href.endsWith('.zip') && a.href.includes('apple')))) {
                download_url = a.href;
            }
        });
        const main = document.querySelector('main, #app, .content');
        const body = (main || document.body).innerText.substring(0, 50000);
        const h1 = document.querySelector('h1');
        return {
            title: h1 ? h1.textContent.trim() : '',
            body,
            download_url
        };
    }
    """)


def parse_sections(text: str, links: list) -> list:
    known_headings = [
        "Essentials", "Samples", "GPU devices", "Command encoders",
        "Resources", "Shader compilation and libraries", "Presentation",
        "Developer tools", "Apple silicon",
        "Compute workflows", "Render workflows", "Textures",
        "Argument buffers", "Shaders", "Synchronization",
        "Lighting techniques", "Multiple techniques", "Ray tracing",
        "HDR", "OpenGL",
    ]
    link_map = {l["title"]: l["href"] for l in links}
    sections = []
    current = None
    for line in text.split("\n"):
        s = line.strip()
        if s in known_headings:
            current = {"title": s, "articles": []}
            sections.append(current)
        elif current and s in link_map:
            slug = slug_from_url(link_map[s])
            if slug and slug != "metal" and not is_api_reference(slug):
                current["articles"].append({
                    "title": s, "slug": slug, "url": link_map[s],
                })
    return [s for s in sections if s["articles"]]


def main():
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # Scrape root + sample code library to get full article list
        all_sections = []
        seen_slugs = set()

        for page_url in [
            ROOT_URL,
            f"https://developer.apple.com/documentation/metal/metal-sample-code-library{LANG}",
        ]:
            print(f"Scraping: {page_url}")
            page.goto(page_url, wait_until="networkidle", timeout=60000)
            data = extract_links_and_text(page)
            sections = parse_sections(data["text"], data["links"])
            for s in sections:
                existing = next((x for x in all_sections if x["title"] == s["title"]), None)
                if existing:
                    for a in s["articles"]:
                        if a["slug"] not in seen_slugs:
                            seen_slugs.add(a["slug"])
                            existing["articles"].append(a)
                else:
                    for a in s["articles"]:
                        seen_slugs.add(a["slug"])
                    all_sections.append(s)
            print(f"  Sections so far: {len(all_sections)}, articles: {sum(len(s['articles']) for s in all_sections)}")

        # Scrape each article page for download URL and body text
        articles = [a for s in all_sections for a in s["articles"]]
        total = len(articles)
        print(f"\nScraping {total} article pages...")

        for i, article in enumerate(articles):
            url = article["url"]
            if not url.endswith(LANG):
                url = url.split("?")[0] + LANG
            print(f"  [{i+1}/{total}] {article['slug']}")
            try:
                detail = extract_article_detail(page, url)
                article["body_markdown"] = detail["body"]
                article["download_url"] = detail["download_url"]
                article["has_sample_code"] = detail["download_url"] is not None
            except Exception as e:
                print(f"    ERROR: {e}")
                article["body_markdown"] = ""
                article["download_url"] = None
                article["has_sample_code"] = False

        sample_count = sum(1 for a in articles if a.get("has_sample_code"))
        result = {
            "title": "Metal",
            "url": ROOT_URL,
            "scraped_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
            "sections": all_sections,
            "total_articles": total,
            "total_samples": sample_count,
        }
        with open(OUTPUT, "w") as f:
            json.dump(result, f, indent=2)

        print(f"\nDone! {total} articles ({sample_count} with sample code)")
        print(f"Output: {OUTPUT}")
        browser.close()


if __name__ == "__main__":
    main()
