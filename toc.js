// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="chapter_1.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="chapter_2.html"><strong aria-hidden="true">2.</strong> NES Platform</a></li><li class="chapter-item expanded "><a href="chapter_3.html"><strong aria-hidden="true">3.</strong> Emulating CPU</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="chapter_3_1.html"><strong aria-hidden="true">3.1.</strong> Getting Started</a></li><li class="chapter-item expanded "><a href="chapter_3_2.html"><strong aria-hidden="true">3.2.</strong> Memory addressing modes</a></li><li class="chapter-item expanded "><a href="chapter_3_3.html"><strong aria-hidden="true">3.3.</strong> The rest of the instructions</a></li><li class="chapter-item expanded "><a href="chapter_3_4.html"><strong aria-hidden="true">3.4.</strong> Running our first game</a></li></ol></li><li class="chapter-item expanded "><a href="chapter_4.html"><strong aria-hidden="true">4.</strong> Emulating BUS</a></li><li class="chapter-item expanded "><a href="chapter_5.html"><strong aria-hidden="true">5.</strong> Cartridges</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="chapter_5_1.html"><strong aria-hidden="true">5.1.</strong> Test ROM</a></li></ol></li><li class="chapter-item expanded "><a href="chapter_6.html"><strong aria-hidden="true">6.</strong> Emulating PPU</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="chapter_6_1.html"><strong aria-hidden="true">6.1.</strong> Emulating PPU Registers</a></li><li class="chapter-item expanded "><a href="chapter_6_2.html"><strong aria-hidden="true">6.2.</strong> Emulating NMI Interrupt</a></li><li class="chapter-item expanded "><a href="chapter_6_3.html"><strong aria-hidden="true">6.3.</strong> Rendering CHR Rom Tiles</a></li><li class="chapter-item expanded "><a href="chapter_6_4.html"><strong aria-hidden="true">6.4.</strong> Rendering Static Screen</a></li></ol></li><li class="chapter-item expanded "><a href="chapter_7.html"><strong aria-hidden="true">7.</strong> Emulating Joypads</a></li><li class="chapter-item expanded "><a href="chapter_8.html"><strong aria-hidden="true">8.</strong> PPU Scrolling</a></li><li class="chapter-item expanded "><a href="chapter_9.html"><strong aria-hidden="true">9.</strong> Emulating APU</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><a href="afterwords.html">Afterword</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
