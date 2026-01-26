use tasm_core::model::{Intent, IntentId};
use tasm_core::engine::{AbsenceEngine, MemoryRegistry, IntentRegistry};

fn main() {
    println!("=== VERIFICATION: Black Swan Scenario (Actor Independence) ===");
    
    // 1. Setup Kernel
    let mut registry = MemoryRegistry::new();
    
    // Seed Accounts (CRITICAL for Slashing Logic)
    registry.set_balance("user1", 2000);
    registry.set_balance("user2", 100000);
    
    let mut kernel = AbsenceEngine::new(registry, 100); // Start at Slot 100

    // 2. Create Long-Term Intents
    println!("\n[Users] Creating intents before 'The Great Silence'...");
    
    // Intent 1 targets Slot 150
    let intent_1 = Intent::new("user1", "Repay Loan", 150, 1000);
    
    // Intent 2 targets Slot 5000 (Very far)
    let intent_2 = Intent::new("user2", "Inheritance Lock", 5000, 50000);
    
    kernel.get_registry_mut().add_intent(intent_1.clone()).unwrap();
    kernel.get_registry_mut().add_intent(intent_2.clone()).unwrap();
    
    println!("-> Registered Intent 1 (Deadline 150)");
    println!("-> Registered Intent 2 (Deadline 5000)");
    
    // 3. Simulate "Black Swan" (Network Halt)
    println!("\n[Event] ⚠️  NETWORK HALT at Slot 100. No blocks produced for 10,000 slots.");
    
    // We intentionally SKIP calling advance_time for slots 101..10099
    
    // 4. Restart Logic (Simulating Slot 10100)
    let restart_slot = 10100;
    println!("\n[Time] Network restarts at Slot {}...", restart_slot);
    
    // The Engine must process ALL gaps in one tick
    let absences = kernel.advance_time(restart_slot);
    
    println!("\n[Analysis] Processing resulting absences...");
    let mut found_1 = false;
    let mut found_2 = false;
    
    for abs in &absences {
        if abs.intent_id == intent_1.id { found_1 = true; }
        if abs.intent_id == intent_2.id { found_2 = true; }
        println!("CRYSTALLIZED: {:?}", abs.intent_id);
    }
    
    // 5. Verification
    if found_1 && found_2 {
        println!("\n✅ VERIFICATION PASSED: All missed deadlines crystallized atomically.");
    } else {
        println!("\n❌ VERIFICATION FAILED: Missing absences.");
        std::process::exit(1);
    }
    
    // Check State Consistency
    let active_count = kernel.get_registry_mut().get_active().len();
    if active_count == 0 {
         println!("✅ STATE CONSISTENT: Registry is empty (0 active items).");
    } else {
        println!("❌ STATE LEAK: Registry still has {} items.", active_count);
        std::process::exit(1);
    }
}
