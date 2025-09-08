#!/bin/bash

echo "🚀 Magic Resume - Rust Edition Demo"
echo "=================================="

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first."
    exit 1
fi

echo "✅ Rust/Cargo found"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Not in the magic-resume-rust directory"
    exit 1
fi

echo "📦 Building the application..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build successful!"
echo ""
echo "📋 Application Information:"
echo "   - Binary location: ./target/release/magic_resume"
echo "   - Binary size: $(du -h ./target/release/magic_resume | cut -f1)"
echo "   - Framework: Axum (Rust)"
echo "   - Database: SQLite"
echo "   - Default port: 3000"
echo ""

echo "🌐 Demo Endpoints:"
echo "   - http://localhost:3000/          (Interactive demo page)"
echo "   - http://localhost:3000/health    (Health check)"
echo "   - http://localhost:3000/api/test  (API test)"
echo "   - http://localhost:3000/api/resumes (Resume API)"
echo ""

echo "🏃‍♂️ To run the application:"
echo "   ./target/release/magic_resume"
echo "   or"
echo "   cargo run"
echo ""

echo "🔧 Environment variables (optional):"
echo "   APP_SERVER_HOST=0.0.0.0"
echo "   APP_SERVER_PORT=3000"
echo "   APP_DATABASE_URL=sqlite://./magic_resume.db"
echo ""

echo "✨ Demo completed successfully!"
echo "The Next.js Magic Resume has been successfully rewritten in Rust!"