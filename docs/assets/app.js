(function () {
  const LANG_KEY = "rustgram_docs_lang";
  const THEME_KEY = "rustgram_docs_theme";

  function detectInitialLang() {
    const hash = window.location.hash || "";
    if (hash.startsWith("#ru")) return "ru";
    if (hash.startsWith("#en")) return "en";

    const stored = localStorage.getItem(LANG_KEY);
    if (stored === "en" || stored === "ru") return stored;

    const navLang = (navigator.language || "").toLowerCase();
    return navLang.startsWith("ru") ? "ru" : "en";
  }

  function applyLanguage(lang) {
    document.documentElement.lang = lang;
    localStorage.setItem(LANG_KEY, lang);

    document.querySelectorAll("[data-lang]").forEach((el) => {
      if (el.getAttribute("data-lang") === lang) {
        el.classList.remove("hidden-lang");
      } else {
        el.classList.add("hidden-lang");
      }
    });

    document.querySelectorAll(".lang-btn").forEach((btn) => {
      btn.classList.toggle("active", btn.getAttribute("data-set-lang") === lang);
    });

    const searchInput = document.getElementById("doc-search");
    if (searchInput) {
      searchInput.placeholder =
        lang === "ru" ? "Поиск по разделам..." : "Filter documentation...";
    }

    updateCopyButtonLabels(lang);
  }

  function updateCopyButtonLabels(lang) {
    const label = lang === "ru" ? "Копировать" : "Copy";
    document.querySelectorAll(".copy-btn").forEach((btn) => {
      if (btn.dataset.copied !== "1") {
        btn.textContent = label;
      }
    });
  }

  function applyTheme(theme) {
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem(THEME_KEY, theme);
    const toggle = document.getElementById("theme-toggle");
    if (toggle) {
      toggle.textContent = theme === "dark" ? "Light" : "Dark";
    }
  }

  function setupCopyButtons() {
    document.querySelectorAll("pre").forEach((pre) => {
      if (pre.parentElement && pre.parentElement.classList.contains("code-wrapper")) {
        return;
      }
      const wrapper = document.createElement("div");
      wrapper.className = "code-wrapper";
      pre.parentNode.insertBefore(wrapper, pre);
      wrapper.appendChild(pre);

      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = "copy-btn";
      btn.textContent = "Copy";
      btn.addEventListener("click", async () => {
        const code = pre.querySelector("code");
        const text = code ? code.innerText : pre.innerText;
        try {
          await navigator.clipboard.writeText(text);
          const lang = document.documentElement.lang || "en";
          btn.dataset.copied = "1";
          btn.textContent = lang === "ru" ? "Скопировано!" : "Copied!";
          setTimeout(() => {
            btn.dataset.copied = "0";
            updateCopyButtonLabels(document.documentElement.lang || "en");
          }, 1500);
        } catch (_) {
          // Clipboard may be unavailable in restricted contexts.
        }
      });
      wrapper.appendChild(btn);
    });
  }

  function setupSearch() {
    const input = document.getElementById("doc-search");
    if (!input) return;

    input.addEventListener("input", () => {
      const q = input.value.trim().toLowerCase();
      document.querySelectorAll("section.doc-section").forEach((sec) => {
        if (!q) {
          sec.classList.remove("hidden-search");
          return;
        }
        const hay = sec.innerText.toLowerCase();
        sec.classList.toggle("hidden-search", !hay.includes(q));
      });
    });
  }

  function setupScrollSpy() {
    const links = Array.from(document.querySelectorAll(".sidebar .nav-link"));
    const sections = Array.from(document.querySelectorAll("section.doc-section"));

    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            const id = entry.target.id;
            links.forEach((l) => {
              l.classList.toggle("active", l.getAttribute("href") === `#${id}`);
            });
          }
        });
      },
      { rootMargin: "-20% 0px -70% 0px" }
    );

    sections.forEach((s) => observer.observe(s));
  }

  document.addEventListener("DOMContentLoaded", () => {
    setupCopyButtons();
    setupSearch();
    setupScrollSpy();

    const storedTheme = localStorage.getItem(THEME_KEY);
    const prefersDark =
      window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches;
    applyTheme(storedTheme || (prefersDark ? "dark" : "light"));

    applyLanguage(detectInitialLang());

    document.querySelectorAll(".lang-btn").forEach((btn) => {
      btn.addEventListener("click", () => {
        applyLanguage(btn.getAttribute("data-set-lang"));
      });
    });

    const themeBtn = document.getElementById("theme-toggle");
    if (themeBtn) {
      themeBtn.addEventListener("click", () => {
        const cur = document.documentElement.getAttribute("data-theme") || "light";
        applyTheme(cur === "dark" ? "light" : "dark");
      });
    }

    const menuBtn = document.getElementById("menu-toggle");
    const sidebar = document.getElementById("sidebar");
    if (menuBtn && sidebar) {
      menuBtn.addEventListener("click", () => sidebar.classList.toggle("open"));
      sidebar.querySelectorAll(".nav-link").forEach((link) => {
        link.addEventListener("click", () => sidebar.classList.remove("open"));
      });
    }
  });
})();
