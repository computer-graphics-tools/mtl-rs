"""Step 1: Crawl Apple's Metal documentation tree and extract article content.

Only scrapes articles that have sample code downloads.
Extracts proper markdown from HTML using markdownify with custom converters.
Output: output/metal_docs.json
"""

import json
import re
import sys
import time
from pathlib import Path

from bs4 import BeautifulSoup, NavigableString, Tag
from markdownify import MarkdownConverter
from playwright.sync_api import Page, sync_playwright

ROOT_URL = "https://developer.apple.com/documentation/metal?language=objc"
LANG = "?language=objc"
OUTPUT = Path(__file__).parent / "output" / "metal_docs.json"


# ---------------------------------------------------------------------------
# Custom HTML → Markdown converter for Apple Developer Documentation
# ---------------------------------------------------------------------------


class AppleDocsConverter(MarkdownConverter):
    """Handles Apple-specific HTML patterns in developer documentation."""

    def convert_svg(self, el, text, parent_tags):
        """Strip SVG icons (anchor link icons in headings)."""
        return ""

    def convert_nav(self, el, text, parent_tags):
        """Strip navigation elements (tab navs above code blocks are handled
        at the TabNavigator level)."""
        return ""

    def convert_wbr(self, el, text, parent_tags):
        """Strip <wbr> word-break hints."""
        return ""

    def convert_a(self, el, text, parent_tags):
        """Convert links, making relative URLs absolute."""
        href = el.get("href", "")

        # Skip anchor links that are just heading self-links
        if "header-anchor" in el.get("class", []):
            return text

        # Make relative URLs absolute
        if href.startswith("/"):
            href = f"https://developer.apple.com{href}"

        if not text.strip():
            return ""

        return f"[{text.strip()}]({href})"

    def convert_img(self, el, text, parent_tags):
        """Convert images with absolute URLs."""
        src = el.get("src", "")
        alt = el.get("alt", "")
        if src.startswith("/"):
            src = f"https://developer.apple.com{src}"
        return f"![{alt}]({src})"

    def convert_aside(self, el, text, parent_tags):
        """Convert aside/note blocks to blockquotes."""
        # Determine note type from class
        classes = el.get("class", [])
        if "note" in classes:
            prefix = "Note"
        elif "important" in classes:
            prefix = "Important"
        elif "warning" in classes:
            prefix = "Warning"
        elif "tip" in classes:
            prefix = "Tip"
        else:
            prefix = "Note"

        content = text.strip()
        # Remove any leading "Note" or similar label that Apple includes
        for label in ("Note", "Important", "Warning", "Tip"):
            if content.startswith(label):
                content = content[len(label) :].strip()

        lines = content.split("\n")
        quoted = "\n".join(f"> {line}" for line in lines)
        return f"\n> **{prefix}:** {lines[0]}\n" + (
            "\n".join(f"> {line}" for line in lines[1:]) + "\n\n"
            if len(lines) > 1
            else "\n"
        )


def _get_code_syntax(el):
    """Find the data-syntax attribute on the element or its children."""
    # Check the element itself
    if hasattr(el, "get") and el.get("data-syntax"):
        return el["data-syntax"]
    # Check child <code> element (pre > code pattern)
    if hasattr(el, "find"):
        code = el.find("code")
        if code and code.get("data-syntax"):
            return code["data-syntax"]
    return ""


def preprocess_apple_html(html: str) -> str:
    """Pre-process Apple's HTML before passing to markdownify.

    Handles patterns that are hard to deal with in the converter:
    - TabNavigator + code-listing → fenced code blocks
    - Strips data-v-* attributes and Vue.js noise
    - Strips code-line-container spans to get clean code text
    """
    soup = BeautifulSoup(html, "html.parser")

    # Handle TabNavigator blocks: extract filename + code content
    for tab_nav in soup.find_all("div", class_="TabNavigator"):
        # Get the filename from the tab link
        tab_link = tab_nav.find("a", class_="tabnav-link")
        filename = tab_link.get_text(strip=True) if tab_link else ""

        # Find the code listing inside
        code_listing = tab_nav.find("div", class_="code-listing")
        if code_listing:
            syntax = code_listing.get("data-syntax", "")
            # Get clean code text from all code-line spans
            code_lines = []
            for line_container in code_listing.find_all("span", class_="code-line"):
                code_lines.append(line_container.get_text())

            code_text = "\n".join(code_lines)

            # Build replacement HTML: a simple pre/code block
            replacement = soup.new_tag("div")
            if filename:
                bold = soup.new_tag("p")
                strong = soup.new_tag("strong")
                strong.string = filename
                bold.append(strong)
                replacement.append(bold)

            pre = soup.new_tag("pre")
            code = soup.new_tag("code")
            code["class"] = [f"language-{syntax}"] if syntax else []
            code["data-syntax"] = syntax
            code.string = code_text
            pre.append(code)
            replacement.append(pre)

            tab_nav.replace_with(replacement)
        else:
            tab_nav.decompose()

    # Handle standalone code-listing blocks (not inside TabNavigator)
    for code_listing in soup.find_all("div", class_="code-listing"):
        syntax = code_listing.get("data-syntax", "")
        code_lines = []
        for line_container in code_listing.find_all("span", class_="code-line"):
            code_lines.append(line_container.get_text())

        code_text = "\n".join(code_lines)

        pre = soup.new_tag("pre")
        code = soup.new_tag("code")
        code["class"] = [f"language-{syntax}"] if syntax else []
        code["data-syntax"] = syntax
        code.string = code_text
        pre.append(code)
        code_listing.replace_with(pre)

    # Strip <wbr> tags inside <code> elements (word-break hints)
    for wbr in soup.find_all("wbr"):
        wbr.decompose()

    # Strip all SVG elements (heading anchor icons)
    for svg in soup.find_all("svg"):
        svg.decompose()

    # Remove header anchor links - just keep the text
    for a in soup.find_all("a", class_="header-anchor"):
        a.replace_with(a.get_text(strip=True))

    return str(soup)


def html_to_markdown(html: str) -> str:
    """Convert Apple documentation HTML to clean markdown."""
    # Pre-process to handle Apple-specific patterns
    processed = preprocess_apple_html(html)

    # Convert to markdown
    md = AppleDocsConverter(
        heading_style="atx",
        bullets="-",
        strong_em_symbol="*",
        code_language_callback=lambda el: _get_code_syntax(el),
    ).convert(processed)

    # Clean up excessive whitespace
    md = re.sub(r"\n{3,}", "\n\n", md)
    md = md.strip()

    return md


# ---------------------------------------------------------------------------
# Scraping logic
# ---------------------------------------------------------------------------


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


def extract_article_detail(page: Page, url: str, retries: int = 3) -> dict:
    """Extract article HTML content and metadata from a documentation page."""
    last_error = None
    for attempt in range(retries):
        try:
            page.goto(url, wait_until="networkidle", timeout=60000)
            page.wait_for_timeout(2000)
            break
        except Exception as e:
            last_error = e
            if attempt < retries - 1:
                wait = 5 * (attempt + 1)
                print(f"    Retry {attempt + 1}/{retries} after {wait}s...", flush=True)
                time.sleep(wait)
            else:
                raise last_error
    return page.evaluate("""
    () => {
        let download_url = null;
        document.querySelectorAll('a').forEach(a => {
            if (a.href && (a.href.includes('docs-assets.developer.apple.com') ||
                (a.href.endsWith('.zip') && a.href.includes('apple')))) {
                download_url = a.href;
            }
        });

        // Extract abstract from hero section
        const hero = document.querySelector('.documentation-hero__content');
        let abstract_html = '';
        if (hero) {
            const abs = hero.querySelector('.abstract');
            if (abs) abstract_html = abs.innerHTML;
        }

        // Extract primary content HTML
        const primary = document.querySelector('.primary-content');
        let body_html = '';
        if (primary) {
            body_html = primary.innerHTML;
        }

        const h1 = document.querySelector('h1');

        return {
            title: h1 ? h1.textContent.trim() : '',
            abstract_html,
            body_html,
            download_url
        };
    }
    """)


def parse_sections(text: str, links: list) -> list:
    known_headings = [
        "Essentials",
        "Samples",
        "GPU devices",
        "Command encoders",
        "Resources",
        "Shader compilation and libraries",
        "Presentation",
        "Developer tools",
        "Apple silicon",
        "Compute workflows",
        "Render workflows",
        "Textures",
        "Argument buffers",
        "Shaders",
        "Synchronization",
        "Lighting techniques",
        "Multiple techniques",
        "Ray tracing",
        "HDR",
        "OpenGL",
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
                current["articles"].append(
                    {
                        "title": s,
                        "slug": slug,
                        "url": link_map[s],
                    }
                )
    return [s for s in sections if s["articles"]]


def main():
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    log_path = Path(__file__).parent / "scrape_output.log"

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
                existing = next(
                    (x for x in all_sections if x["title"] == s["title"]), None
                )
                if existing:
                    for a in s["articles"]:
                        if a["slug"] not in seen_slugs:
                            seen_slugs.add(a["slug"])
                            existing["articles"].append(a)
                else:
                    for a in s["articles"]:
                        seen_slugs.add(a["slug"])
                    all_sections.append(s)
            print(
                f"  Sections so far: {len(all_sections)}, "
                f"articles: {sum(len(s['articles']) for s in all_sections)}"
            )

        # Visit each article page — only do full extraction for sample-code articles
        articles = [a for s in all_sections for a in s["articles"]]
        total = len(articles)
        print(f"\nScraping {total} article pages...", flush=True)

        sample_count = 0
        for i, article in enumerate(articles):
            url = article["url"]
            if not url.endswith(LANG):
                url = url.split("?")[0] + LANG
            print(f"  [{i + 1}/{total}] {article['slug']}", flush=True)
            try:
                detail = extract_article_detail(page, url)
                article["download_url"] = detail["download_url"]
                article["has_sample_code"] = detail["download_url"] is not None

                if article["has_sample_code"]:
                    sample_count += 1
                    # Convert HTML to proper markdown
                    abstract_md = ""
                    if detail["abstract_html"]:
                        abstract_md = html_to_markdown(detail["abstract_html"])

                    body_md = ""
                    if detail["body_html"]:
                        body_md = html_to_markdown(detail["body_html"])

                    article["overview"] = abstract_md
                    article["body_markdown"] = body_md
                    print(
                        f"    ✓ Sample code article ({len(body_md)} chars markdown)",
                        flush=True,
                    )
                else:
                    article["overview"] = ""
                    article["body_markdown"] = ""
            except Exception as e:
                print(f"    ERROR: {e}", flush=True)
                article["body_markdown"] = ""
                article["overview"] = ""
                article["download_url"] = None
                article["has_sample_code"] = False

        # Filter sections to only include articles with sample code
        filtered_sections = []
        for section in all_sections:
            sample_articles = [
                a for a in section["articles"] if a.get("has_sample_code")
            ]
            if sample_articles:
                filtered_sections.append(
                    {
                        "title": section["title"],
                        "articles": sample_articles,
                    }
                )

        filtered_total = sum(len(s["articles"]) for s in filtered_sections)

        result = {
            "title": "Metal",
            "url": ROOT_URL,
            "scraped_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
            "sections": filtered_sections,
            "total_articles": filtered_total,
            "total_samples": sample_count,
        }
        with open(OUTPUT, "w") as f:
            json.dump(result, f, indent=2)

        print(f"\nDone! {filtered_total} sample-code articles (from {total} total)")
        print(f"Output: {OUTPUT}")
        browser.close()


if __name__ == "__main__":
    main()
