# 🩺 GlucoGuard Systems — Automated Insulin Delivery (AID) 

📘 Project Description

GlucoGuard Systems is an open-source Automated Insulin Delivery (AID) System simulation written in Rust.
The project aims to model real-world insulin pump behavior by:

- Simulating continuous glucose monitoring (CGM) readings.

- Delivering safe, automated basal and bolus insulin doses.

- Generating alerts for high or low glucose levels.

- Providing secure access and role-based control for clinicians, caretakers, and patients.

- This system supports CLI interaction and can later be extended with a web or GUI interface for visualization and management.

⚙️ Features

- Continuous glucose simulation with configurable input.

- Safe insulin dose calculation with built-in limits.

- Role-based authentication (Clinician, Caretaker, Patient).

- Real-time alerts and secure logging of all operations.

- Extendable for web or GUI front-end visualization.

🧩 Project Structure
```
glucoguard/
├── src/
│   ├── main.rs              # Entry point (CLI handling)
│   ├── cgm.rs               # CGM data simulation and parsing
│   ├── insulin.rs           # Basal/Bolus insulin control logic
│   ├── auth.rs              # Authentication and role management
│   ├── alerts.rs            # Alert generation for glucose levels
│   ├── logger.rs            # Logging and data persistence
│   └── utils.rs             # Helper functions
├── data/
│   └── sample_readings.txt  # Example glucose data input
├── Cargo.toml               # Rust project configuration
└── README.md

```
🧰 Setup Instructions
1. Clone the Repository
```
git clone https://github.com/Anwar720/glucoguard.git
cd glucoguard

```
2. Build the Project
```
cargo build

```

3. Run the Simulation
```
cargo run data/sample_readings.txt

```


You can also feed glucose readings via STDIN or socket input.

👥 Contributing

Fork the repository and create your own branch:

```
git checkout -b feature/your-feature-name

```


Commit your changes with clear messages:

```
git commit -m "Add CGM simulation logic"

```


Push your branch and create a pull request:

```
git push origin feature/your-feature-name

```


Wait for team review before merging into main.

🧪 Testing

Run tests with:

```
cargo test

```


Add tests for new modules or edge cases (invalid data, overdose prevention, etc.).

🔒 Security & Safety

All critical actions (doses, alerts, settings) must be logged with timestamps and user roles.

Follow secure coding practices and handle user authentication carefully.

Never push real patient data or credentials to the repository.

🧑‍💻 Team: GlucoGuard Systems

[Anwar Jahid] 

[Kwame Davour] 

[MD Younus] 

[Honore Mandiamy] 
