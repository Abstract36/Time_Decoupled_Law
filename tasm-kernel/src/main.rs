mod model;
mod engine;

use model::{Intent, IntentId};
use engine::{AbsenceEngine, MemoryRegistry, IntentRegistry}; // Added IntentRegistry trait usage

fn main() {
    println!("=== TASM Kernel Simulation (Phoenix) - Economic Edition ===");
    println!("Initializing Time-Absence State Machine with Slashing Logic...");

    // 1. Setup
    let mut registry = MemoryRegistry::new();
    
    // Seed Accounts
    registry.set_balance("alice", 2000);
    registry.set_balance("charlie", 800);
    
    // Create Engine (giving ownership of registry)
    let mut kernel = AbsenceEngine::new(registry, 0);

    println!("\n[State] Initial Balances:");
    println!("    Alice:   {}", kernel.get_registry_mut().get_balance("alice"));
    println!("    Charlie: {}", kernel.get_registry_mut().get_balance("charlie"));

    // 2. Create User Intents (Commitments)
    println!("\n[Users] Creating intents with collateral (SHA-256 Proofs)...");
    
    let intent_a = Intent::new(
        "alice",
        "Alice pays Bob 100",
        10,
        1000
    );

    let intent_b = Intent::new(
        "charlie",
        "Charlie delivers Data",
        20,
        500
    );

    kernel.get_registry_mut().add_intent(intent_a.clone()).unwrap();
    kernel.get_registry_mut().add_intent(intent_b.clone()).unwrap();

    println!("-> Registered Intent A (Alice)");
    println!("   ID: {:?}", intent_a.id);
    println!("-> Registered Intent B (Charlie)");
    println!("   ID: {:?}", intent_b.id);

    // 3. Simulate Time Passing (Tick 1 -> 5)
    println!("\n[Time] Advancing to Slot 5...");
    let absences = kernel.advance_time(5);
    if absences.is_empty() {
        println!("-> No violations. Collateral is safe.");
    }

    // 4. Simulate Deadline Violation (Tick 5 -> 12)
    println!("\n[Time] Advancing to Slot 12...");
    let absences = kernel.advance_time(12);
    
    for abs in &absences {
        println!("!!! ABSENCE CRYSTALLIZED: {:?} !!!", abs.intent_id);
    }
    
    // Check Economics
    println!("\n[Economics] Checking Alice's Balance (Expected Slash)...");
    let alice_bal = kernel.get_registry_mut().get_balance("alice");
    println!("    Alice New Balance: {} (Started with 2000)", alice_bal);
    
    if alice_bal == 1000 {
        println!("    ✅ SLASH CONFIRMED: 1000 coins confiscated.");
    } else {
        println!("    ❌ SLASH FAILED.");
    }

    // 5. Check State
    println!("\n[Time] Advancing to Slot 25...");
    let final_absences = kernel.advance_time(25);
    for abs in &final_absences {
         println!("!!! ABSENCE CRYSTALLIZED: {:?} !!!", abs.intent_id);
    }

    println!("\n[Economics] Checking Charlie's Balance (Expected Slash)...");
    let charlie_bal = kernel.get_registry_mut().get_balance("charlie");
    println!("    Charlie New Balance: {} (Started with 800)", charlie_bal);
    
    println!("\n=== Simulation Complete ===");
}
