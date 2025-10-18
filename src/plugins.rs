// Static list of known plugins used by the GUI to render one-click install options.
// This is intentionally lightweight and can be generated or loaded from a remote
// source in the future.
pub struct Plugin {
    pub name: &'static str,
    pub author: &'static str,
    pub description: &'static str,
    pub repo: &'static str,
}

pub const GENERAL_PLUGINS: &[Plugin] = &[
    Plugin { name: "BrowserSearch", author: "TBM13", description: "Search your browser history", repo: "https://github.com/TBM13/BrowserSearch" },
    Plugin { name: "GEmojiSharp", author: "hlaueriksson", description: "Search GitHub Emoji", repo: "https://github.com/hlaueriksson/GEmojiSharp" },
    Plugin { name: "PowerTranslator", author: "N0I0C0K", description: "Text translator based on Youdao", repo: "https://github.com/N0I0C0K/PowerTranslator" },
    Plugin { name: "QuickLookup", author: "GTGalaxi", description: "Search across multiple security/dev tools", repo: "https://github.com/GTGalaxi/quick-lookup-ptrun" },
    Plugin { name: "InputTyper", author: "CoreyHayward", description: "Type the input as if sent from a keyboard", repo: "https://github.com/CoreyHayward/PowerToys-Run-InputTyper" },
    Plugin { name: "ClipboardManager", author: "CoreyHayward", description: "Search and paste text from clipboard history", repo: "https://github.com/CoreyHayward/PowerToys-Run-ClipboardManager" },
    Plugin { name: "CurrencyConverter", author: "Advaith3600", description: "Convert real and crypto currencies", repo: "https://github.com/Advaith3600/PowerToys-Run-Currency-Converter" },
    Plugin { name: "FastWeb", author: "CCcat", description: "Open website in browser quickly", repo: "https://github.com/CCcat8059/FastWeb" },
    Plugin { name: "WebSearchShortcut", author: "Riri", description: "Select a specific search engine to perform searches", repo: "https://github.com/Daydreamer-riri/CmdPal-WebSearchShortcut" },
    Plugin { name: "UnicodeInput", author: "nathancartlidge", description: "Copy Unicode characters to the clipboard", repo: "https://github.com/nathancartlidge/powertoys-run-unicode" },
    Plugin { name: "PowerHexInspector", author: "NaroZeol", description: "Peek other forms of an input number", repo: "https://github.com/NaroZeol/PowerHexInspector" },
    Plugin { name: "GitHubRepo", author: "8LWXpg", description: "Search and open GitHub repositories", repo: "https://github.com/8LWXpg/PowerToysRun-GitHubRepo" },
    Plugin { name: "ProcessKiller", author: "8LWXpg", description: "Search and kill processes", repo: "https://github.com/8LWXpg/PowerToysRun-ProcessKiller" },
    Plugin { name: "ChatGPT", author: "ferraridavide", description: "Ask a question to ChatGPT", repo: "https://github.com/ferraridavide/ChatGPTPowerToys" },
    Plugin { name: "CanIUse", author: "skttl", description: "Look up browser feature support with caniuse.com", repo: "https://github.com/skttl/ptrun-caniuse" },
    Plugin { name: "TailwindCSS", author: "skttl", description: "Search TailwindCSS documentation", repo: "https://github.com/skttl/ptrun-tailwindcss" },
    Plugin { name: "HttpStatusCodes", author: "grzhan", description: "Search for http status codes", repo: "https://github.com/grzhan/HttpStatusCodePowerToys" },
    Plugin { name: "SVGL", author: "SameerJS6", description: "Search, Browse and copy SVG logos from SVGL", repo: "https://github.com/SameerJS6/powertoys-svgl" },
    Plugin { name: "QuickNotes", author: "ruslanlap", description: "Create, manage, and search notes directly from PowerToys Run.", repo: "https://github.com/ruslanlap/CommunityPowerToysRunPlugin-QuickNotes" },
    Plugin { name: "Weather", author: "ruslanlap", description: "Get real-time weather information directly from PowerToys Run.", repo: "https://github.com/ruslanlap/PowerToysRun-Weather" },
    Plugin { name: "Pomodoro", author: "ruslanlap", description: "Manage Pomodoro productivity sessions directly from PowerToys Run.", repo: "https://github.com/ruslanlap/PowerToysRun-Pomodoro" },
    Plugin { name: "Definition", author: "ruslanlap", description: "Lookup word definitions, phonetics, and synonyms directly in PowerToys Run.", repo: "https://github.com/ruslanlap/PowerToysRun-Definition" },
    Plugin { name: "Hotkeys", author: "ruslanlap", description: "Create, manage, and trigger custom keyboard shortcuts directly from PowerToys Run.", repo: "https://github.com/ruslanlap/PowerToysRun-Hotkeys" },
    Plugin { name: "RandomGen", author: "ruslanlap", description: "Generate random data instantly with a single keystroke.", repo: "https://github.com/ruslanlap/PowerToysRun-RandomGen" },
    Plugin { name: "OpenWithCursor", author: "VictorNoxx", description: "Open Visual Studio, VS Code recents with Cursor AI integration.", repo: "https://github.com/VictorNoxx/PowerToys-Run-Cursor" },
    Plugin { name: "CheatSheets", author: "ruslanlap", description: "Find cheat sheets and command examples instantly.", repo: "https://github.com/ruslanlap/PowerToysRun-CheatSheets" },
];

pub const EXTENDING_PLUGINS: &[Plugin] = &[
    Plugin { name: "EdgeFavorite", author: "davidegiacometti", description: "Open Microsoft Edge favorites", repo: "https://github.com/davidegiacometti/PowerToys-Run-EdgeFavorite" },
    Plugin { name: "EdgeWorkspaces", author: "quachpas", description: "Open Microsoft Edge workspaces", repo: "https://github.com/quachpas/PowerToys-Run-EdgeWorkspaces" },
    Plugin { name: "Everything", author: "Yu Chieh (Victor) Lin", description: "Get search results from Everything", repo: "https://github.com/lin-ycv/EverythingPowerToys" },
    Plugin { name: "GitKraken", author: "davidegiacometti", description: "Open GitKraken repositories", repo: "https://github.com/davidegiacometti/PowerToys-Run-GitKraken" },
    Plugin { name: "RDP", author: "anthony81799", description: "Open Remote Desktop connections", repo: "https://github.com/anthony81799/PowerToysRun-RDP" },
    Plugin { name: "VisualStudioRecents", author: "davidegiacometti", description: "Open Visual Studio recents", repo: "https://github.com/davidegiacometti/PowerToys-Run-VisualStudio" },
    Plugin { name: "WinGet", author: "bostrot", description: "Search and install packages from WinGet", repo: "https://github.com/bostrot/PowerToysRunPluginWinget" },
    Plugin { name: "Scoop", author: "Quriz", description: "Search and install packages from Scoop", repo: "https://github.com/Quriz/PowerToysRunScoop" },
    Plugin { name: "Spotify", author: "waaverecords", description: "Search Spotify and control its player", repo: "https://github.com/waaverecords/PowerToys-Run-Spotify" },
    Plugin { name: "PowerSearch1Password", author: "KairuDeibisu", description: "Unofficial plugin for searching 1Password", repo: "https://github.com/KairuDeibisu/PowerToysRunPlugin1Password" },
    Plugin { name: "HackMD", author: "8LWXpg", description: "Open HackMD notes", repo: "https://github.com/8LWXpg/PowerToysRun-HackMD" },
    Plugin { name: "SSH", author: "8LWXpg", description: "Connect to ssh clients", repo: "https://github.com/8LWXpg/PowerToysRun-SSH" },
    Plugin { name: "Bilibili", author: "Whuihuan", description: "Use AVID or BVID to parse and jump to Bilibili", repo: "https://github.com/Whuihuan/PowerToysRun-Bilibili" },
    Plugin { name: "YubicoOauthOTP", author: "dlnilsson", description: "Display generated codes from OATH accounts stored on the YubiKey", repo: "https://github.com/dlnilsson/Community.PowerToys.Run.Plugin.YubicoOauthOTP" },
    Plugin { name: "FirefoxBookmark", author: "8LWXpg", description: "Open bookmarks in Firefox based browser", repo: "https://github.com/8LWXpg/PowerToysRun-FirefoxBookmark" },
    Plugin { name: "Linear", author: "vednig", description: "Create Linear Issues directly from PowerToys Run", repo: "https://github.com/vednig/powertoys-linear" },
    Plugin { name: "PerplexitySearchShortcut", author: "0x6f677548", description: "Search Perplexity", repo: "https://github.com/0x6f677548/PowerToys-Run-PerplexitySearchShortcut" },
    Plugin { name: "SpeedTest", author: "ruslanlap", description: "One-command internet speed tests", repo: "https://github.com/ruslanlap/PowerToysRun-SpeedTest" },
];
