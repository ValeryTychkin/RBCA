# Web Clean Architecture Example in Rust

This project is an example of implementing Web Clean Architecture in Rust, with a focus on a web backend application. 
The structure is designed to ensure separation of concerns, making the code modular, scalable, and maintainable. 
It provides a solid foundation for building complex, production-ready applications with a clear distinction between 
the core domain, services, and infrastructure layers.

The key principles of this architecture are:
- **DRY (Don't Repeat Yourself)**: The architecture strives to minimize code duplication by organizing the code 
in a way that maximizes reuse, making the system easier to maintain and extend.
- **Minimal Code**: The design avoids unnecessary complexity, ensuring that no spaghetti code is introduced. 
Each component has a clear responsibility, and the codebase remains clean and easy to navigate.
- **Modularity**: The architecture is built with modularity in mind, ensuring that components are reusable 
and can be easily integrated into different parts of the system or even in other projects.
- **Layered Architecture**: The architecture follows a clear and explicit layering model (often referred to as the Onion Architecture), 
where components are organized into distinct layers. Each layer has a well-defined responsibility, and communication between layers occurs through interfaces, promoting separation of concerns and making the system easier to test and maintain.

## Project Structure

### Crates

- **libs**: Contains various libraries used in the services and other libs.
  - **adapter**: Adapter layer for connecting different systems (database, amqp, ...).
  - **entity**: Core domain entities and models.
  - **orm-addons**: Additional ORM-related functionality.
  - **repository-amqp**: Repository implementation for AMQP.
  - **repository-db**: Repository for database interaction.
  - **repository-redis**: Redis-based repository.
  - **util**: Utility functions and helper methods.

- **services**: Contains the service layers and executables.
  - **api-server**: The API server implementation.
  - **migration**: Database migration tool and scripts.

### Configuration Files

- **.gitignore**: Git ignore settings for the project.
- **.keydb.conf**: Configuration file for the KeyDB database.
- **Cargo.lock**: Rust Cargo lock file, specifying the exact versions of dependencies.
- **Cargo.toml**: Main Cargo configuration file for dependencies and settings.
- **Rocket.toml**: Configuration file for the Rocket framework.
- **docker-compose.yml**: Docker Compose configuration for container orchestration.


## Run Project
For running db, amqp and cache:
```bash
docker-compose up -d --build
```

For running web server:
```bash
cargo rest_api
```
