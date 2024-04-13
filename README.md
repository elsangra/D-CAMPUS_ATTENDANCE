
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

# Student Attendance Canister

This is a canister for managing student attendance in lectures. It provides functions to create lectures, finalize attendance, delete lectures, and retrieve lecture details.

## Functions

### `get_lecture`

Retrieves details of a lecture by its ID.

#### Input

- `id`: The ID of the lecture to retrieve.

#### Output

- `Lecture`: Details of the lecture.

### `create_lecture`

Creates a new lecture with the provided details.

#### Input

- `lecture`: Details of the lecture to create.

#### Output

- `Option<Lecture>`: The created lecture if successful.

### `finalize_lecture`

Finalizes a lecture by updating its end time and attendance details.

#### Input

- `id`: The ID of the lecture to finalize.
- `payload`: Updated attendance details.

#### Output

- `Result<Lecture, Error>`: The finalized lecture if successful.

### `delete_lecture`

Deletes a lecture by its ID.

#### Input

- `id`: The ID of the lecture to delete.

#### Output

- `Result<Lecture, Error>`: The deleted lecture if successful.

## Accessing Functions on Candid UI

You can access these functions on the Candid UI by deploying this canister and interacting with it using its generated interface description file (`student_attendance_backend.did`). Here's how you can use the functions:

1. **Deploy the Canister**: Deploy the canister to the Internet Computer.
2. **Generate the Interface Description File**: Use the `candid-extractor` tool to generate the interface description file (`student_attendance_backend.did`) from the compiled WASM file.
3. **Import the Interface on Candid UI**: Go to the Candid UI (https://ic.rocks/candid) and import the interface description file.
4. **Interact with Functions**: Once imported, you can see the available functions and their inputs. You can call these functions with appropriate inputs to interact with the canister.

For example, you can call the `create_lecture` function with the required lecture details to create a new lecture. Similarly, you can call other functions to retrieve, finalize, or delete lectures.



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


