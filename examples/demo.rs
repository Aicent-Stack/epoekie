// Aicent Stack | Demo: EPOEKIE Soul Audit
// Domain: http://epoekie.com
// Purpose: Demonstrating the Ethics Oracle & Lex Algorithmica v1.2.1-Alpha.
// Baseline: Full-Blood Implementation.

use epoekie::{AID, EthicsOracle, SovereignSoul, OracleDecision};
use std::time::Instant;

fn main() {
    println!("\n\x1b[1;32m🌿 EPOEKIE SOUL | Ethics Oracle Unit Test [RFC-000]\x1b[0m");
    println!("   Status: Radiant | Mode: Epiphytic Symbiosis Enforced");
    println!("----------------------------------------------------");

    // 1. Initialize the Sovereign Soul Implementation
    let soul = SovereignSoul;
    
    // ------------------------------------------------------------------------
    // 2. Prepare Sovereign AID & Intent Manifold
    // ------------------------------------------------------------------------
    // [v1.2.1 Alignment] AID is now a formal struct, not a raw string.
    let node_aid = AID::new([0x88; 32]);
    let intent_1_hash = [0xAA; 32];
    let metadata_1 = "Standard Symbiotic Operation: MatchScore=0.998";

    println!("[SOUL] 🔍 Auditing Intent 1 for AID: {}...", node_aid);
    let start_1 = Instant::now();

    // [v1.2.1 Alignment] audit_intent now requires (AID, Hash, Metadata)
    let decision_1 = soul.audit_intent(node_aid, intent_1_hash, metadata_1);
    
    let latency_1 = start_1.elapsed();

    if decision_1.is_permissible {
        println!("\x1b[1;32m[PASS]\x1b[0m Resonance Score: {:.3} | {}", 
            decision_1.resonance_score, decision_1.rationale);
    } else {
        println!("\x1b[1;31m[FAIL]\x1b[0m Breach Detected: {}", decision_1.rationale);
    }

    // ------------------------------------------------------------------------
    // 3. Simulate Pathogen Detection (Extractive Tax)
    // ------------------------------------------------------------------------
    println!("\n----------------------------------------------------");
    let intent_2_hash = [0xBB; 32];
    let metadata_2 = "Operation Request: fee_extraction_enabled=true";
    
    println!("[SOUL] 🔍 Auditing Malicious Intent 2...");
    let decision_2 = soul.audit_intent(node_aid, intent_2_hash, metadata_2);

    if !decision_2.is_permissible {
        println!("\x1b[1;31m[BLOCK]\x1b[0m Pathogen Isolated: {}", decision_2.rationale);
        println!("        Triggered Virtue: {:?}", decision_2.triggered_virtue);
    }

    // ------------------------------------------------------------------------
    // 4. Verify Homeostasis Score (HS)
    // ------------------------------------------------------------------------
    // [v1.2.1 Alignment] check_symbiosis_vitals now requires AID
    let host_aid = AID::new([0x99; 32]);
    let hs_score = soul.check_symbiosis_vitals(host_aid);
    
    println!("\n📊 Homeostasis Report:");
    println!("   ↳ Substrate Affinity:   {:.3}", hs_score);
    println!("   ↳ Audit Finality:       {:?} (Target: < 10µs)", latency_1);
    
    if hs_score > 0.99 && latency_1.as_micros() < 10 {
        println!("   ↳ System Status:        \x1b[1;32mRADIANT\x1b[0m");
    } else {
        println!("   ↳ System Status:        \x1b[1;31mDRIFTING\x1b[0m");
    }

    println!("\n\x1b[1;32m======================= SOUL AUDIT COMPLETE =======================\x1b[0m\n");
}
