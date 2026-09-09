//! Static content for the map. This is the ONLY file that changes when the
//! resume changes — nothing here is computed, and nothing else hardcodes copy.

#[derive(Clone, Copy, PartialEq)]
pub enum Kind {
    Employment,
    Product,
    Origin,
    Capture,
}

impl Kind {
    pub fn color(self) -> &'static str {
        match self {
            Kind::Employment => "#F2A93B",
            Kind::Product => "#5CC7D8",
            Kind::Origin => "#A78BFA",
            Kind::Capture => "#F2726F",
        }
    }
}

pub struct Moon {
    pub name: &'static str,
    /// Orbit radius as a fraction of the scene scale.
    pub dist: f32,
    pub speed: f32,
    pub size: f32,
}

pub struct Body {
    pub id: &'static str,
    pub name: &'static str,
    pub role: &'static str,
    pub when: &'static str,
    pub kind: Kind,
    /// Hours invested, relative. Drives radius — never a self-rating.
    pub mass: f32,
    /// Semi-major axis as a fraction of the scene scale.
    pub a: f32,
    /// Eccentricity. A recent capture has not circularised yet.
    pub e: f32,
    /// Rotation of the ellipse in the plane (argument of periapsis).
    pub tilt: f32,
    /// Starting eccentric anomaly.
    pub phase: f32,
    /// Orbital plane tilt in radians. Shell = kind: employment ~0.4, product ~1.2, origin ~0.8, capture ~1.4.
    pub inclination: f32,
    /// Rotation of the orbital plane around the y-axis (ascending node).
    pub ascending_node: f32,
    pub bullets: &'static [&'static str],
    pub chips: &'static [&'static str],
    pub moons: &'static [Moon],
}

pub const BODIES: &[Body] = &[
    Body {
        id: "newengen",
        name: "NewEngen",
        role: "Software Engineer, AI Platform",
        when: "Sep 2024 — now",
        kind: Kind::Employment,
        mass: 9.4,
        a: 0.30,
        e: 0.05,
        tilt: 0.4,
        phase: 0.2,
        inclination: 0.40,
        ascending_node: 0.0,
        bullets: &[
            "Designed and shipped LIFT AI — Google Ads, Meta and Shopify data turned into analyst-grade reports, replacing hours of manual reporting per account each week.",
            "Owns a Redis-backed distributed concurrency layer: atomic execution gates, heartbeat monitoring, three pools with differentiated TTLs across a multi-pod Kubernetes deployment.",
            "Eliminated a class of LLM output failures by tracing phantom dimension values and broken pacing tables to tool-layer key mismatches; shipped a three-tier normalisation fallback.",
            "Built LIFT AI MFE, the micro-frontend shell (React, Module Federation, TypeScript) hosting the platform inside NewEngen\u{2019}s existing app: independently deployable, with live SSE streaming so users watch analysis unfold rather than waiting on a spinner.",
            "Owned delivery of a data taxonomy tool serving 150+ international brands (Python, GraphQL, PostgreSQL over large-scale pipelines), working directly with marketing strategists to shape the model around how they name and group campaigns.",
        ],
        chips: &["Claude", "Agno", "FastAPI", "BigQuery", "Vertex AI", "Kubernetes", "Redis"],
        moons: &[
            Moon { name: "LIFT AI", dist: 0.052, speed: 2.6, size: 4.2 },
            Moon { name: "LIFT AI MFE", dist: 0.078, speed: 1.7, size: 3.4 },
            Moon { name: "Taxonomy tool", dist: 0.100, speed: 1.2, size: 3.0 },
        ],
    },
    Body {
        id: "lia",
        name: "Lia",
        role: "Local-first personal AI · solo",
        when: "2025 — now",
        kind: Kind::Product,
        mass: 7.8,
        a: 0.37,
        e: 0.16,
        tilt: 5.6,
        phase: 4.2,
        inclination: 1.20,
        ascending_node: 0.8,
        bullets: &[
            "33k lines written alone: hub service with ten tool packages and an append-only Postgres event log with semantic recall for lifelong memory.",
            "Capability-based permission engine graded by reversibility — read, write and shell actions require confirmation in proportion to how hard they are to undo.",
        ],
        chips: &["Python", "PostgreSQL", "device mesh", "RAG"],
        moons: &[Moon { name: "Device mesh", dist: 0.048, speed: 2.2, size: 3.4 }],
    },
    Body {
        id: "greenliving",
        name: "GreenLiving",
        role: "Founding Software Engineer",
        when: "May — Aug 2024",
        kind: Kind::Employment,
        mass: 4.6,
        a: 0.44,
        e: 0.10,
        tilt: 2.1,
        phase: 2.6,
        inclination: 0.44,
        ascending_node: 2.0,
        bullets: &[
            "First engineer on the product: founders' requirements from whiteboard to deployed application, owning backend, frontend and infrastructure with no spec to hand off to.",
            "Built the FastAPI backend, translated Figma into production ReactJS, and ran end-to-end AWS deployment with Docker and CI/CD without a dedicated ops team.",
        ],
        chips: &["FastAPI", "ReactJS", "ViteJS", "Chakra UI", "AWS", "Docker"],
        moons: &[],
    },
    Body {
        id: "dawnn",
        name: "Dawnn",
        role: "Solo product · AI life OS",
        when: "2025 — now",
        kind: Kind::Product,
        mass: 5.4,
        a: 0.50,
        e: 0.13,
        tilt: 0.9,
        phase: 0.9,
        inclination: 1.15,
        ascending_node: 3.5,
        bullets: &[
            "\"The only app that notices things about you that you haven't noticed yourself.\" Pivoted toward retention mechanics; live at getdawnn.app.",
        ],
        chips: &["React Native", "Expo", "FastAPI", "Supabase", "Mem0"],
        moons: &[],
    },
    Body {
        id: "reliable",
        name: "Reliable AI",
        role: "Backend & DevOps Intern",
        when: "Feb — Jun 2024",
        kind: Kind::Employment,
        mass: 3.4,
        a: 0.56,
        e: 0.07,
        tilt: 4.4,
        phase: 5.0,
        inclination: 0.38,
        ascending_node: 4.2,
        bullets: &[
            "Architected backend infrastructure in NestJS with Kafka for real-time streaming and Grafana for BI.",
            "Cut AWS infrastructure cost 30% through optimised deployment pipelines and resource scaling.",
        ],
        chips: &["NestJS", "Kafka", "Grafana", "AWS"],
        moons: &[],
    },
    Body {
        id: "upgrad",
        name: "UpGrad",
        role: "DevOps Consultant",
        when: "Oct 2023 — Feb 2024",
        kind: Kind::Employment,
        mass: 2.6,
        a: 0.66,
        e: 0.12,
        tilt: 1.2,
        phase: 1.4,
        inclination: 0.50,
        ascending_node: 5.5,
        bullets: &[
            "Advised engineering teams on CI/CD, cloud-native deployment and Agile adoption, adapting guidance per team's existing stack.",
        ],
        chips: &["CI/CD", "Cloud-native", "Agile"],
        moons: &[],
    },
    Body {
        id: "rust",
        name: "Rust / WASM",
        role: "Recent capture",
        when: "2026 —",
        kind: Kind::Capture,
        mass: 1.4,
        a: 0.72,
        e: 0.55,
        tilt: 5.0,
        phase: 1.8,
        inclination: 1.40,
        ascending_node: 2.0,
        bullets: &[
            "Newest body in the system, still on a long ellipse. An honest thing for a website to say out loud.",
        ],
        chips: &["wasm-bindgen", "wgpu", "trunk", "Cloudflare Workers"],
        moons: &[],
    },
    Body {
        id: "traveey",
        name: "Traveey",
        role: "Founding Backend & DevOps Engineer",
        when: "Apr — Aug 2023",
        kind: Kind::Employment,
        mass: 2.4,
        a: 0.79,
        e: 0.09,
        tilt: 3.3,
        phase: 3.4,
        inclination: 0.42,
        ascending_node: 1.1,
        bullets: &[
            "Built core backend services for a fast-scaling travel-tech startup: secure password hashing, notification services, CI/CD on GitHub Actions and EC2 deployment.",
        ],
        chips: &["NestJS", "MongoDB", "GitHub Actions", "EC2"],
        moons: &[],
    },
    Body {
        id: "iiit",
        name: "IIIT Bhopal",
        role: "B.Tech · President, GNU/Linux Users Club",
        when: "Dec 2021 — Jun 2025",
        kind: Kind::Origin,
        mass: 4.0,
        a: 0.90,
        e: 0.06,
        tilt: 2.7,
        phase: 2.0,
        inclination: 0.79,
        ascending_node: 3.0,
        bullets: &["Ran 10+ workshops on system design and backend architecture as club president."],
        chips: &["B.Tech", "GNU/Linux Users Club"],
        moons: &[],
    },
];

/// Skill belt. `group` indexes SKILL_COLORS.
pub struct Skill {
    pub name: &'static str,
    pub group: usize,
}

pub const SKILL_COLORS: &[&str] = &["#7FD1A4", "#5CC7D8", "#F2A93B", "#A78BFA", "#8FA6D8", "#D8A0C8"];

pub const SKILLS: &[Skill] = &[
    Skill { name: "Python", group: 0 },
    Skill { name: "TypeScript", group: 0 },
    Skill { name: "Go", group: 0 },
    Skill { name: "Rust", group: 0 },
    Skill { name: "C/C++", group: 0 },
    Skill { name: "SQL", group: 0 },
    Skill { name: "Claude APIs", group: 1 },
    Skill { name: "Agno", group: 1 },
    Skill { name: "Vertex AI", group: 1 },
    Skill { name: "RAG", group: 1 },
    Skill { name: "Evals", group: 1 },
    Skill { name: "FastAPI", group: 2 },
    Skill { name: "NodeJS", group: 2 },
    Skill { name: "NestJS", group: 2 },
    Skill { name: "GraphQL", group: 2 },
    Skill { name: "RabbitMQ", group: 2 },
    Skill { name: "Redis", group: 2 },
    Skill { name: "SSE", group: 2 },
    Skill { name: "React", group: 3 },
    Skill { name: "Module Federation", group: 3 },
    Skill { name: "Vite", group: 3 },
    Skill { name: "BigQuery", group: 3 },
    Skill { name: "PostgreSQL", group: 3 },
    Skill { name: "MongoDB", group: 3 },
    Skill { name: "ETL", group: 3 },
    Skill { name: "Kubernetes", group: 4 },
    Skill { name: "Docker", group: 4 },
    Skill { name: "Terraform", group: 4 },
    Skill { name: "Helm", group: 4 },
    Skill { name: "GCP", group: 4 },
    Skill { name: "AWS", group: 4 },
    Skill { name: "Azure", group: 4 },
    Skill { name: "ArgoCD", group: 4 },
    Skill { name: "Jenkins", group: 4 },
    Skill { name: "Grafana", group: 5 },
    Skill { name: "Prometheus", group: 5 },
    Skill { name: "OpenTelemetry", group: 5 },
    Skill { name: "Superset", group: 5 },
];
