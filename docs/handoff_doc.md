# Project Handoff: crustimg - Rust Image Processing CLI

## Project Goal
The primary goal of `crustimg` is to provide a robust and efficient command-line interface (CLI) tool for various image processing tasks. This includes displaying image information, applying filters, performing basic manipulations, handling EXIF data, and potentially more advanced effects, all within a Rust ecosystem.

## Key Technologies Utilized
This project leverages the following core Rust technologies and libraries:

*   **Rust (Edition 2024):** The foundational language for performance and safety.
*   **Tokio:** An asynchronous runtime for building fast, reliable, and scalable network applications and other async tasks. This is crucial for potentially non-blocking image operations or future network capabilities.
*   **Image Crate (`image`):** A powerful and versatile library for loading, manipulating, and saving various image formats (JPEG, PNG, GIF, BMP, TIFF, WebP, etc.).
*   **Photon-rs:** A high-performance, cross-platform Rust library for processing images. It provides a wide range of image filtering and manipulation functions.
*   **Clap:** For declarative and user-friendly command-line argument parsing.
*   **Anyhow:** For simplified and robust error handling across the application.
*   **Viuer:** For displaying images directly in the terminal, enhancing the CLI experience.
*   **Ratatui / Crossterm:** For building rich terminal user interfaces (TUIs), suggesting potential for interactive image viewers or editors.
*   **Kamadak-exif:** For reading and writing EXIF metadata from images.
*   **Walkdir:** For efficient directory traversal, useful for batch processing.
*   **Imagesize:** For quickly determining image dimensions.
*   **Log / Humantime:** For structured logging and human-readable time formats.

## Current Efforts and Implemented Features
The current codebase (`src/` directory) indicates significant progress in implementing the following functionalities:

*   **`image_display.rs`:** Logic for displaying images, likely utilizing `viuer` or `ratatui`.
*   **`image_filters.rs`:** Implementation of various image filters.
*   **`image_info.rs`:** Functionality to extract and present image metadata.
*   **`image_ops.rs`:** General image manipulation operations (e.g., resizing, cropping).
*   **`image_ops_exif.rs`:** Operations specifically related to EXIF data (reading, modifying).
*   **`image_sz.rs`:** Handling image size-related tasks.
*   **`photon_effects.rs` / `photon_ops.rs`:** Integration and application of effects provided by the `photon-rs` library.
*   **`main.rs`:** The main entry point, orchestrating CLI commands and calling into the various modules.

## Potential and Future Direction
The project demonstrates strong potential due to:
*   **Performance:** Leveraging Rust's speed and efficiency for image processing, which is often computationally intensive.
*   **Asynchronous Capabilities:** Integration with Tokio opens doors for highly responsive applications, especially for batch processing or future network-enabled features.
*   **Rich Feature Set:** The combination of `image` and `photon-rs` provides a comprehensive suite of image manipulation and filtering capabilities.
*   **CLI/TUI Focus:** A well-designed CLI/TUI can offer a powerful and accessible tool for developers and power users.
*   **Extensibility:** The modular structure (e.g., separate files for display, filters, info, ops) makes it easy to add new features.

## Rationale for a Clean Start
A clean start is desired to:
*   **Refine Architecture:** Re-evaluate and potentially optimize the overall project structure and module interactions.
*   **Improve Code Quality & Consistency:** Ensure a consistent coding style, best practices, and potentially introduce more rigorous testing from the outset.
*   **Address Technical Debt:** Start fresh without carrying over any existing design compromises or quick fixes.
*   **Leverage Latest Best Practices:** Incorporate the most up-to-date Rust idioms and library usage patterns.
*   **Clear Vision:** Re-establish a clear roadmap and implementation strategy for the next phase of development.
