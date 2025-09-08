# Magic Resume - Rust Edition

A modern, fast resume builder built with Rust and the Silent framework. This is a complete rewrite of the original Next.js Magic Resume application.

## ✨ Features

- 🚀 **Fast & Efficient**: Built with Rust for maximum performance
- 📝 **Resume Builder**: Intuitive interface for creating professional resumes
- 🤖 **AI-Powered**: Grammar checking and text polishing with OpenAI integration
- 📄 **PDF Export**: Generate beautiful PDF resumes
- 💾 **Persistent Storage**: SQLite database for reliable data storage
- 🎨 **Modern UI**: Clean, responsive interface
- 🔧 **Easy Deployment**: Single binary deployment

## 🛠️ Technology Stack

- **Backend**: Rust + Silent Framework
- **Database**: SQLite with SQLx
- **PDF Generation**: printpdf
- **AI Integration**: OpenAI API
- **Frontend**: Vanilla JavaScript with modern CSS
- **Build System**: Cargo

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ installed
- Optional: OpenAI API key for AI features

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd magic-resume-rust
```

2. Copy the environment file and configure:
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. Build and run:
```bash
cargo build --release
cargo run
```

The application will be available at `http://localhost:3000`

### Configuration

Configure the application using environment variables or the `.env` file:

- `APP_SERVER_HOST`: Server host (default: 0.0.0.0)
- `APP_SERVER_PORT`: Server port (default: 3000)
- `APP_DATABASE_URL`: Database connection string (default: sqlite://./magic_resume.db)
- `APP_AI_OPENAI_API_KEY`: OpenAI API key for AI features (optional)
- `APP_AI_GRAMMAR_CHECK_ENABLED`: Enable grammar checking (default: true)
- `APP_AI_POLISH_ENABLED`: Enable text polishing (default: true)

## 📚 API Endpoints

### Resumes
- `POST /api/resumes` - Create a new resume
- `GET /api/resumes` - List user resumes
- `GET /api/resumes/:id` - Get resume by ID
- `PUT /api/resumes/:id` - Update resume
- `DELETE /api/resumes/:id` - Delete resume
- `GET /api/resumes/:id/export/pdf` - Export resume as PDF

### AI Features
- `POST /api/grammar` - Check grammar
- `POST /api/polish` - Polish text

### Static Files
- `GET /` - Serve main application
- `GET /static/*` - Serve static assets

## 🏗️ Project Structure

```
src/
├── main.rs              # Application entry point
├── config/              # Configuration management
├── models/              # Data models
├── handlers/            # HTTP request handlers
├── services/            # Business logic services
└── utils/               # Utility functions

static/                  # Frontend assets
├── index.html          # Main HTML file
├── css/                # Stylesheets
└── js/                 # JavaScript files

migrations/             # Database migrations
templates/              # HTML templates (if using Tera)
```

## 🔧 Development

### Running in Development Mode

```bash
cargo run
```

### Building for Production

```bash
cargo build --release
```

The optimized binary will be available at `target/release/magic_resume`.

### Database Migrations

Migrations are automatically run on startup. To create a new migration:

1. Add a new SQL file in the `migrations/` directory
2. Follow the naming convention: `XXX_description.sql`

## 🚀 Deployment

### Binary Deployment

1. Build the release binary:
```bash
cargo build --release
```

2. Copy the binary and static files to your server:
```bash
cp target/release/magic_resume /path/to/deployment/
cp -r static /path/to/deployment/
```

3. Set environment variables and run:
```bash
export APP_SERVER_PORT=8080
export APP_DATABASE_URL=sqlite:///path/to/database.db
./magic_resume
```

### Docker Deployment

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/magic_resume .
COPY --from=builder /app/static ./static
EXPOSE 3000
CMD ["./magic_resume"]
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the Apache 2.0 License - see the LICENSE file for details.

## 🔄 Migration from Next.js Version

This Rust version provides the same functionality as the original Next.js Magic Resume with improved performance and resource efficiency. Key differences:

- **Performance**: Significantly faster response times and lower memory usage
- **Deployment**: Single binary deployment vs Node.js application
- **Dependencies**: Minimal runtime dependencies
- **Scalability**: Better handling of concurrent requests

## 🐛 Troubleshooting

### Common Issues

1. **Database connection errors**: Check the `APP_DATABASE_URL` environment variable
2. **AI features not working**: Verify your OpenAI API key is set correctly
3. **Static files not loading**: Ensure the `APP_STATIC_FILES_DIR` path is correct
4. **PDF export failing**: Check that the application has write permissions

### Logs

Enable detailed logging by setting the `RUST_LOG` environment variable:
```bash
export RUST_LOG=debug
cargo run
```

## 📞 Support

For issues and questions:
- Create an issue on GitHub
- Check the documentation
- Review the troubleshooting section

---

Built with ❤️ using Rust and the Silent framework.