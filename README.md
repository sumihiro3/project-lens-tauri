# Tauri + Nuxt Project Template

A streamlined GitHub template for initializing a modern project with [Tauri](https://tauri.app/) (v2.0.6) and [Nuxt](https://nuxt.com/) (v3.14.159). Quickly spin up a secure and high-performance desktop application with the versatility of Nuxt for frontend development.

> **Note**: Tauri 2.0.6 does not work out-of-the-box with Nuxt, and running Tauri directly through npm scripts may cause issues. This template addresses these compatibility challenges and will be continuously updated for improvements.

## Features

- **Tauri**: Secure, lightweight, and optimized for native desktop applications.
- **Nuxt 3**: Flexible and powerful full-stack framework, enhancing frontend capabilities with Vue 3.
- **Cross-platform Support**: Build for Windows, macOS, and Linux from a single codebase.
- **Easy Setup**: Streamlined template for rapid initialization of Tauri and Nuxt projects.

## Prerequisites

- **Node.js**: Recommended version [v20.19 or higher](https://nodejs.org/en/).
- **Rust**: Latest stable version. Install from [rustup.rs](https://rustup.rs/).
- **Tauri CLI**: Install globally via `cargo install tauri-cli`.

## Getting Started


Replace all instances of tauri-nuxt-app with your-target-app-name

### 1. Install Dependencies\*\*

Install Nuxt and other yarn dependencies\*

```bash
yarn install
```

### 3. Run the Application

**Development Mode**

To start the Nuxt development server with Tauri’s hot reload enabled:

```bash
yarn tauri:dev
```

**Build for Production**

To bundle the application and prepare it for release:

```bash
yarn tauri:build
```

## Project Structure

tauri-nuxt-app/
├─ src/ # Nuxt app source files
├─ src-tauri/ # Tauri configuration and Rust backend
├─ public/ # Static assets for the app
└─ README.md # Project documentation

## Configuration

All Tauri configurations are managed within the src-tauri/tauri.conf.json file. Adjust these settings based on your specific needs, including window size, title, and permission configurations.

## Additional Resources

- **Tauri Documentation**: [tauri.app](https://tauri.app/v2/)

- **Nuxt Documentation**: [nuxt.com](https://nuxt.com/docs/getting-started/introduction)

## License

This template is licensed under the MIT License.

With this template, you’re ready to start building powerful desktop applications leveraging the best of Tauri and Nuxt. Happy coding! 🎉
