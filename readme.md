# URL Shortener

A simple and efficient URL shortener built with Rust using Actix Web and SQLx
for PostgreSQL. This project allows users to shorten long URLs and retrieve them
using the generated short links.

## Table of Contents

- [Features](#features)
- [Technologies Used](#technologies-used)
- [Getting Started](#getting-started)
- [API Endpoints](#api-endpoints)
- [Database Migration](#database-migration)
- [Contributing](#contributing)
- [License](#license)

## Features

- Shorten long URLs with a unique 6-character identifier.
- Retrieve the original long URL using the short URL.
- List all shortened URLs with their corresponding long URLs.
- Built with asynchronous programming for high performance.

## Technologies Used

- **Rust**: A systems programming language focused on safety and performance.
- **Actix Web**: A powerful, pragmatic, and extremely fast web framework for
  Rust.
- **SQLx**: An async SQL toolkit for Rust, supporting PostgreSQL.
- **Serde**: A framework for serializing and deserializing Rust data structures.
- **Nanoid**: A tiny, secure, URL-friendly, unique string ID generator.

## Getting Started

To get started with this project, follow these steps:

### Prerequisites

- Rust (1.56.0 or later)
- PostgreSQL (installed and running)
- Cargo (Rust package manager)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/url-shortener.git
   cd url-shortener
   ```

2. Install the dependencies:

   ```bash
   cargo build
   ```

3. Set up your PostgreSQL database and update the connection string in your
   environment variables or `.env` file.

4. Run the database migrations:

   ```bash
   cargo run --bin sqlx migrate run
   ```

5. Start the server:

   ```bash
   cargo run
   ```

The server will start on `http://localhost:8080`.

## API Endpoints

### Shorten a URL

- **POST** `/shorten`

  **Request Body:**
  ```json
  {
      "url": "https://example.com"
  }
  ```

  **Response:**
  ```json
  {
      "short_url": "http://localhost:8080/abc123",
      "long_url": "https://example.com"
  }
  ```

### Redirect to Long URL

- **GET** `/{short_url}`

  **Response:** Redirects to the original long URL.

### Get All Shortened URLs

- **GET** `/all`

  **Response:**
  ```json
  [
      {
          "short_url": "http://localhost:8080/abc123",
          "long_url": "https://example.com"
      },
      ...
  ]
  ```

## Database Migration

The project includes a SQL migration file located in the `migrations` directory.
It creates a `urls` table to store the shortened URLs and their corresponding
long URLs.

To run the migration, use the following command:

```bash
bash
cargo run --bin sqlx migrate run
```

## Contributing

Contributions are welcome! If you have suggestions for improvements or new
features, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file
for details.
