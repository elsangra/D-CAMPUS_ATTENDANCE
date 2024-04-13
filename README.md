Here's the README file for your Decentralized Campus Attendance project:

---

# Decentralized Campus Attendance - README

## Introduction
This project, developed on the Internet Computer (ICP), aims to capture student attendance data for lectures on campus, including the duration of their presence.

## System Requirements (Linux)

### Prerequisites
Ensure the following prerequisites are met on your Linux system:

* **Internet Connection & Terminal:** Ensure internet access and have a terminal window open.
* **Node Package Manager (npm):** Verify npm installation.
* **Rust & Cargo:**
    1. Follow the official Rust installation instructions: [Rust Installation Guide](https://www.rust-lang.org/tools/install).
    2. Install the `wasm32-unknown-unknown` target:
        ```bash
        rustup target add wasm32-unknown-unknown
        ```
* **IC Development Kit (SDK):**
    1. Refer to the official installation guide: [IC SDK Installation Guide](https://internetcomputer.org/docs/current/developer-docs/getting-started/install/)
    2. Provides tools, sample code, and documentation for Internet Computer development.
* **Code Editor:** Choose a code editor (e.g., VS Code) for Rust development.
* **Git:** Ensure Git is installed.

### Updating and Installing DFX
1. **Update Packages:** Ensure all mentioned packages and tools are up-to-date.
2. **DFX Installation:**
    * Run the following command to install DFX:
        ```bash
        DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
        ```
    * Add DFX to your system path:
        ```bash
        echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
        ```

**Project Setup**

1. **Clone the Project:** Clone the Decentralized Campus Attendance repository:
    ```bash
    git clone <project_url>
    ```
2. **Start Local Development:**
    * Navigate to the project directory:
        ```bash
        cd Decentralized_Campus_Attendance
        ```
    * Start the local development environment:
        ```bash
        dfx start --clean --background
        ```

**Deployment**
* Deploy the project to the Internet Computer(Locally):
    ```bash
    dfx deploy
    ```

**Project Functionality**

* **Attendance Tracking:**
    * Records student attendance for lectures.
    * Tracks the duration of student presence in lectures.
* **Lecturer Management:**
    * Enables lecturers to create lectures.
    * Automatically records lecture start time.
    * Allows lecturers to update student attendance numbers.
    * Automatically records lecture end time upon attendance update.
* **Student Monitoring:**
    * Evaluates student time spent on campus based on attendance data.
    * Provides a reference for checking student attendance and monitoring lecturer activity.

**Note:** Further details such as application usage instructions, contribution guidelines, and license information can be added as needed.

---

Feel free to enhance this README with additional information specific to your project's requirements or audience.
