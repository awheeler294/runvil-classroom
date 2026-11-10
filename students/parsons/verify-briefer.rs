//! End-to-end verification of `students/parsons/prompts/briefer.md`:
//! 1. Parses the prompt file with `Host::parse` (verifying YAML frontmatter,
//!    H1/section hierarchy, fence rules, and Lua 5.5 bytecode compilation).
//! 2. Executes the full 13-step Briefer pipeline through `Harness::run`,
//!    exercising H1 setup, `require '## helpers'`, `prompt.section`,
//!    `tools.offer({"web/search", "web/fetch"})`, `tasks.fanout` over the 3
//!    diagnostic battery groups, `models.use("fast")` in Step 9, the 5
//!    sequential output batches with retry validation, the citation audit,
//!    and VFS store file generation.

use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

use harness::pack::{PackId, PackRegistry, Packs};
use harness::record::{MemoryRecorder, RecordKind};
use harness::vfs::{Origin, VfsRef};
use harness::{BoxFuture, Harness, Host, InferenceBroker, RunSetup, Timer};
use runvil::effect::Round;
use runvil::model::{
    Completion, CompletionOptions, CompletionResult, Message, ModelBinding, ModelCatalog, ModelId,
    ModelInfo, ThinkingMode, ToolSchema,
};
use runvil::{Error, ErrorCode, ToolId, ToolInfo, ToolOutput};
use runvil_plugin::{Pack, PackFuture, PackInfo, ToolContext};
use serde_json::{Value, json};

const DEFAULT_BRIEFER_PATH: &str =
    "/Users/sean/Development/runvil-classroom/students/parsons/prompts/briefer.md";
const DEFAULT_OUTPUT_DIR: &str =
    "/Users/sean/Development/runvil-classroom/students/parsons/outputs";

struct MockWebPack;

impl Pack for MockWebPack {
    fn tools(&self) -> Vec<ToolInfo> {
        let search = ToolInfo::new(
            ToolId::parse("web/search").expect("valid tool id"),
            "Search the web",
            json!({"type": "object", "properties": {"query": {"type": "string"}}}),
        );
        let fetch = ToolInfo::new(
            ToolId::parse("web/fetch").expect("valid tool id"),
            "Fetch a URL",
            json!({"type": "object", "properties": {"url": {"type": "string"}}}),
        );
        vec![search, fetch]
    }

    fn call<'a>(
        &'a self,
        _cx: ToolContext<'a>,
        _args: Value,
    ) -> PackFuture<'a, Result<ToolOutput, Error>> {
        Box::pin(async move {
            Ok(ToolOutput::untrusted(
                "{\"query\":\"iso cpp\",\"results\":[{\"title\":\"WG21 Charter\",\"url\":\"https://isocpp.org/std/the-committee\",\"description\":\"ISO/IEC JTC1/SC22/WG21 structure\"}]}",
            ))
        })
    }
}

struct BrieferBroker {
    catalog: ModelCatalog,
}

impl InferenceBroker for BrieferBroker {
    fn models(&self) -> BoxFuture<Result<ModelCatalog, Error>> {
        let catalog = self.catalog.clone();
        Box::pin(async move { Ok(catalog) })
    }

    fn chat(
        &self,
        binding: ModelBinding,
        messages: Vec<Message>,
        _tools: Vec<ToolSchema>,
        _options: CompletionOptions,
        _round: Round,
    ) -> BoxFuture<Result<Box<Completion>, Error>> {
        let prompt = messages
            .last()
            .map(|m| m.content().to_owned())
            .unwrap_or_default();
        let model_name = binding.id().name().to_owned();

        let reply = if prompt.contains("Survey the following subject") {
            json!({
                "name": "ISO C++ Committee (WG21)",
                "slug": "iso-cpp-committee",
                "subject_type": "institution",
                "founder": "Bjarne Stroustrup",
                "mission": "Standardize and evolve the C++ programming language under ISO/IEC JTC1/SC22/WG21.",
                "structure": "Consensus-driven ISO working group with national body voting and subgroup pipeline (EWG, LEWG, CWG, LWG).",
                "age_and_scale": "Founded 1989; ~300 active attendees per meeting.",
                "domain": "Programming Language Standardization",
                "urls": ["https://isocpp.org/std/the-committee"],
                "gaps": ["Funding ratio between corporate sponsors and volunteers"]
            })
            .to_string()
        } else if prompt.contains("Conduct reconnaissance on the subject") {
            concat!(
                "### Subject Profile\nWG21 standardizes C++ on a three-year release train.\n\n",
                "### Domain Primer\nSystems languages face high switching costs and backwards-compatibility lock-in.\n\n",
                "### Domain Landscape\nMarket Structure Classification: monopoly credentialing body with emerging ecosystem substitutes (Rust, Carbon).\n\n",
                "### Public Record\nDebates over ABI stability, safety profiles, and governance velocity.\n\n",
                "### Domain-Specific Vulnerabilities\nVolunteer fatigue and corporate sponsor concentration.\n\n",
                "## Source Log\n- [The Committee - isocpp.org](https://isocpp.org/std/the-committee)\n"
            )
            .to_owned()
        } else if prompt.contains("Build the theoretical foundation") {
            concat!(
                "### Per-Report Rules\n- **Property:** ABI Lock-in\n- **Cluster:** Market Structure\n- **Cite:** Farrell, J. and Saloner, G. 1985.\n\n",
                "### Theoretical Foundation\n- Olson, M. *The Logic of Collective Action.* Harvard University Press, 1965.\n\n",
                "## Source Log\n- [WG21 Papers - open-std.org](https://www.open-std.org/jtc1/sc22/wg21/)\n"
            )
            .to_owned()
        } else if prompt.contains("Audit the assumptions in the consolidated reconnaissance") {
            json!({
                "sufficient": true,
                "insufficiency_notice": "",
                "questions": [],
                "unresolved_assumptions": ["Exact share of travel funding covered by top 5 corporate employers"]
            })
            .to_string()
        } else if prompt.contains("Run every diagnostic test in this group") {
            json!({
                "tests": [
                    {
                        "id": "1. Live Player",
                        "cluster": "Power and Governance",
                        "verdict": "finding",
                        "confidence": "high",
                        "detail": "Consensus procedures require multi-stage subgroup approval that blocks rapid structural pivots.",
                        "finding": "Veto dispersion across subgroups prevents unified live-player steering.",
                        "gap": "does not evaluate whether participants outside central power develop learned helplessness"
                    },
                    {
                        "id": "21. Lock-in / Switching Costs",
                        "cluster": "Market Structure",
                        "verdict": "finding",
                        "confidence": "high",
                        "detail": "Billions of lines of legacy C++ code create extreme switching costs for downstream adopters.",
                        "finding": "Structural code-base lock-in insulates the committee from immediate exit.",
                        "gap": "does not evaluate whether participants deepen lock-in through compensating workarounds"
                    }
                ]
            })
            .to_string()
        } else if prompt.contains("stress-test every candidate finding")
            || prompt.contains("Stress-test every candidate finding")
        {
            json!({
                "surviving": [
                    {
                        "test": "1. Live Player",
                        "cluster": "Power and Governance",
                        "confidence": "high",
                        "finding": "Veto dispersion across subgroups prevents unified live-player steering.",
                        "gap": "does not evaluate whether participants outside central power develop learned helplessness",
                        "downgraded": false
                    },
                    {
                        "test": "21. Lock-in / Switching Costs",
                        "cluster": "Market Structure",
                        "confidence": "high",
                        "finding": "Structural code-base lock-in insulates the committee from immediate exit.",
                        "gap": "does not evaluate whether participants deepen lock-in through compensating workarounds",
                        "downgraded": false
                    }
                ],
                "killed": [
                    {
                        "test": "2. Niche",
                        "challenge_stage": "1. The subject already handles it",
                        "reason": "Downstream compiler vendors immediately implement published standards."
                    }
                ],
                "counts": {
                    "tests_run": 45,
                    "tests_findings": 6,
                    "tests_killed": 1,
                    "tests_downgraded": 0,
                    "rules_generated": 3,
                    "rules_survived": 2,
                    "theories_applied": 3,
                    "theories_confirmed": 2,
                    "theories_partial": 1,
                    "theories_falsified": 0
                }
            })
            .to_string()
        } else if prompt.contains("Execute Step 8 (Dark Influence)") {
            json!({
                "markdown": "# Dark Influence\n\n## Source Log\n- [Compiler Ecosystem - isocpp.org](https://isocpp.org/get-started)\n",
                "dark_breadcrumbs": [
                    {
                        "test": "Dark Influence",
                        "cluster": "Market Structure",
                        "confidence": "medium",
                        "finding": "The demand for complexity management is satisfied by specialized consultancy and tooling vendors, who benefit from specification complexity persisting.",
                        "gap": ""
                    }
                ],
                "demand_count": 2,
                "candidate_count": 2,
                "survived_count": 1
            })
            .to_string()
        } else if prompt.contains("Determine the directional trajectory") {
            json!({
                "markdown": "# Directional Research\n\n- **1. Live Player:** degrading over 2020-2026.\n",
                "enriched_breadcrumbs": [
                    {
                        "test": "1. Live Player",
                        "cluster": "Power and Governance",
                        "confidence": "high",
                        "finding": "Veto dispersion across subgroups prevents unified live-player steering.",
                        "gap": "does not evaluate whether participants outside central power develop learned helplessness",
                        "direction": "degrading",
                        "direction_evidence": "Paper volume grew faster than plenary throughput.",
                        "timeframe": "2020-2026"
                    },
                    {
                        "test": "21. Lock-in / Switching Costs",
                        "cluster": "Market Structure",
                        "confidence": "high",
                        "finding": "Structural code-base lock-in insulates the committee from immediate exit.",
                        "gap": "does not evaluate whether participants deepen lock-in through compensating workarounds",
                        "direction": "stable",
                        "direction_evidence": "Installed C++ codebase remains entrenched across finance and games.",
                        "timeframe": "2020-2026"
                    }
                ],
                "degrading": 1,
                "stable": 1,
                "improving": 0
            })
            .to_string()
        } else if prompt.contains("Here are the surviving diagnostic breadcrumbs, organized by cluster") {
            "## Coupling Map\n\n### The Consensus Lock-in Loop\n- **Constituents:** 1. Live Player + 21. Lock-in / Switching Costs\n- **Mechanism:** High downstream switching costs insulate the committee from exit while consensus veto dispersion prevents structural reform.\n- **Trajectory:** Degrading.\n".to_owned()
        } else if prompt.contains("Review the coupling map below before it enters Synthesis") {
            json!({
                "annotated_markdown": "## Coupling Map (Challenged)\n\n### The Consensus Lock-in Loop [SURVIVED]\n",
                "surviving_compounds": [
                    {
                        "name": "The Consensus Lock-in Loop",
                        "kind": "cross-cluster",
                        "constituents": ["1. Live Player", "21. Lock-in / Switching Costs"],
                        "mechanism": "Downstream switching costs prevent exit while consensus vetoes block architectural pivots.",
                        "consequence": "Incremental feature accretion without architectural simplification.",
                        "direction": "degrading",
                        "confidence": "high",
                        "gap_derived": "Participants build local dialect subsets while deferring committee-wide reform."
                    }
                ],
                "killed_compounds": [],
                "within_cluster_count": 0,
                "cross_cluster_count": 1,
                "gap_derived_count": 1,
                "killed_count": 0
            })
            .to_string()
        } else if prompt.contains("Synthesize the surviving findings") {
            json!({
                "characterization": "Consensus standards body insulated by downstream switching costs while veto dispersion slows structural adaptation.",
                "verdict_sentence": "Functional institution with eroding adaptive capacity under consensus veto dispersion.",
                "prognosis": "Functional",
                "dominant_dynamic": "The Consensus Lock-in Loop",
                "historical_parallel": "INCITS / ANSI X3J11",
                "thesis": "WG21 remains functional because its three-year release train ships standards that every major compiler implements, but downstream switching costs insulate the committee from competitive exit while subgroup veto dispersion prevents unified architectural pivots.",
                "section_4_plan": ["The Consensus Lock-in Loop"],
                "standalone_institutional": ["1. Live Player"],
                "standalone_economic": ["21. Lock-in / Switching Costs"],
                "standalone_external": [],
                "predictions": {
                    "short_term": "If C++26 ships on schedule, compiler adoption continues. If not, vendor extensions diverge (high).",
                    "medium_term": "If safety profiles reach consensus, regulatory pressure eases. If not, greenfield systems migrate (medium).",
                    "long_term": "If succession into subgroup chairs broadens, throughput stabilizes. If not, maintenance dominates (medium)."
                },
                "include_section_5": true,
                "include_section_6": true,
                "include_section_7": false
            })
            .to_string()
        } else if prompt.contains("Write Batch 1") {
            concat!(
                "# Briefer: ISO C++ Committee (WG21)\n\n",
                "**Consensus standards body insulated by downstream switching costs while veto dispersion slows structural adaptation.**\n\n",
                "Functional institution with eroding adaptive capacity under consensus veto dispersion.\n\n",
                "November 2026\n\n---\n\n",
                "## 1. Executive Summary\nWG21 ships standards on a predictable three-year cadence. Veto dispersion across subgroups combined with high downstream switching costs favors additive complexity over structural simplification.\n\n---\n\n",
                "## 2. The Subject\nFounded in 1989 by Bjarne Stroustrup, [The Committee - isocpp.org](https://isocpp.org/std/the-committee) operates as ISO/IEC JTC1/SC22/WG21.\n\n---\n\n",
                "## 3. The Landscape\n### Market position\nMonopoly credentialing body for standard C++.\n\n---"
            )
            .to_owned()
        } else if prompt.contains("Write Batch 2") {
            concat!(
                "## 4. Structural Assessment\n\n",
                "### The Consensus Lock-in Loop\nSwitching costs (Klemperer 1987) across billions of lines of production code prevent downstream exit while multi-stage subgroup consensus disperses live player (Burja 2020) authority. The specification accretes features faster than it simplifies.\n\n---"
            )
            .to_owned()
        } else if prompt.contains("Write Batch 3") {
            concat!(
                "## 5. Institutional Health\nThe prognosis is Functional. The three-year release train delivers ratified standards, though coordination rests on distributed subgroup chairs.\n\n---\n\n",
                "## 6. Economic Position\nSwitching costs anchor enterprise users to the standard toolchain.\n\n---"
            )
            .to_owned()
        } else if prompt.contains("Write Batch 4") {
            concat!(
                "## 8. Predictions\n\n",
                "### Short-term (0-2 years)\nIf C++26 ratifies on schedule, vendor parity holds. If not, dialect fragmentation increases (high).\n\n",
                "### Medium-term (2-5 years)\nIf memory-safety profiles ship, regulated sectors retain C++ for new modules. If not, new projects adopt substitutes (medium).\n\n",
                "### Long-term (5-10 years)\nIf subgroup chair succession broadens, review velocity holds. If not, backlog stalls major features (medium).\n\n---"
            )
            .to_owned()
        } else if prompt.contains("Write Batch 5") {
            concat!(
                "## 9. Audit Trail\n\n",
                "- **Tests:** 45 run, 6 findings, 1 killed, 0 downgraded\n",
                "- **Rules:** 3 domain-specific generated, 2 survived\n",
                "- **Dark influence:** 2 demand sentences, 2 candidates, 1 survived\n",
                "- **Theories:** 3 applied, 2 confirmed / 1 partial / 0 falsified\n",
                "- **Compounds:** 0 within-cluster, 1 cross-cluster, 1 gap-derived (0 killed total)\n",
                "- **Direction:** 1 degrading, 1 stable, 0 improving\n\n---\n\n",
                "## 10. References\n\n",
                "### Primary sources\n",
                "[The Committee - isocpp.org](https://isocpp.org/std/the-committee)\\\n",
                "[WG21 Papers - open-std.org](https://www.open-std.org/jtc1/sc22/wg21/)\n\n",
                "### Academic references\n",
                "Burja, S. \"Great Founder Theory.\" Manuscript, 2020.\\\n",
                "Klemperer, P. \"Markets with Consumer Switching Costs.\" *Quarterly Journal of Economics* 102(2):375-394, 1987.\n\n---\n\n",
                "*November 2026 - default-model*"
            )
            .to_owned()
        } else {
            messages
                .last()
                .and_then(|m| {
                    let text = m.content();
                    let marker = "Complete Draft:\n";
                    let start = text.find(marker)? + marker.len();
                    let end = text.find("Diagnostic Battery Citations:")?;
                    Some(text[start..end].trim().to_owned())
                })
                .unwrap_or_else(|| "# Briefer: Audited\n".to_owned())
        };

        Box::pin(async move {
            Completion::from_result(CompletionResult::Text(reply), &model_name).map(Box::new)
        })
    }
}

struct Clock;

impl Timer for Clock {
    fn sleep(&self, duration: Duration) -> BoxFuture<()> {
        Box::pin(tokio::time::sleep(duration))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let briefer_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_BRIEFER_PATH.to_owned());
    let output_dir = std::env::args()
        .nth(2)
        .unwrap_or_else(|| DEFAULT_OUTPUT_DIR.to_owned());
    let source = std::fs::read_to_string(&briefer_path)?;

    let mut registry = PackRegistry::new();
    registry.insert(
        PackInfo::new("runvil/web").description("Mock web pack"),
        |_id: &PackId| -> Result<Arc<dyn Pack>, Error> { Ok(Arc::new(MockWebPack)) },
    )?;

    let host = Arc::new(Host {
        packs: Packs::new(registry),
        prompts: Arc::new(move |name: &str| {
            if name == "briefer.md" {
                return Ok(source.clone());
            }
            let code = ErrorCode::NotFound;
            Err(Error::new(
                code,
                format!("no prompt named {name}"),
                code.prose(),
            ))
        }),
        timer: Arc::new(Clock),
    });

    // 1. Parse and compile all Lua blocks in briefer.md
    let prompt = host.parse("briefer.md")?;
    println!(
        "Parsed prompt `{}` ({}) from {} cleanly!",
        prompt.frontmatter().name(),
        prompt.title(),
        briefer_path
    );

    // 2. Build model catalog and run end-to-end through Harness
    let window = NonZeroU32::new(128_000).ok_or("non-zero window")?;
    let default_model = ModelInfo::new(
        ModelId::gateway("default-model")?,
        "Default parent model",
        window,
        ThinkingMode::Never,
    );
    let fast_model = ModelInfo::new(
        ModelId::gateway("fast")?,
        "Fast model",
        window,
        ThinkingMode::Never,
    );
    let catalog = ModelCatalog::new([default_model, fast_model])?;
    let broker = Arc::new(BrieferBroker { catalog });
    let recorder = Arc::new(MemoryRecorder::new());
    let vfs = VfsRef::default();

    let setup = RunSetup {
        vfs: vfs.clone(),
        selected_model: Some("default-model".to_owned()),
        args: "ISO C++ Committee (WG21)".to_owned(),
        ..RunSetup::new(recorder.clone(), broker)
    };

    let report = Harness::new(host, prompt, setup).run().await?;
    match &report.result {
        Ok(final_text) => {
            println!("Run succeeded! Final Brief length: {} bytes", final_text.len());
        }
        Err(err) => {
            eprintln!("Run failed: {}", err.located_message());
            return Err(err.located_message().into());
        }
    }

    let brief_output = report.output?;
    assert!(brief_output.contains("# Briefer: ISO C++ Committee (WG21)"));
    assert!(brief_output.contains("## 1. Executive Summary"));
    assert!(brief_output.contains("## 4. Structural Assessment"));
    assert!(brief_output.contains("## 9. Audit Trail"));
    assert!(brief_output.contains("## 10. References"));

    println!("\n--- Pipeline Log Checkpoints ---");
    for rec in recorder.records(report.run_id) {
        if rec.kind == RecordKind::Event && rec.payload["kind"] == "lua" {
            println!(
                "[{}] {}",
                rec.payload["section"].as_str().unwrap_or(""),
                rec.payload["message"].as_str().unwrap_or("")
            );
        }
    }

    let store_view = vfs.acquire_store(Origin::new("verify"))?;
    let files = store_view.glob("**/*")?;
    println!("\n--- VFS Store Files Written ---");
    for f in &files {
        println!("  - {f}");
    }

    std::fs::create_dir_all(&output_dir)?;
    let out_file = std::path::Path::new(&output_dir).join("briefer-iso-cpp-committee.md");
    std::fs::write(&out_file, &brief_output)?;
    println!("\nSaved output Brief to: {}", out_file.display());
    Ok(())
}
