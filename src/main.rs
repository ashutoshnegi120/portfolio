use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

#[derive(Clone, Copy)]
struct Repo {
    name: &'static str,
    url: &'static str,
    language: &'static str,
    updated: &'static str,
    pitch: &'static str,
    detail: &'static str,
    tools: &'static [&'static str],
    skills: &'static [&'static str],
    system: &'static [&'static str],
}

#[derive(Clone, Copy)]
struct SkillCoin {
    name: &'static str,
    icon: &'static str,
    category: &'static str,
    info: &'static str,
}

const REPOS: &[Repo] = &[
    Repo {
        name: "Mock-E-Com-Cart",
        url: "https://github.com/ashutoshnegi120/Mock-E-Com-Cart",
        language: "JavaScript",
        updated: "2025",
        pitch: "VibeCart, a full-stack mock e-commerce cart.",
        detail: "React, Node.js, Express, and MongoDB shopping flow with product grid, add-to-cart actions, cart quantities, and checkout-style state. Good proof that you can build product-facing flows and connect them to backend basics.",
        tools: &["React", "Node.js", "Express", "MongoDB"],
        skills: &["REST APIs", "Cart state", "CRUD", "Auth basics"],
        system: &[
            "Client-server app",
            "Product catalog",
            "Cart persistence",
            "Checkout flow",
        ],
    },
    Repo {
        name: "frontend-eventSwap",
        url: "https://github.com/ashutoshnegi120/frontend-eventSwap",
        language: "JavaScript",
        updated: "2025",
        pitch: "SlotSwapper frontend for calendar event trades.",
        detail: "A React/Vite/Tailwind interface for trading calendar slots. It pairs with the backend API and focuses on clean user flows, real-time status, and a modern responsive web app experience.",
        tools: &["React", "Vite", "Tailwind", "Event API"],
        skills: &[
            "UI state",
            "API integration",
            "Responsive layout",
            "Realtime UX",
        ],
        system: &[
            "Frontend shell",
            "Swap request states",
            "Notification surface",
            "Calendar views",
        ],
    },
    Repo {
        name: "backend-eventSwap",
        url: "https://github.com/ashutoshnegi120/backend-eventSwap",
        language: "JavaScript",
        updated: "2025",
        pitch: "REST API for event management and swapping.",
        detail: "Node.js, Express, and MongoDB API with JWT auth, bcrypt password hashing, event CRUD, time conflict validation, SSE notifications, and MongoDB transactions for safer swaps.",
        tools: &["Node.js", "Express", "MongoDB", "SSE"],
        skills: &["JWT auth", "Transactions", "Validation", "API design"],
        system: &[
            "REST API",
            "Auth layer",
            "Swap marketplace",
            "Conflict detection",
            "Realtime notifications",
        ],
    },
    Repo {
        name: "olx-contract",
        url: "https://github.com/ashutoshnegi120/olx-contract",
        language: "Rust",
        updated: "2025",
        pitch: "Solana marketplace smart contract in Rust.",
        detail: "An OLX-style decentralized marketplace program on Solana. It explores listings, purchases, peer-to-peer escrow flow, and zero-copy architecture for better compute efficiency.",
        tools: &["Rust", "Solana", "Bytemuck", "On-chain accounts"],
        skills: &[
            "Smart contracts",
            "Escrow flow",
            "Zero-copy",
            "Account modeling",
        ],
        system: &[
            "Marketplace listings",
            "Purchase flow",
            "Escrow logic",
            "Compute-aware storage",
        ],
    },
    Repo {
        name: "Smart_Inventory_Management_With_AI_ML_integration",
        url: "https://github.com/ashutoshnegi120/Smart_Inventory_Management_With_AI_ML_integration",
        language: "TypeScript",
        updated: "2025",
        pitch: "AI/ML inventory platform with microservices.",
        detail: "A production-style SaaS idea using Rust microservices, Python AI/ML pipelines, and a React TypeScript frontend for forecasting, data processing, and business insights.",
        tools: &["Rust", "Python", "React", "TypeScript"],
        skills: &[
            "Microservices",
            "Forecasting",
            "Data pipelines",
            "SaaS design",
        ],
        system: &[
            "Tenant-aware services",
            "ML pipeline",
            "Inventory data flow",
            "Insight dashboard",
        ],
    },
    Repo {
        name: "voting_dapp",
        url: "https://github.com/ashutoshnegi120/voting_dapp",
        language: "TypeScript",
        updated: "2025",
        pitch: "Decentralized voting app experiment.",
        detail: "A Web3 voting project from your blockchain practice. It fits the hobby Solana/dApp track and shows that you are exploring product patterns beyond standard CRUD apps.",
        tools: &["TypeScript", "Web3", "Wallet flow", "dApp UI"],
        skills: &[
            "Voting logic",
            "Wallet UX",
            "State sync",
            "Frontend contracts",
        ],
        system: &[
            "Proposal flow",
            "Vote casting",
            "Result state",
            "User wallet boundary",
        ],
    },
    Repo {
        name: "solana-learning",
        url: "https://github.com/ashutoshnegi120/solana-learning",
        language: "Rust",
        updated: "2025",
        pitch: "Hands-on Solana learning playground.",
        detail: "Rust examples for Solana fundamentals: loading keypairs, working with client tools, async flows, and small focused experiments that build blockchain muscle memory.",
        tools: &["Rust", "Solana SDK", "Tokio", "Keypairs"],
        skills: &["Async Rust", "Wallet files", "RPC clients", "Chain basics"],
        system: &[
            "Learning modules",
            "Client scripts",
            "Keypair loading",
            "Small experiments",
        ],
    },
    Repo {
        name: "fps_Shooter-Unity",
        url: "https://github.com/ashutoshnegi120/fps_Shooter-Unity",
        language: "C#",
        updated: "2023",
        pitch: "FPS Zombies, a Unity shooter prototype.",
        detail: "A Unity hobby game project with first-person movement, sprinting, shooting, reloading, enemy AI, and health/damage systems. It shows your game-dev side without diluting the backend focus.",
        tools: &["Unity", "C#", "Game objects", "AI scripts"],
        skills: &["Movement", "Combat loop", "Enemy AI", "Health systems"],
        system: &[
            "Player controller",
            "Weapon loop",
            "Enemy behavior",
            "Damage model",
        ],
    },
];

const SKILLS: &[SkillCoin] = &[
    SkillCoin {
        name: "Rust",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-original.svg",
        category: "Language",
        info: "A systems language for fast, memory-safe backend services, CLIs, WebAssembly apps, and blockchain programs.",
    },
    SkillCoin {
        name: "Python",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/python/python-original.svg",
        category: "Language",
        info: "A productive language for APIs, scripting, automation, data work, and AI/ML pipelines.",
    },
    SkillCoin {
        name: "Go",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/go/go-original-wordmark.svg",
        category: "Language",
        info: "A simple, fast backend language used for APIs, networking services, concurrency, and cloud tooling.",
    },
    SkillCoin {
        name: "Actix Web",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-original.svg",
        category: "Backend",
        info: "A high-performance Rust web framework for building APIs and services with strong type safety.",
    },
    SkillCoin {
        name: "Leptos",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-original.svg",
        category: "Frontend",
        info: "A Rust web framework for building reactive frontends and full-stack web apps with WebAssembly.",
    },
    SkillCoin {
        name: "Elixir",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/elixir/elixir-original.svg",
        category: "Language",
        info: "A functional language built for fault-tolerant, concurrent systems on the BEAM virtual machine.",
    },
    SkillCoin {
        name: "Phoenix",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/phoenix/phoenix-original.svg",
        category: "Backend",
        info: "An Elixir framework for realtime web apps, APIs, channels, and highly concurrent services.",
    },
    SkillCoin {
        name: "Fiber",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/go/go-original.svg",
        category: "Backend",
        info: "A Go web framework inspired by Express, useful for quick and lightweight HTTP APIs.",
    },
    SkillCoin {
        name: "DevOps",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/docker/docker-original.svg",
        category: "Ops",
        info: "The practical skill of shipping, configuring, observing, and maintaining software reliably.",
    },
    SkillCoin {
        name: "PostgreSQL",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/postgresql/postgresql-original.svg",
        category: "Database",
        info: "A powerful relational database for transactional apps, structured data, indexing, and SQL-heavy backends.",
    },
    SkillCoin {
        name: "Docker",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/docker/docker-original.svg",
        category: "Ops",
        info: "Container tooling that packages apps and dependencies so services run consistently across machines.",
    },
    SkillCoin {
        name: "Linux",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/linux/linux-original.svg",
        category: "Ops",
        info: "The operating system foundation behind many servers, deployment targets, shells, and backend environments.",
    },
    SkillCoin {
        name: "TypeScript",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/typescript/typescript-original.svg",
        category: "Language",
        info: "JavaScript with types, useful for safer frontends, Node backends, and shared web app contracts.",
    },
    SkillCoin {
        name: "Solana",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-original.svg",
        category: "Blockchain",
        info: "A high-throughput blockchain where Rust programs and client apps can power decentralized products.",
    },
];

const HOBBIES: &[SkillCoin] = &[
    SkillCoin {
        name: "Unity",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/unity/unity-original.svg",
        category: "Game Engine",
        info: "A C# game engine for 2D, 3D, mobile, desktop, and rapid gameplay prototyping.",
    },
    SkillCoin {
        name: "Unreal Engine 5",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/unrealengine/unrealengine-original.svg",
        category: "Game Engine",
        info: "A high-end game engine known for 3D visuals, Blueprints, C++, and cinematic environments.",
    },
    SkillCoin {
        name: "Godot",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/godot/godot-original.svg",
        category: "Game Engine",
        info: "An open-source game engine for lightweight 2D/3D projects, scripting, and indie game experiments.",
    },
    SkillCoin {
        name: "Bevy",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-original.svg",
        category: "Game Engine",
        info: "A Rust game engine built around ECS patterns, data-driven systems, and native Rust workflows.",
    },
    SkillCoin {
        name: "Game Systems",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/csharp/csharp-original.svg",
        category: "Design",
        info: "The craft of building loops like movement, combat, inventory, enemies, progression, and feedback.",
    },
    SkillCoin {
        name: "Solana dApps",
        icon: "https://cdn.jsdelivr.net/gh/devicons/devicon/icons/rust/rust-original.svg",
        category: "Blockchain",
        info: "User-facing blockchain apps that connect wallets, frontend state, and on-chain programs.",
    },
];

fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let visits = increment_visits();
    let (flipped_repo, set_flipped_repo) = signal(None::<&'static str>);
    let (focused_repo, set_focused_repo) = signal(None::<&'static str>);
    let (selected_coin, set_selected_coin) = signal(Some(SKILLS[0].name));

    Effect::new(move |_| {
        if let Some(win) = window() {
            let _ = js_sys::Reflect::get(&win, &"startArcadePortfolio".into())
                .ok()
                .and_then(|start| start.dyn_into::<js_sys::Function>().ok())
                .map(|func| func.call0(&win));
        }
    });

    view! {
        <main class="game-shell">
            <div class="scanline-glow" aria-hidden="true"></div>
            {move || focused_repo.get().and_then(find_repo).map(|repo| {
                view! {
                    <section class="project-overlay" aria-modal="true" role="dialog">
                        <button class="overlay-close" type="button" on:click=move |_| set_focused_repo.set(None)>"Close"</button>
                        <article class="project-focus">
                            <header class="focus-summary">
                                <p class="eyebrow">"PROJECT INTEL"</p>
                                <h2>{repo.name}</h2>
                                <p>{repo.detail}</p>
                            </header>
                            <div class="focus-columns">
                                <section>
                                    <h3>"Tools Used"</h3>
                                    <div class="mini-chip-wall">
                                        {repo.tools.iter().map(|item| view! { <span>{*item}</span> }).collect_view()}
                                    </div>
                                </section>
                                <section>
                                    <h3>"Skills Needed"</h3>
                                    <div class="mini-chip-wall">
                                        {repo.skills.iter().map(|item| view! { <span>{*item}</span> }).collect_view()}
                                    </div>
                                </section>
                            </div>
                            <footer class="focus-system">
                                <h3>"System Design"</h3>
                                <div class="system-path">
                                    {repo.system.iter().map(|item| view! { <span>{*item}</span> }).collect_view()}
                                </div>
                                <a class="pixel-button primary" href={repo.url} target="_blank" rel="noreferrer">"Open Repository"</a>
                            </footer>
                        </article>
                    </section>
                }
            })}
            <section class="hero-level" id="home">
                <div class="sky-grid" aria-hidden="true">
                    <span class="cloud cloud-a"></span>
                    <span class="cloud cloud-b"></span>
                    <span class="coin coin-a"></span>
                    <span class="coin coin-b"></span>
                    <span class="coin coin-c"></span>
                </div>

                <nav class="top-hud" aria-label="Main navigation">
                    <span class="brand">"ASHUTOSH.EXE"</span>
                    <div class="nav-buttons">
                        <button type="button" data-warp="#projects">"Projects"</button>
                        <button type="button" data-warp="#skills">"Skills"</button>
                        <button type="button" data-warp="#contact">"Contact"</button>
                    </div>
                </nav>

                <div class="hero-grid">
                    <div class="dialogue-box">
                        <p class="eyebrow">"PLAYER 01 / BACKEND DEVELOPER"</p>
                        <h1>"Ashutosh Negi"</h1>
                        <p class="hero-copy">
                            "I build backend systems in Rust, Go, and Python. Actix Web, Leptos, Phoenix, Fiber, and practical DevOps are my main quest; game dev and Solana are my side quests."
                        </p>
                        <div class="hero-actions">
                            <a class="pixel-button primary" href="https://github.com/ashutoshnegi120" target="_blank" rel="noreferrer">
                                "Open GitHub"
                            </a>
                            <button type="button" class="pixel-button" data-warp="#projects">"Start Quest"</button>
                        </div>
                    </div>

                    <div class="player-card hud-panel" aria-label="Profile summary">
                        <div class="avatar-frame">
                            <img src="https://avatars.githubusercontent.com/u/97330349?v=4" alt="Ashutosh Negi GitHub avatar" />
                        </div>
                        <div class="hero-sprite" aria-hidden="true">
                            <span class="sprite-head"></span>
                            <span class="sprite-body"></span>
                            <span class="sprite-arm left"></span>
                            <span class="sprite-arm right"></span>
                            <span class="sprite-leg left"></span>
                            <span class="sprite-leg right"></span>
                        </div>
                        <dl class="stats-grid">
                            <div><dt>"Repos"</dt><dd>"24"</dd></div>
                            <div><dt>"Followers"</dt><dd>"3"</dd></div>
                            <div><dt>"Visits"</dt><dd>{visits}</dd></div>
                        </dl>
                    </div>
                </div>
            </section>

            <section class="level-section" id="projects">
                <div class="section-title">
                    <p>"LEVEL 02"</p>
                    <h2>"GitHub Quest Board"</h2>
                </div>
                <div class="repo-grid">
                    {REPOS.iter().map(|repo| {
                        let is_flipped = move || flipped_repo.get() == Some(repo.name);
                        view! {
                            <article
                                class:flipped=is_flipped
                                class="repo-card"
                                tabindex="0"
                                role="button"
                                aria-pressed=move || is_flipped().to_string()
                                on:click=move |_| {
                                    set_flipped_repo.update(|current| {
                                        *current = if *current == Some(repo.name) {
                                            None
                                        } else {
                                            Some(repo.name)
                                        };
                                    });
                                }
                            >
                                <div class="repo-card-inner">
                                    <div class="repo-face repo-front">
                                        <span class="repo-chip">{repo.language}</span>
                                        <h3>{repo.name}</h3>
                                        <p>{repo.pitch}</p>
                                        <div class="repo-footer">
                                            <span>{repo.updated}</span>
                                            <button type="button">"Brief"</button>
                                        </div>
                                    </div>
                                    <div class="repo-face repo-back">
                                        <span class="repo-chip">"Brief Intel"</span>
                                        <h3>"What it does"</h3>
                                        <p>{repo.detail}</p>
                                        <div class="repo-footer">
                                            <button type="button">"Back"</button>
                                            <button
                                                type="button"
                                                class="primary-card-action"
                                                on:click=move |ev| {
                                                    ev.stop_propagation();
                                                    set_focused_repo.set(Some(repo.name));
                                                }
                                            >
                                                "More Info"
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </article>
                        }
                    }).collect_view()}
                </div>
            </section>

            <section class="level-section split-level" id="skills">
                <div>
                    <div class="section-title">
                        <p>"LEVEL 03"</p>
                        <h2>"Skill Inventory"</h2>
                    </div>
                    <div class="skill-wall">
                        {SKILLS.iter().enumerate().map(|(index, coin)| {
                            let is_selected = move || selected_coin.get() == Some(coin.name);
                            let style = coin_style(index);
                            view! {
                                <button
                                    type="button"
                                    class:selected=is_selected
                                    class="skill-coin"
                                    style=style
                                    on:click=move |_| set_selected_coin.set(Some(coin.name))
                                >
                                    <span class="coin-face coin-icon">
                                        <img src={coin.icon} alt={coin.name} />
                                    </span>
                                    <span class="coin-face coin-name">{coin.name}</span>
                                </button>
                            }
                        }).collect_view()}
                    </div>
                    <div class="section-title hobby-title">
                        <p>"SIDE QUESTS"</p>
                        <h2>"Game + Chain Lab"</h2>
                    </div>
                    <div class="skill-wall hobby-wall">
                        {HOBBIES.iter().enumerate().map(|(index, coin)| {
                            let is_selected = move || selected_coin.get() == Some(coin.name);
                            let style = coin_style(index + SKILLS.len());
                            view! {
                                <button
                                    type="button"
                                    class:selected=is_selected
                                    class="skill-coin"
                                    style=style
                                    on:click=move |_| set_selected_coin.set(Some(coin.name))
                                >
                                    <span class="coin-face coin-icon">
                                        <img src={coin.icon} alt={coin.name} />
                                    </span>
                                    <span class="coin-face coin-name">{coin.name}</span>
                                </button>
                            }
                        }).collect_view()}
                    </div>
                </div>
                <aside class="quest-log hud-panel">
                    <h3>"Info Coin"</h3>
                    {move || selected_coin.get().and_then(find_coin).map(|coin| view! {
                        <div class="coin-info">
                            <span>{coin.category}</span>
                            <h4>{coin.name}</h4>
                            <p>{coin.info}</p>
                        </div>
                    })}
                    <div class="progress-track"><span></span></div>
                </aside>
            </section>

            <section class="contact-level" id="contact">
                <div class="dialogue-box compact">
                    <p class="eyebrow">"READY FOR CO-OP"</p>
                    <h2>"Let's build the next stage."</h2>
                    <p>"Open to backend roles, Rust/Go/Python work, API design, DevOps tasks, and projects where reliable systems still get a little game-like spark."</p>
                    <div class="hero-actions">
                        <a class="pixel-button primary" href="mailto:ashutoshnegi120@gmail.com">"Send Mail"</a>
                        <a class="pixel-button" href="https://github.com/ashutoshnegi120" target="_blank" rel="noreferrer">"GitHub"</a>
                    </div>
                </div>
            </section>
        </main>
    }
}

fn find_repo(name: &'static str) -> Option<&'static Repo> {
    REPOS.iter().find(|repo| repo.name == name)
}

fn find_coin(name: &'static str) -> Option<&'static SkillCoin> {
    SKILLS
        .iter()
        .chain(HOBBIES.iter())
        .find(|coin| coin.name == name)
}

fn coin_style(index: usize) -> String {
    let delay = (index * 371) % 3100;
    let duration = 4200 + ((index * 619) % 2600);
    let float_delay = (index * 193) % 1800;
    format!(
        "--coin-delay: {delay}ms; --coin-duration: {duration}ms; --float-delay: {float_delay}ms;"
    )
}

fn increment_visits() -> u32 {
    let Some(storage) = window().and_then(|win| win.local_storage().ok()).flatten() else {
        return 1;
    };

    let next = storage
        .get_item("ashutosh_portfolio_visits")
        .ok()
        .flatten()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(0)
        .saturating_add(1);

    let _ = storage.set_item("ashutosh_portfolio_visits", &next.to_string());
    next
}
