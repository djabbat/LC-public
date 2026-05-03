(function(){
  // shared theme across *.longevity.ge — cookie on parent domain, fallback to localStorage
  function getTheme(){
    var m = document.cookie.match(/(?:^|; )lc_theme=([^;]+)/);
    if (m) return decodeURIComponent(m[1]);
    return localStorage.getItem("theme");
  }
  function setTheme(v){
    var d = new Date(); d.setTime(d.getTime()+365*24*60*60*1000);
    document.cookie = "lc_theme="+encodeURIComponent(v)+"; expires="+d.toUTCString()+"; path=/; domain=.longevity.ge; SameSite=Lax";
    try { localStorage.setItem("theme", v); } catch(e) {}
  }
  var t = getTheme();
  if (t === "dark") document.documentElement.setAttribute("data-theme","dark");

  var host = window.location.hostname;
  var path = window.location.pathname;
  var links = [
    ["https://longevity.ge","Home","longevity.ge","/"],
    ["https://mcoa.longevity.ge","MCOA","mcoa.longevity.ge",null],
    ["https://cdata.longevity.ge","CDATA","cdata.longevity.ge",null],
    ["https://ze.longevity.ge","Ze","ze.longevity.ge",null],
    ["https://biosense.longevity.ge","BioSense","biosense.longevity.ge",null],
    ["https://fclc.longevity.ge","FCLC","fclc.longevity.ge",null],
    ["https://hive.longevity.ge","Hive","hive.longevity.ge",null],
    ["https://longevity.ge/rescience/","Annals","longevity.ge","/rescience/"],
    ["https://longevity.ge/team/","Team","longevity.ge","/team/"],
    ["https://longevity.ge/#donate","Donate",null,null],
    ["https://github.com/djabbat/LongevityCommon","Source",null,null]
  ];
  function isActive(l){
    if (l[2] !== host) return false;
    if (l[3] === null) return true;
    if (l[3] === "/") {
      return path === "/" || path === "" || (path.indexOf("/rescience/") !== 0 && path.indexOf("/team/") !== 0 && path.indexOf("/longhoriz/") !== 0);
    }
    return path.indexOf(l[3]) === 0;
  }
  var nav = links.map(function(l){
    var act = isActive(l) ? " class=\"active\"" : "";
    var rel = (l[1] === "Source") ? " rel=\"noopener\"" : "";
    return "<a href=\"" + l[0] + "\"" + act + rel + ">" + l[1] + "</a>";
  }).join("\n");

  var html = "<div class=\"eco-bar-injected\"><div class=\"eco-inner-i\"><span class=\"eco-brand-i\">LongevityCommon</span><nav class=\"eco-nav-i\">" + nav + "<button type=\"button\" class=\"theme-toggle-i\" aria-label=\"Toggle dark mode\">☾</button></nav></div></div>";

  var style = document.createElement("style");
  style.textContent = [
    ".eco-bar-injected{position:sticky !important;top:0 !important;z-index:100 !important;background:rgba(15,23,42,0.97) !important;backdrop-filter:blur(8px) !important;border-bottom:1px solid rgba(255,255,255,0.06) !important;font-family:Inter,-apple-system,system-ui,sans-serif !important;font-size:15px !important;line-height:1.4 !important;width:100% !important;box-sizing:border-box !important;margin:0 !important}",
    ".eco-inner-i{max-width:1100px !important;margin:0 auto !important;padding:12px 32px !important;display:flex !important;align-items:center !important;justify-content:space-between !important;gap:16px !important;flex-wrap:wrap !important;box-sizing:border-box !important;width:100% !important}",
    ".eco-brand-i{font-weight:700 !important;font-size:15px !important;color:#fff !important;letter-spacing:-0.01em !important;line-height:1.2 !important}",
    ".eco-brand-i::before{content:\"\\25CF\" !important;color:#4f46e5 !important;margin-right:8px !important;font-size:10px !important;vertical-align:middle !important}",
    ".eco-nav-i{display:flex !important;gap:2px !important;flex-wrap:wrap !important;align-items:center !important;font-size:13px !important;background:transparent !important;border:none !important;position:static !important}",
    ".eco-nav-i a{color:#cbd5e1 !important;padding:6px 12px !important;border-radius:6px !important;font-size:13px !important;font-weight:500 !important;transition:all 0.15s !important;text-decoration:none !important;line-height:1.2 !important;background:transparent !important;border:none !important}",
    ".eco-nav-i a:hover{background:rgba(255,255,255,0.08) !important;color:#fff !important}",
    ".eco-nav-i a.active{background:#4f46e5 !important;color:#fff !important}",
    ".theme-toggle-i{background:transparent !important;border:1px solid rgba(255,255,255,0.35) !important;color:#fff !important;cursor:pointer !important;padding:4px 10px !important;border-radius:4px !important;font-size:16px !important;margin-left:12px !important;line-height:1 !important}",
    ".theme-toggle-i:hover{background:rgba(255,255,255,0.12) !important}",
    "html[data-theme=\"dark\"] body{background:#0f1117 !important;color:#e0e3eb !important}",
    "html[data-theme=\"dark\"] .eco-bar-injected{background:rgba(6,8,15,0.97) !important}",
    "html[data-theme=\"dark\"] .header,html[data-theme=\"dark\"] .card,html[data-theme=\"dark\"] section,html[data-theme=\"dark\"] .axiom{background:#15171f !important;color:#d8dce4 !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] h1,html[data-theme=\"dark\"] h2,html[data-theme=\"dark\"] h3,html[data-theme=\"dark\"] h4{color:#fff !important}",
    "html[data-theme=\"dark\"] code,html[data-theme=\"dark\"] .formula,html[data-theme=\"dark\"] .badge{background:#2a2f40 !important;color:#e0e3eb !important}",
    "html[data-theme=\"dark\"] a{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .pkp_structure_page,html[data-theme=\"dark\"] .pkp_structure_head,html[data-theme=\"dark\"] .pkp_structure_main,html[data-theme=\"dark\"] .pkp_structure_content,html[data-theme=\"dark\"] .pkp_brand_footer,html[data-theme=\"dark\"] .pkp_footer_content,html[data-theme=\"dark\"] .pkp_block,html[data-theme=\"dark\"] .pkp_structure_footer,html[data-theme=\"dark\"] .pkp_structure_footer_wrapper{background:#0f1117 !important;color:#d8dce4 !important;border-color:#2a2f40 !important}",
"html[data-theme=\"dark\"] .pkp_site_name,html[data-theme=\"dark\"] .pkp_site_name *,html[data-theme=\"dark\"] .pkp_navigation_primary a,html[data-theme=\"dark\"] .pkp_navigation_user a,html[data-theme=\"dark\"] .pkp_brand_footer a{color:#fff !important}",
"html[data-theme=\"dark\"] .pkp_navigation_primary,html[data-theme=\"dark\"] .pkp_navigation_user,html[data-theme=\"dark\"] .pkp_navigation_primary_wrapper{background:#15171f !important;border-color:#2a2f40 !important}",
"html[data-theme=\"dark\"] .pkp_block,html[data-theme=\"dark\"] .pkp_block *{background-color:#15171f !important;color:#d8dce4 !important;border-color:#2a2f40 !important}",
"html[data-theme=\"dark\"] .pkp_structure_head{border-bottom-color:#2a2f40 !important}",
"html[data-theme=\"dark\"] input,html[data-theme=\"dark\"] textarea,html[data-theme=\"dark\"] select{background:#1a1d28 !important;color:#e0e3eb !important;border-color:#2a2f40 !important}",
"html[data-theme=\"dark\"] .pkp_search input{background:#1a1d28 !important;color:#e0e3eb !important}",
    "html[data-theme=\"dark\"] body,html[data-theme=\"dark\"] p,html[data-theme=\"dark\"] li,html[data-theme=\"dark\"] td,html[data-theme=\"dark\"] dt,html[data-theme=\"dark\"] dd,html[data-theme=\"dark\"] label,html[data-theme=\"dark\"] span,html[data-theme=\"dark\"] em,html[data-theme=\"dark\"] strong,html[data-theme=\"dark\"] small,html[data-theme=\"dark\"] article,html[data-theme=\"dark\"] section,html[data-theme=\"dark\"] div{color:#d8dce4}",
    "html[data-theme=\"dark\"] .eco-bar-injected,html[data-theme=\"dark\"] .eco-bar-injected *{color:inherit}",
    "html[data-theme=\"dark\"] .eco-nav-i a{color:#cbd5e1}",
    "html[data-theme=\"dark\"] .eco-brand-i{color:#fff}",
    "html[data-theme=\"dark\"] .pkp_block_title,html[data-theme=\"dark\"] .pkp_block li,html[data-theme=\"dark\"] .pkp_block a,html[data-theme=\"dark\"] .obj_announcement_summary,html[data-theme=\"dark\"] .obj_article_summary,html[data-theme=\"dark\"] .cmp_announcement_summary,html[data-theme=\"dark\"] .cmp_article_list,html[data-theme=\"dark\"] .pkp_structure_main *{color:#d8dce4}",
    "html[data-theme=\"dark\"] .pkp_structure_main h1,html[data-theme=\"dark\"] .pkp_structure_main h2,html[data-theme=\"dark\"] .pkp_structure_main h3,html[data-theme=\"dark\"] .pkp_structure_main h4{color:#fff}",
    "html[data-theme=\"dark\"] .pkp_structure_main a{color:#88a8ff}",
    "html[data-theme=\"dark\"] [style*=\"color:#18181b\"],html[data-theme=\"dark\"] [style*=\"color: #18181b\"],html[data-theme=\"dark\"] [style*=\"color:#27272a\"],html[data-theme=\"dark\"] [style*=\"color: #27272a\"],html[data-theme=\"dark\"] [style*=\"color:#3f3f46\"],html[data-theme=\"dark\"] [style*=\"color:#52525b\"],html[data-theme=\"dark\"] [style*=\"color: #52525b\"],html[data-theme=\"dark\"] [style*=\"color:#71717a\"],html[data-theme=\"dark\"] [style*=\"color: #71717a\"]{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] [style*=\"background:#fff\"],html[data-theme=\"dark\"] [style*=\"background: #fff\"],html[data-theme=\"dark\"] [style*=\"background:white\"],html[data-theme=\"dark\"] [style*=\"background:#fafafa\"],html[data-theme=\"dark\"] [style*=\"background:#f4f4f5\"]{background-color:#15171f !important}",
    "html[data-theme=\"dark\"] .obj_article_summary,html[data-theme=\"dark\"] .obj_issue_toc,html[data-theme=\"dark\"] .cmp_article_list,html[data-theme=\"dark\"] .current_issue,html[data-theme=\"dark\"] .homepage_about,html[data-theme=\"dark\"] .highlights,html[data-theme=\"dark\"] .footer-container,html[data-theme=\"dark\"] .swiper-slide,html[data-theme=\"dark\"] .swiper-slide-content{background-color:#15171f !important;color:#d8dce4 !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .obj_article_summary .title,html[data-theme=\"dark\"] .obj_article_summary .title a,html[data-theme=\"dark\"] .current_issue_title,html[data-theme=\"dark\"] .swiper-slide-title,html[data-theme=\"dark\"] .journal-name,html[data-theme=\"dark\"] .section,html[data-theme=\"dark\"] .sections,html[data-theme=\"dark\"] h2.title,html[data-theme=\"dark\"] h3.title{color:#fff !important}",
    "html[data-theme=\"dark\"] .obj_article_summary .authors,html[data-theme=\"dark\"] .authors,html[data-theme=\"dark\"] .meta,html[data-theme=\"dark\"] .meta *,html[data-theme=\"dark\"] .description,html[data-theme=\"dark\"] .published,html[data-theme=\"dark\"] .label,html[data-theme=\"dark\"] .heading,html[data-theme=\"dark\"] .issn,html[data-theme=\"dark\"] .copyright,html[data-theme=\"dark\"] .rights-access{color:#c8ccd5 !important}",
    "html[data-theme=\"dark\"] .obj_galley_link,html[data-theme=\"dark\"] .obj_galley_link.pdf,html[data-theme=\"dark\"] .read_more,html[data-theme=\"dark\"] .pkp_button,html[data-theme=\"dark\"] .swiper-slide-button{background-color:#1a2440 !important;color:#fff !important;border-color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .obj_galley_link:hover,html[data-theme=\"dark\"] .pkp_button:hover{background-color:#2a3450 !important}",
    "html[data-theme=\"dark\"] .swiper-pagination-bullet{background:#88a8ff !important}",
    "html[data-theme=\"dark\"] .galleys_links{background:transparent !important}",
    "html[data-theme=\"dark\"] a:not(.pkp_button):not(.obj_galley_link):not(.read_more){color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .page_article,html[data-theme=\"dark\"] .page_article *,html[data-theme=\"dark\"] .main_entry,html[data-theme=\"dark\"] .main_entry *,html[data-theme=\"dark\"] .entry_details,html[data-theme=\"dark\"] .entry_details *,html[data-theme=\"dark\"] .obj_article_details,html[data-theme=\"dark\"] .obj_article_details *{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .page_article h1,html[data-theme=\"dark\"] .page_title,html[data-theme=\"dark\"] .label,html[data-theme=\"dark\"] .item h2,html[data-theme=\"dark\"] .item h3,html[data-theme=\"dark\"] .obj_article_details .label{color:#fff !important}",
    "html[data-theme=\"dark\"] .csl-bib-body,html[data-theme=\"dark\"] .csl-entry,html[data-theme=\"dark\"] .csl-bib-body *,html[data-theme=\"dark\"] .csl-entry *,html[data-theme=\"dark\"] .references,html[data-theme=\"dark\"] .references *,html[data-theme=\"dark\"] .item.references,html[data-theme=\"dark\"] .item.references *{color:#c8ccd5 !important;background:transparent !important}",
    "html[data-theme=\"dark\"] .item.abstract,html[data-theme=\"dark\"] .item.abstract *,html[data-theme=\"dark\"] .item.authors,html[data-theme=\"dark\"] .item.authors *,html[data-theme=\"dark\"] .item.published,html[data-theme=\"dark\"] .item.published *,html[data-theme=\"dark\"] .item.issue,html[data-theme=\"dark\"] .item.section,html[data-theme=\"dark\"] .item.keywords,html[data-theme=\"dark\"] .item.copyright,html[data-theme=\"dark\"] .item.doi,html[data-theme=\"dark\"] .item.citation,html[data-theme=\"dark\"] .item.galleys{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .userGroup,html[data-theme=\"dark\"] .profile,html[data-theme=\"dark\"] .name,html[data-theme=\"dark\"] .citation_formats_styles,html[data-theme=\"dark\"] .sub_item,html[data-theme=\"dark\"] .citation_display,html[data-theme=\"dark\"] .cmp_breadcrumbs,html[data-theme=\"dark\"] .cmp_breadcrumbs *,html[data-theme=\"dark\"] .newsletter-signup-ojs,html[data-theme=\"dark\"] .newsletter-signup-ojs *,html[data-theme=\"dark\"] .pflPlugin,html[data-theme=\"dark\"] .pflPlugin *{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .page_article a,html[data-theme=\"dark\"] .csl-entry a,html[data-theme=\"dark\"] .references a,html[data-theme=\"dark\"] .citation_formats_styles a{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .citation_formats_list{background-color:#15171f !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .citation_formats_button{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .getftr,html[data-theme=\"dark\"] .getftr *{background:transparent !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .getftr [class*=\"button\"],html[data-theme=\"dark\"] .getftr [role=\"button\"]{background:#1a2440 !important;color:#fff !important;border-color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .newsletter-signup-ojs{background:#15171f !important;border-left-color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .newsletter-signup-ojs *{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .newsletter-signup-ojs input[type=email]{background:#1a1d28 !important;color:#e0e3eb !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .newsletter-signup-ojs input[type=email]::placeholder{color:#7a808d !important}",
    "html[data-theme=\"dark\"] .newsletter-signup-ojs button[type=submit]{background:#1a2440 !important;color:#fff !important;border-color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .citation_display,html[data-theme=\"dark\"] #citationOutput,html[data-theme=\"dark\"] .citation_display .value,html[data-theme=\"dark\"] .citation_display .label,html[data-theme=\"dark\"] .item.citation,html[data-theme=\"dark\"] .item.citation *,html[data-theme=\"dark\"] .csl-bib-body,html[data-theme=\"dark\"] .csl-entry,html[data-theme=\"dark\"] .csl-entry i,html[data-theme=\"dark\"] .csl-entry em{background:transparent !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .citation_display .label{color:#fff !important}",
    "html[data-theme=\"dark\"] .csl-entry a,html[data-theme=\"dark\"] .item.citation a{color:#88a8ff !important;background:transparent !important}",
    "html[data-theme=\"dark\"] .item.doi,html[data-theme=\"dark\"] .item.doi *{background:transparent !important}",
    "html[data-theme=\"dark\"] .item.doi .value a{color:#88a8ff !important;background:transparent !important}",
    "html[data-theme=\"dark\"] .pkp_block,html[data-theme=\"dark\"] .pkp_block .content,html[data-theme=\"dark\"] .pkp_block .content ul,html[data-theme=\"dark\"] .pkp_block .content li{background:#15171f !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .pkp_block .title{color:#fff !important;border-bottom-color:#88a8ff !important;background:transparent !important}",
    "html[data-theme=\"dark\"] .obj_galley_link.pdf{background:#1a2440 !important;color:#fff !important;border:1px solid #88a8ff !important}",
    "html[data-theme=\"dark\"] aside[style*=\"background\"],html[data-theme=\"dark\"] section[style*=\"background\"],html[data-theme=\"dark\"] div[style*=\"background:#f\"],html[data-theme=\"dark\"] div[style*=\"background: #f\"]{background:#15171f !important}",
    "html[data-theme=\"dark\"] [style*=\"color:#0a2540\"],html[data-theme=\"dark\"] [style*=\"color: #0a2540\"],html[data-theme=\"dark\"] [style*=\"color:#222\"],html[data-theme=\"dark\"] [style*=\"color: #222\"]{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] header:not(.eco-bar-injected),html[data-theme=\"dark\"] footer{background:#15171f !important;color:#d8dce4 !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] header:not(.eco-bar-injected) *,html[data-theme=\"dark\"] footer *{color:#d8dce4 !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] header:not(.eco-bar-injected) a,html[data-theme=\"dark\"] footer a{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .bg-white,html[data-theme=\"dark\"] .bg-zinc-50,html[data-theme=\"dark\"] .bg-zinc-100,html[data-theme=\"dark\"] .bg-zinc-200,html[data-theme=\"dark\"] .bg-gray-50,html[data-theme=\"dark\"] .bg-gray-100{background-color:#15171f !important}",
    "html[data-theme=\"dark\"] .text-zinc-500,html[data-theme=\"dark\"] .text-zinc-600,html[data-theme=\"dark\"] .text-zinc-700,html[data-theme=\"dark\"] .text-zinc-800,html[data-theme=\"dark\"] .text-zinc-900,html[data-theme=\"dark\"] .text-gray-500,html[data-theme=\"dark\"] .text-gray-600,html[data-theme=\"dark\"] .text-gray-700,html[data-theme=\"dark\"] .text-gray-800,html[data-theme=\"dark\"] .text-gray-900{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .border-zinc-100,html[data-theme=\"dark\"] .border-zinc-200,html[data-theme=\"dark\"] .border-zinc-300,html[data-theme=\"dark\"] .border-gray-100,html[data-theme=\"dark\"] .border-gray-200,html[data-theme=\"dark\"] .border-gray-300{border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .hover\\:bg-zinc-100:hover,html[data-theme=\"dark\"] .hover\\:bg-zinc-200:hover,html[data-theme=\"dark\"] .hover\\:text-zinc-700:hover,html[data-theme=\"dark\"] .hover\\:text-zinc-900:hover{background-color:#1a2440 !important;color:#fff !important}",
    "html[data-theme=\"dark\"] .bg-blue-600{background-color:#4f46e5 !important}",
    "header:not(.eco-bar-injected) > *,footer > *,.max-w-7xl,.max-w-6xl,.max-w-5xl,.max-w-4xl{max-width:1100px !important;margin-left:auto !important;margin-right:auto !important;box-sizing:border-box !important}",
    ".container,.header-inner,.footer-inner,.section-inner,.page-hero-inner,.hero-inner,.footer-grid,main > section,main > article,main > div{max-width:1100px !important;margin-left:auto !important;margin-right:auto !important;box-sizing:border-box !important}",
    ".section .title,.pkp_block .title,.issue_heading,.issue_identify,.pkp_navigation_primary ul,.pkp_structure_footer,h2.pkp_helpers_align_left{border-top-color:transparent !important;border-bottom-color:transparent !important;border-left-color:transparent !important}",
    "[style*=\"crimson\"]{border-color:transparent !important}",
    ".card[style*=\"crimson\"],.card[style*=\"border-top:3px solid var(--crimson)\"],.card[style*=\"border-top: 3px solid var(--crimson)\"]{border-top:0 !important}",
    "html[data-theme=\"dark\"] .pkp_navigation_primary,html[data-theme=\"dark\"] .pkp_navigation_primary_wrapper,html[data-theme=\"dark\"] .pkp_navigation_primary_row,html[data-theme=\"dark\"] .pkp_navigation_user_wrapper,html[data-theme=\"dark\"] .pkp_navigation_search_wrapper,html[data-theme=\"dark\"] .pkp_navigation_user{background:#15171f !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .pkp_navigation_primary a,html[data-theme=\"dark\"] .pkp_navigation_user a,html[data-theme=\"dark\"] .pkp_navigation_primary ul a{color:#e0e3eb !important;background:transparent !important}",
    "html[data-theme=\"dark\"] .pkp_navigation_primary a:hover,html[data-theme=\"dark\"] .pkp_navigation_primary li.current > a,html[data-theme=\"dark\"] .pkp_navigation_user a:hover,html[data-theme=\"dark\"] .pkp_navigation_primary ul a:hover{color:#88a8ff !important;background:#1a2440 !important}",
    "html[data-theme=\"dark\"] .pkp_navigation_primary ul{background:#15171f !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .pkp_navigation_primary ul a{border-bottom-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .pkp_search{color:#e0e3eb !important;background:#1a1d28 !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .pkp_search:hover{color:#88a8ff !important;border-color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .pkp_search input,html[data-theme=\"dark\"] .pkp_navigation_search_wrapper input{background:#1a1d28 !important;color:#e0e3eb !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .pkp_site_name a{color:#fff !important}",
    "html[data-theme=\"dark\"] .pkp_site_name a:hover{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .dropdown-menu{background:#15171f !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .dropdown-menu a,html[data-theme=\"dark\"] .dropdown-menu li a{color:#e0e3eb !important;background:transparent !important}",
    "html[data-theme=\"dark\"] .dropdown-menu a:hover{color:#88a8ff !important;background:#1a2440 !important}",
    "html[data-theme=\"dark\"] .pkp_site_nav_menu,html[data-theme=\"dark\"] .pkp_navigation_primary_row{background:#15171f !important}",
    "html[data-theme=\"dark\"] .footer-container,html[data-theme=\"dark\"] .footer-container *{background-color:#15171f !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .footer-container a{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .homepage_about,html[data-theme=\"dark\"] .homepage_about *{background:#15171f !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .obj_article_summary{background:#15171f !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .item.galleys,html[data-theme=\"dark\"] .page_article .item.galleys{background:#15171f !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] fieldset,html[data-theme=\"dark\"] .page_search fieldset,html[data-theme=\"dark\"] fieldset.search_advanced,html[data-theme=\"dark\"] .page_search fieldset.search_advanced{background:#15171f !important;border-color:#2a2f40 !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] fieldset legend,html[data-theme=\"dark\"] .page_search fieldset legend{color:#fff !important;background:transparent !important}",
    "html[data-theme=\"dark\"] fieldset label,html[data-theme=\"dark\"] fieldset .label,html[data-theme=\"dark\"] .page_search label{color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] select,html[data-theme=\"dark\"] option{background:#1a1d28 !important;color:#e0e3eb !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] input[type=text],html[data-theme=\"dark\"] input[type=search],html[data-theme=\"dark\"] input[type=email],html[data-theme=\"dark\"] input[type=password],html[data-theme=\"dark\"] input[type=number]{background:#1a1d28 !important;color:#e0e3eb !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] input::placeholder,html[data-theme=\"dark\"] textarea::placeholder{color:#7a808d !important}",
    "html[data-theme=\"dark\"] .page_search input.query,html[data-theme=\"dark\"] .page_search .search_input input[type=text]{background:#1a1d28 !important;color:#e0e3eb !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .galleys_links,html[data-theme=\"dark\"] .galleys_links li{background:transparent !important;list-style:none !important}",
    "html[data-theme=\"dark\"] .galleys_links li a,html[data-theme=\"dark\"] .obj_galley_link,html[data-theme=\"dark\"] .obj_galley_link.pdf,html[data-theme=\"dark\"] .obj_galley_link:not(.pdf){background:#1a2440 !important;color:#fff !important;border:1px solid #88a8ff !important}",
    "html[data-theme=\"dark\"] .galleys_links li a:hover,html[data-theme=\"dark\"] .obj_galley_link:hover,html[data-theme=\"dark\"] .obj_galley_link.pdf:hover{background:#2a3450 !important;color:#fff !important;border-color:#a8bfff !important}",
    "html[data-theme=\"dark\"] .swiper-slide-content,html[data-theme=\"dark\"] .swiper-slide-desc,html[data-theme=\"dark\"] .swiper-slide-title{color:#fff !important}",
    "html[data-theme=\"dark\"] .swiper-slide-button.pkp_button{background:#1a2440 !important;color:#fff !important;border-color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .pkp_brand_footer,html[data-theme=\"dark\"] .pkp_brand_footer *{background:#0f1117 !important;color:#d8dce4 !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .pkp_brand_footer a{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .cmp_breadcrumbs,html[data-theme=\"dark\"] .cmp_breadcrumbs li,html[data-theme=\"dark\"] .cmp_breadcrumbs a{color:#d8dce4 !important;background:transparent !important}",
    "html[data-theme=\"dark\"] .cmp_breadcrumbs a{color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .heading,html[data-theme=\"dark\"] .heading *{background:transparent !important;color:#d8dce4 !important}",
    "html[data-theme=\"dark\"] .pkp_structure_head,html[data-theme=\"dark\"] .pkp_head_wrapper,html[data-theme=\"dark\"] .pkp_site_name_wrapper{background:#15171f !important;border-color:#2a2f40 !important;box-shadow:none !important}",
    "html[data-theme=\"dark\"] .pkp_site_name a,html[data-theme=\"dark\"] .longevity-journal-name{color:#fff !important}",
    "html[data-theme=\"dark\"] .longevity-platform-corner,html[data-theme=\"dark\"] .longevity-platform-corner:hover,html[data-theme=\"dark\"] .longevity-platform-corner:visited,html[data-theme=\"dark\"] .longevity-platform-corner:focus{color:#88a8ff !important;opacity:0.7 !important}",
    "html[data-theme=\"dark\"] .pkp_site_nav_toggle{color:#e0e3eb !important;background:transparent !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .task_count{background:#1a2440 !important;color:#88a8ff !important;border-color:#88a8ff !important}",
    "html[data-theme=\"dark\"] .pkp_navigation_primary li.current{background:transparent !important}",
    "html[data-theme=\"dark\"] .longevity-platform-title{color:#fff !important}",
    "html[data-theme=\"dark\"] .section > h2,html[data-theme=\"dark\"] .section > h3,html[data-theme=\"dark\"] .sections .section > h2,html[data-theme=\"dark\"] .sections .section > h3{background:#15171f !important;color:#fff !important;border-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .section{border-top-color:#2a2f40 !important}",
    "html[data-theme=\"dark\"] .section::before,html[data-theme=\"dark\"] .section::after{border-color:#2a2f40 !important}"
  ].join("\n");

  function init(){
    document.head.appendChild(style);
    var div = document.createElement("div");
    div.innerHTML = html;
    document.body.insertBefore(div.firstChild, document.body.firstChild);
    var btn = document.querySelector(".theme-toggle-i");
    function syncIcon(){
      var dark = document.documentElement.getAttribute("data-theme") === "dark";
      btn.textContent = dark ? "☀" : "☾";
    }
    btn.addEventListener("click", function(){
      var dark = document.documentElement.getAttribute("data-theme") === "dark";
      if (dark) {
        document.documentElement.removeAttribute("data-theme");
        setTheme("light");
      } else {
        document.documentElement.setAttribute("data-theme","dark");
        setTheme("dark");
      }
      syncIcon();
    });
    syncIcon();
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();
