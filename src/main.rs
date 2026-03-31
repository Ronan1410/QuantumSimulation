use QuantumSimulation::computer::QuantumComputer;
use QuantumSimulation::deutsch;
use QuantumSimulation::gates;
use std::io::{self, Write};

fn main() {
    loop {
        display_menu();
        let choice = get_user_choice();

        match choice {
            1 => run_identity_simulation(),
            2 => run_coin_flip_simulation(),
            3 => run_deutsch_algorithm_constant(),
            4 => run_deutsch_algorithm_balanced(),
            5 => run_pauli_gates_demo(),
            6 => run_superposition_test(),
            7 => run_multi_qubit_hadamard(),
            8 => run_caesar_cipher_demo(),
            9 => run_xor_cipher_demo(),
            10 => run_substitution_cipher_demo(),
            0 => {
                println!("Exiting quantum simulator. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
        println!("\n{}", "=".repeat(60));
    }
}

fn display_menu() {
    println!("        QUANTUM SIMULATION SCENARIOS");
    println!("{}", "=".repeat(60));
    println!("1. Identity Gate on 3-Qubit System");
    println!("2. Coin Flip (Hadamard Transform)");
    println!("3. Deutsch Algorithm (Constant Function)");
    println!("4. Deutsch Algorithm (Balanced Function)");
    println!("5. Pauli Gates Demonstration");
    println!("6. Superposition Test");
    println!("7. Multi-Qubit Hadamard Demo");
    println!("8. Caesar Cipher (Encryption/Decryption)");
    println!("9. XOR Cipher (Encryption/Decryption)");
    println!("10. Substitution Cipher (Encryption/Decryption)");
    println!("0. Exit");
    println!("{}", "=".repeat(60));
}

fn get_user_choice() -> u32 {
    loop {
        print!("Enter your choice (0-10): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<u32>() {
            Ok(num) if num <= 10 || num == 0 => return num,
            _ => println!("Invalid input. Please enter a number between 0 and 10"),
        }
    }
}

fn run_identity_simulation() {
    println!("\n[Scenario 1: Identity Gate on 3-Qubit System]");
    println!("Creating a 3-qubit quantum computer and initializing to state 5");

    let mut computer = QuantumComputer::new(3);
    computer.initialize(5);
    println!("Applied Identity gate (should preserve state).");
    computer.apply(gates::identity(3));
    computer.collapse();

    let result = computer.value();
    println!("Result: {}", result);
    assert_eq!(5, result);
    println!("State preserved correctly");
}

fn run_coin_flip_simulation() {
    println!("\n[Scenario 2: Coin Flip using Hadamard Transform]");
    println!("Initializing 1-qubit system and applying Hadamard gate");
    println!("Running simulation 5 times:\n");

    for i in 1..=5 {
        let mut computer = QuantumComputer::new(1);
        computer.initialize(0);
        computer.apply(gates::hadamard(1));
        computer.collapse();

        let result = computer.value();
        let coin = if result == 1 { "HEADS" } else { "TAILS" };
        println!("  Flip {}: {} (state: {})", i, coin, result);
    }
    println!("✓ Coin flip simulation complete!");
}

fn run_deutsch_algorithm_constant() {
    println!("\n[Scenario 3: Deutsch Algorithm - Constant Function]");
    println!("Testing with function f(x) = 0 (constant)");

    let f = |_x: i32| -> i32 { 0 }; // Constant function

    let mut computer = QuantumComputer::new(2);
    computer.initialize(1);
    computer.apply(gates::hadamard(2));
    computer.apply(deutsch::deutsch_gate(f));
    computer.apply(gates::hadamard(2));
    computer.collapse();

    let result = computer.value();
    let function_type = if result == 1 { "CONSTANT" } else { "BALANCED" };
    println!("Result: {:?}", function_type);
    println!("Deutsch algorithm executed successfully");
}

fn run_deutsch_algorithm_balanced() {
    println!("\n[Scenario 4: Deutsch Algorithm - Balanced Function]");
    println!("Testing with function f(x) = x (balanced)");

    let f = |x: i32| -> i32 { x }; // Balanced function (identity)

    let mut computer = QuantumComputer::new(2);
    computer.initialize(1);
    computer.apply(gates::hadamard(2));
    computer.apply(deutsch::deutsch_gate(f));
    computer.apply(gates::hadamard(2));
    computer.collapse();

    let result = computer.value();
    let function_type = if result == 1 { "CONSTANT" } else { "BALANCED" };
    println!("Result: {:?}", function_type);
    println!("Deutsch algorithm executed successfully");
}

fn run_pauli_gates_demo() {
    println!("\n[Scenario 5: Pauli Gates Demonstration]");
    println!("Demonstrating Pauli X, Y, Z gates on 1-qubit system:\n");

    // Pauli X (NOT gate)
    println!("  Pauli X (NOT gate): flips |0⟩ to |1⟩");
    let mut c_x = QuantumComputer::new(1);
    c_x.initialize(0);
    c_x.apply(gates::pauli_x());
    c_x.collapse();
    println!("    Initial state: 0 → Final state: {}\n", c_x.value());

    // Pauli Z
    println!("  Pauli Z (Phase gate):");
    let mut c_z = QuantumComputer::new(1);
    c_z.initialize(1);
    c_z.apply(gates::pauli_z());
    c_z.collapse();
    println!("    Applied to |1⟩ → Final state: {}\n", c_z.value());

    // Double Pauli X (should return to original)
    println!("  Double Pauli X (should return to original state):");
    let mut c_double = QuantumComputer::new(1);
    c_double.initialize(0);
    c_double.apply(gates::pauli_x());
    c_double.apply(gates::pauli_x());
    c_double.collapse();
    println!("    Initial: 0 → After 2× Pauli X: {}", c_double.value());
    println!("Pauli gates demonstration complete");
}

fn run_superposition_test() {
    println!("\n[Scenario 6: Superposition Test]");
    println!("Creating superposition with Hadamard and measuring multiple times:\n");

    let mut measurements = vec![0, 0];

    println!("Running 10 measurements:");
    for i in 1..=10 {
        let mut computer = QuantumComputer::new(1);
        computer.initialize(0);
        computer.apply(gates::hadamard(1));
        computer.collapse();

        let result = computer.value();
        measurements[result as usize] += 1;
        println!("  Measurement {}: {}", i, result);
    }

    println!("\nStatistics:");
    println!("  State 0: {} times", measurements[0]);
    println!("  State 1: {} times", measurements[1]);
    println!("Superposition test complete");
}

fn run_multi_qubit_hadamard() {
    println!("\n[Scenario 7: Multi-Qubit Hadamard Demo]");
    println!("Applying Hadamard gates to a 1-qubit system:\n");

    let mut computer = QuantumComputer::new(1);
    computer.initialize(0);

    println!("Applying Hadamard gates sequentially:");
    println!("  Initial state: 0");

    computer.apply(gates::hadamard(1));
    println!("  After 1st Hadamard:");
    computer.collapse();
    println!("    State: {}", computer.value());

    let mut computer = QuantumComputer::new(1);
    computer.initialize(0);
    computer.apply(gates::hadamard(1));
    computer.apply(gates::hadamard(1));
    println!("  After 2nd Hadamard: (should return to 0/1 - random due to superposition)");
    computer.collapse();
    println!("    State: {}", computer.value());

    println!("Multi-Qubit Hadamard demo complete");
}

fn run_caesar_cipher_demo() {
    println!("\n[Scenario 8: Caesar Cipher - Encryption/Decryption]");
    println!("A classical cipher that shifts letters by a fixed amount.\n");

    let plaintext = "HELLO WORLD";
    let shift = 3;

    println!("Original text: {}", plaintext);
    println!("Shift value: {}\n", shift);

    let encrypted = caesar_encrypt(plaintext, shift);
    println!("Encrypted: {}", encrypted);

    let decrypted = caesar_decrypt(&encrypted, shift);
    println!("Decrypted: {}\n", decrypted);

    assert_eq!(plaintext, decrypted);
    println!("Caesar cipher demo complete");
}

fn caesar_encrypt(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                let shifted = ((c as u8 - b'A' + shift) % 26) + b'A';
                shifted as char
            } else if c.is_ascii_lowercase() {
                let shifted = ((c as u8 - b'a' + shift) % 26) + b'a';
                shifted as char
            } else {
                c // Keep non-alphabetic characters unchanged
            }
        })
        .collect()
}

fn caesar_decrypt(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                let shifted = ((c as u8 - b'A' + 26 - shift % 26) % 26) + b'A';
                shifted as char
            } else if c.is_ascii_lowercase() {
                let shifted = ((c as u8 - b'a' + 26 - shift % 26) % 26) + b'a';
                shifted as char
            } else {
                c // Keep non-alphabetic characters unchanged
            }
        })
        .collect()
}

fn run_xor_cipher_demo() {
    println!("\n[Scenario 9: XOR Cipher - Encryption/Decryption]");
    println!("XOR cipher using a secret key (symmetric encryption).\n");

    let plaintext = "QUANTUM";
    let key = 42u8;

    println!("Original text: {}", plaintext);
    println!("XOR Key: {} (binary: {:08b})\n", key, key);

    let encrypted = xor_encrypt(plaintext, key);
    println!("Encrypted (bytes): {:?}", encrypted);

    let decrypted = xor_decrypt(&encrypted, key);
    println!("Decrypted text: {}\n", decrypted);

    assert_eq!(plaintext, decrypted);
    println!("XOR cipher demo complete");
}

fn xor_encrypt(text: &str, key: u8) -> Vec<u8> {
    text.as_bytes().iter().map(|&b| b ^ key).collect()
}

fn xor_decrypt(encrypted: &[u8], key: u8) -> String {
    encrypted
        .iter()
        .map(|&b| (b ^ key) as char)
        .collect()
}

fn run_substitution_cipher_demo() {
    println!("\n[Scenario 10: Substitution Cipher - Encryption/Decryption]");
    println!("Simple cipher that replaces each letter with a fixed substitution.\n");

    let plaintext = "CRYPTOGRAPHY";
    let key = "BCDEFGHIJKLMNOPQRSTUVWXYZA"; // ROT1 substitution

    println!("Original text: {}", plaintext);
    println!("Substitution key: {}\n", key);

    let encrypted = substitution_encrypt(plaintext, key);
    println!("Encrypted: {}", encrypted);

    // Create reverse key for decryption
    let mut reverse_key = ['A'; 26];
    for (i, c) in key.chars().enumerate() {
        reverse_key[c as usize - 'A' as usize] = ('A' as u8 + i as u8) as char;
    }
    let reverse_key_str: String = reverse_key.iter().collect();

    let decrypted = substitution_encrypt(&encrypted, &reverse_key_str);
    println!("Decrypted: {}\n", decrypted);

    assert_eq!(plaintext, decrypted);
    println!("Substitution cipher demo complete");
}

fn substitution_encrypt(text: &str, key: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                let index = (c as usize - 'A' as usize) % 26;
                key.chars().nth(index).unwrap_or(c)
            } else if c.is_ascii_lowercase() {
                let index = (c as usize - 'a' as usize) % 26;
                key.chars().nth(index).unwrap_or(c).to_lowercase().next().unwrap()
            } else {
                c
            }
        })
        .collect()
}