# XET Composer

XET Composer is a tool for composing and deploying Solidity smart contracts using a visual interface. It features a Rust backend for contract generation and deployment, and a Next.js frontend for user interaction.

## Project Structure

- `frontend/`: Next.js frontend application.
- `backend/`: Rust backend application (Axum server).
- `contracts/`: Solidity contract templates (`.sol.tera`) and vendorized libraries.
  - `contracts/lib/openzeppelin-repo/`: Expected location for the vendorized OpenZeppelin contracts repository.
- `deployments/`: Intended for storing deployment artifacts (not yet used).

## Prerequisites

- **Rust:** Version 1.78.0 or newer. Install via [rustup](https://rustup.rs/).
- **Solidity Compiler (`solc`):** Required by the backend. Ensure it's installed and accessible. The backend can be configured to use a specific `solc` executable via the `SOLC_PATH` environment variable (defaults to `solc` if in system PATH).
- **Node.js and npm:** For running the Next.js frontend. Install via [nvm](https://github.com/nvm-sh/nvm) or official installers.
- **OpenZeppelin Contracts (Vendorized):**
  - Clone or copy the [OpenZeppelin Contracts repository](https://github.com/OpenZeppelin/openzeppelin-contracts) into `xet-composer/contracts/lib/openzeppelin-repo/`.
  - The backend is configured to find imports like `@openzeppelin/contracts/...` at `lib/openzeppelin-repo/contracts/...` relative to the `xet-composer/contracts/` directory.

## Backend Setup & Run (Rust)

1.  **Navigate to the backend directory:**
    ```bash
    cd xet-composer/backend/xet_composer_backend
    ```

2.  **Set `SOLC_PATH` (Optional):**
    If `solc` is not in your system PATH, or you want to use a specific version:
    ```bash
    export SOLC_PATH=/path/to/your/solc
    ```

3.  **Run the backend server:**
    ```bash
    cargo run
    ```
    The server will start on `http://localhost:8000`.

## Frontend Setup & Run (Next.js)

1.  **Navigate to the frontend directory:**
    ```bash
    cd xet-composer/frontend
    ```

2.  **Install dependencies:**
    ```bash
    npm install
    ```

3.  **Run the frontend development server:**
    ```bash
    npm run dev
    ```
    The frontend will be accessible at `http://localhost:3000`.
    It expects the backend to be running on `http://localhost:8000`.

## Development Workflow

1.  Ensure the OpenZeppelin contracts are vendorized as described in Prerequisites.
2.  Start the backend server.
3.  Start the frontend server.
4.  Open `http://localhost:3000` in your browser.
5.  Use the form to configure and "deploy" (currently compiles and simulates deployment) a contract.
    - The `TokenVesting.sol.tera` template is available.
    - Input parameters, including a valid ERC20 token address for the chain you intend to deploy to eventually, beneficiary address, start time, durations, and an initial owner.
