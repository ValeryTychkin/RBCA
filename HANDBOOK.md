# HANDBOOK

## Crates Organization

### Library Crates
Crates located in the `libs` directory are strictly libraries. They contain only a `lib.rs` file and cannot function as standalone applications. 
Their purpose is to provide reusable functionality to other parts of the system. 

#### Example Usage
To add a library crate:
1. Create a folder in `./crates/libs/` with the desired library name.
2. The package name must end with `-lib`.

**Example:**  
To add a library called `adapter`:
- Create the folder: `./crates/libs/adapter`
- Update `workspace.dependencies` in `./Cargo.toml`:
  ```toml
  adapter-lib = { path = "crates/libs/adapter" }
  ```

### Adding Procedural Macros
If a library requires procedural macros, the following structure should be used:  
1. Create the main library folder: `./crates/libs/{library_name}`.
2. Inside this folder, create a `proc` directory for procedural macros.
3. Use a specific naming convention for the procedural macros crate (`./crates/libs/{library_name}/proc/Cargo.toml`): `{library_name}-lib-proc`.

**Example:**
To add a library named `util` with procedural macros:
- Create the folder: `./crates/libs/util`.
- Add the procedural macros in: `./crates/libs/util/proc`.
  
The resulting structure:
```bash
./crates/libs/util/Cargo.toml
./crates/libs/util/src/
./crates/libs/util/proc/Cargo.toml
./crates/libs/util/proc/src/
```
  
**Usage Rules**
- Procedural macros should only be used within their corresponding library.
- Update `./crates/libs/util/Cargo.toml` to include `util-lib-proc`:
  ```toml
  util-lib-proc = { path = "./proc" }
  ```
- Update `./crates/libs/util/lib.rs` or other modules in `util-lib`: `pub use util-lib-proc::*`
- Add util-lib to workspace.dependencies:
  ```toml
  util-lib = { path = "crates/libs/util" }
  ```
- Using in other crates: `use util-lib::SomeProcMacro;`

### Service Crates 
Crates in the `services` directory are executable applications. 
They can be run directly using the `cargo run` command. 
These crates provide service-level functionality such as APIs, migrations, or other business logic.
  
Unlike libraries, service crate names do not require any special suffixes.

#### Example Commands
- Run the API server:
  ```bash
  cargo run -p api-server
  ```
- Run the JSON-RPC server:
  ```bash
  cargo run -p jrpc-server
  ```

---

## Dependency Management

All dependencies must be declared in the root `Cargo.toml` file under `workspace.dependencies`.
*Exceptions:* Procedural macros, as described above.
  
This ensures:
- Consistency in dependency versions and features across crates.
- Simplified dependency management for developers.
  
### In Crates
When adding dependencies in `libs` or `services`, simply use:
```toml
{dependency_name} = { workspace = true }
```

---

## Naming Convention for Files and Directories

To ensure consistency and readability across the project, all Rust files and directories must be named in the **singular form**. This convention helps avoid ambiguity and aligns with common Rust practices.

### Rules:
1. **Files:**  
   Use singular, lowercase names for Rust files, separating words with underscores (`_`).  
   **Example:**  
   - `schema.rs` (✅)  
   - `schemas.rs` (❌)

2. **Directories:**  
   Similarly, use singular, lowercase names for directories.  
   **Example:**  
   - `schema/` (✅)  
   - `schemas/` (❌)

3. **Module Names:**  
   When declaring modules, follow the file name without adding plural forms.  
   **Example:**  
   ```rust
   mod user; // File: user.rs
   mod product; // File: product.rs
   ```

4. **Avoid `mod.rs`**
   Do not use `mod.rs` files for module definitions. While historically common, this approach is now considered outdated and less flexible. 
   Instead, prefer naming files directly after their module.


#### Why Avoid `mod.rs`?
1. **Modern Rust Convention:**  
   Using `mod.rs` is an older convention that has been replaced by more intuitive module naming practices.
   
2. **Readability:**  
   Having files named after their module makes it easier to navigate and understand the project's structure.

3. **Flexibility:**  
   Direct naming avoids potential ambiguity and supports better IDE tooling and code navigation.


---

## Importance of Thorough Documentation for Libraries

Libraries in this project serve as reusable components, distinct from business logic, and are designed to be used across various contexts and scenarios. 
Because of their broad applicability and abstraction level, it is critical to ensure that libraries are thoroughly documented.


### Why is Documentation Crucial for Libraries?

1. **Libraries Are Context-Agnostic:**  
   Unlike business logic, which is tightly coupled to specific use cases, libraries are designed to be general-purpose. 
   This means their functionality and implementation might not immediately align with the specific needs of a given service or application. 
   Clear documentation bridges this gap, making libraries easier to understand and integrate.

2. **Avoiding Redundancy:**  
   Poorly documented libraries increase the likelihood of misunderstandings. 
   Developers may find it too difficult or time-consuming to decipher how a library works and might opt to rewrite functionality that already exists. 
   Thorough documentation prevents such duplication by providing developers with the knowledge they need to leverage existing libraries.

3. **Improved Readability and Maintainability:**  
   Comprehensive comments and explanations enhance the readability of library code. This ensures that:
   - Future contributors can quickly grasp the purpose and usage of methods, structures, and traits.
   - Maintenance and debugging become significantly easier.


### Guidelines for Documenting Libraries

1. **Document All Public Items:**
   - **Structures and Enums:**  
     Provide a high-level explanation of the structure, its purpose, and use cases. For each field, describe its role and any constraints.
     ```rust
     /// Represents a user in the system.
     pub struct User {
         pub id: i32,
         pub name: String,
     }
     ```

   - **Methods:**  
     Explain the method's purpose, its input parameters, and the expected return value. Mention any side effects or performance considerations.
     ```rust
     /// Calculates the user's age based on their birthdate.
     ///
     /// # Arguments
     /// - `birthdate`: The user's date of birth.
     ///
     /// # Returns
     /// - The user's age as an `i32`.
     ///
     /// # Examples
     /// ```
     /// let age = calculate_age("2000-01-01");
     /// assert_eq!(age, 25);
     /// ```
     pub fn calculate_age(birthdate: &str) -> i32 {
         // Implementation
     }
     ```

2. **Provide Use Cases and Examples:**
   - Include **real-world examples** or usage patterns in the documentation. These examples help developers understand how to integrate the library in practical scenarios.

3. **Clarify Intentions and Edge Cases:**
   - Explain why certain design choices were made or how specific edge cases are handled. This prevents incorrect usage and aids in debugging.


### Benefits of Good Documentation

1. **Increased Adoption:**  
   Well-documented libraries are more likely to be used correctly and frequently.

2. **Time Savings:**  
   Developers spend less time trying to understand how a library works and more time implementing business logic.

3. **Reduced Risk of Duplication:**  
   Developers are less inclined to rewrite existing functionality when documentation makes the library's capabilities clear.

4. **Higher Code Quality:**  
   Clear documentation often leads to cleaner and more structured code as developers consider how to explain their designs effectively.

---

## Summary of Rules

1. **Library Crates:**
  - Located in `./crates/libs/`.
  - Package names end with `-lib`.
  - Cannot be executed as standalone applications.
2. **Service Crates:**
  - Located in `./crates/services/`.
  - No special naming convention required.
  - Can be executed using `cargo run`.
3. **Procedural Macros:**
  - Organized within the respective library's folder under a `proc` directory.
  - Usage is limited to their corresponding library.
4. **Dependency Management:**
  - Declare all dependencies in the root `Cargo.toml`.
  - Use `{ workspace = true }` for internal dependencies.
5. **Write Documentation for all crates in libs**
  - For hard understude structs/methods write examples
  - It's criticle for *Reduced Risk of Duplication*, *Time Savings* and *High Code Quality*

