# SimuWork - A User Simulation Utility

SimuWork is a sophisticated user simulation utility designed to automate and mimic human-like user interactions on desktop operating systems. Built with a focus on performance and a low resource footprint, this application can simulate mouse movements, clicks, keyboard typing, and scrolling to create a convincing illusion of user activity.

This project was developed as a proof-of-concept, building a full-featured application from the ground up using a modern, cross-platform technology stack.

## ✨ Core Features

* **Human-like Mouse Movement**: Simulates mouse movement along smooth, randomized Bézier curves using the `kurbo` library, avoiding robotic, straight-line paths.
* **Multi-Modal Simulation**: Can perform a variety of actions, including mouse movement, left-clicks, vertical scrolling, and keyboard typing.
* **Randomized Action Scheduler**: A background scheduler runs on a continuous loop, randomly selecting different actions to perform at randomized intervals to avoid predictable, machine-like patterns.
* **Interactive UI**: A polished user interface built with React allows for real-time configuration of the simulation.
* **User-Configurable Target**: Features an interactive "mini-screen" on the control panel that allows the user to set a specific target coordinate on their screen for clicking and typing actions.
* **Cross-Platform Foundation**: Built on the Tauri framework, enabling compilation for Windows, macOS, and Linux from a single codebase.

## 🛠️ Technology Stack

* **Framework**: **Tauri** (v2)
* **Backend**: **Rust**
* **Frontend**: **React** with **TypeScript** and Vite
* **Core Rust Dependencies**:
    * `enigo`: For cross-platform input simulation (mouse, keyboard).
    * `kurbo`: For 2D curve geometry and Bézier path generation.
    * `rand`: For randomization of actions and timing.

## 🚀 Getting Started

To get a local copy up and running, follow these steps.

### Prerequisites

You will need to have a complete development environment for Tauri set up. This includes:
* Rust and Cargo
* Node.js and npm
* OS-specific dependencies (e.g., Build Tools for Visual Studio on Windows, Xcode Command Line Tools on macOS).

A full guide can be found at the [official Tauri documentation](https://tauri.app/v1/guides/getting-started/prerequisites).

### Installation and Running

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/pyvmag/SimuWork-Automate-WFH-.git](https://github.com/pyvmag/SimuWork-Automate-WFH-.git)
    ```
2.  **Navigate into the project directory:**
    ```bash
    cd SimuWork-Automate-WFH-
    ```
3.  **Install frontend dependencies:**
    ```bash
    npm install
    ```
4.  **Run the application in development mode:**
    ```bash
    npm run tauri dev
    ```
    > **Note for Windows users:** For the mouse and keyboard simulation to be able to control other applications, you must run the command from a terminal with **Administrator** privileges.

## 📦 Building for Production

To build a standalone installer or portable executable for your platform, run the following command:

```bash
npm run tauri build
