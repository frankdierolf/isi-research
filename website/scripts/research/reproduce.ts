/**
 * Research Methodology Documentation
 *
 * This script documents the step-by-step process used to create
 * the ISI (Intelligent System Interface) Voice Image research paper.
 *
 * Run with: npm run research
 */

const CYAN = "\x1b[36m";
const YELLOW = "\x1b[33m";
const GREEN = "\x1b[32m";
const BOLD = "\x1b[1m";
const DIM = "\x1b[2m";
const RESET = "\x1b[0m";

function header(text: string): void {
  console.log(`\n${CYAN}${BOLD}═══════════════════════════════════════════════════════════════${RESET}`);
  console.log(`${CYAN}${BOLD}  ${text}${RESET}`);
  console.log(`${CYAN}${BOLD}═══════════════════════════════════════════════════════════════${RESET}\n`);
}

function step(num: number, title: string): void {
  console.log(`${YELLOW}${BOLD}Step ${num}: ${title}${RESET}`);
  console.log(`${DIM}${"─".repeat(60)}${RESET}`);
}

function bullet(text: string): void {
  console.log(`  ${GREEN}•${RESET} ${text}`);
}

function note(text: string): void {
  console.log(`\n  ${DIM}${text}${RESET}`);
}

function blank(): void {
  console.log();
}

// ─────────────────────────────────────────────────────────────────
// Introduction
// ─────────────────────────────────────────────────────────────────

header("ISI Voice Image Research - Methodology Documentation");

console.log(`This document explains how the ISI (Intelligent System Interface)
research paper was created using AI-assisted research methods.

The goal is ${BOLD}reproducibility${RESET} - allowing others to understand and
replicate the AI-assisted research process used to produce this paper.`);

blank();

// ─────────────────────────────────────────────────────────────────
// Step 1: Build the Prototype
// ─────────────────────────────────────────────────────────────────

step(1, "Build the Prototype (48-hour hackathon)");

bullet("Built a working voice-to-image prototype");
bullet("Created this website to showcase the project");
bullet("Timeline: January 22-23, 2026");
bullet("Focus: Rapid iteration, functional demo");

note("The prototype demonstrates feasibility of voice-controlled image generation.");

blank();

// ─────────────────────────────────────────────────────────────────
// Step 2: Find Distribution Venue
// ─────────────────────────────────────────────────────────────────

step(2, "Find Distribution Venue");

bullet("Used Claude to identify appropriate academic venues");
bullet("Selected: CHI LBW (Late Breaking Work at CHI)");
bullet("CHI = ACM Conference on Human Factors in Computing Systems");
bullet("CHI LBW is ideal for rapid prototypes and feasibility studies");

note("LBW papers are 4-6 pages and focus on early-stage research.");

blank();

// ─────────────────────────────────────────────────────────────────
// Step 3: Generate Research Questions
// ─────────────────────────────────────────────────────────────────

step(3, "Generate Research Questions");

bullet("Spun up Claude Code in the project root directory");
bullet("Asked for 5 distinct research angles");
bullet("Generated researcher prompts stored in artifacts/ folder:");

console.log(`
    ${DIM}Researcher 1:${RESET} Literature Review & Academic Foundations
    ${DIM}Researcher 2:${RESET} Technical Foundations & AI/ML Landscape
    ${DIM}Researcher 3:${RESET} Feasibility Study Methodology
    ${DIM}Researcher 4:${RESET} User Experience & Voice Interface Design
    ${DIM}Researcher 5:${RESET} Industry Context & Market Landscape`);

note("Each prompt defines a specific research perspective and questions.");

blank();

// ─────────────────────────────────────────────────────────────────
// Step 4: Deep Research
// ─────────────────────────────────────────────────────────────────

step(4, "Deep Research with Claude.ai");

bullet("Used Claude.ai with the Deep Research feature");
bullet("Fed each researcher prompt as a separate session");
bullet("Received comprehensive markdown responses");
bullet("Stored results as researcher-*-response.md files");

note("Deep Research provides citation-backed, comprehensive analysis.");

blank();

// ─────────────────────────────────────────────────────────────────
// Step 5: Write the Paper
// ─────────────────────────────────────────────────────────────────

step(5, "Write the Paper with Claude Code");

bullet("Used Claude Code with full project context:");
console.log(`
    ${DIM}•${RESET} Prototype codebase
    ${DIM}•${RESET} Website implementation
    ${DIM}•${RESET} All research artifacts (prompts + responses)`);

bullet("Requested output in Typst format");
bullet("Core philosophy: truthful, honest, accurate");
bullet("Requirements: simple, concise, no false claims");

note("Typst is a modern typesetting system, similar to LaTeX but simpler.");

blank();

// ─────────────────────────────────────────────────────────────────
// Step 6: Review
// ─────────────────────────────────────────────────────────────────

step(6, "Personal Review");

bullet("Read through the generated paper");
bullet("Verified alignment with research goals");
bullet("Checked factual accuracy of claims");
bullet("No formal peer review (this is a feasibility study)");

note("Human oversight remains essential for AI-assisted research.");

blank();

// ─────────────────────────────────────────────────────────────────
// Step 7: Generate PDF
// ─────────────────────────────────────────────────────────────────

step(7, "Generate PDF");

bullet("Compiled Typst document to PDF");
bullet("Command: typst compile isi-paper.typ");
bullet("Placed PDF in public/ folder for download");

note("The paper is available at: /isi-paper.pdf");

blank();

// ─────────────────────────────────────────────────────────────────
// Closing Notes
// ─────────────────────────────────────────────────────────────────

header("Important Notes on Reproducibility");

console.log(`${YELLOW}AI systems are probabilistic.${RESET}

This means:
  • Running the same prompts will produce different outputs
  • Research quality depends on prompt engineering
  • Results require human verification

This documentation describes the ${BOLD}procedure${RESET}, not guaranteed outcomes.
The artifacts in this repository show what ${BOLD}we${RESET} received - your results
will vary.`);

blank();

console.log(`${DIM}─────────────────────────────────────────────────────────────────${RESET}`);
console.log(`${DIM}Artifacts location: website/scripts/research/artifacts/${RESET}`);
console.log(`${DIM}Paper source:       website/scripts/research/isi-paper.typ${RESET}`);
console.log(`${DIM}Paper PDF:          website/public/isi-paper.pdf${RESET}`);
console.log(`${DIM}─────────────────────────────────────────────────────────────────${RESET}`);

blank();
