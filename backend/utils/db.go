package utils

import (
	"context"
	"fmt"
	"log"
	"os"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

// initPool initializes the pgx connection pool (called once)
func initPool() (*pgxpool.Pool, error) {
	var err error

	// Build connection string from environment variables
	dbURL := os.Getenv("DATABASE_URL") // Can use a single URL if preferred

	if dbURL == "" {
		// Build URL from individual components
		dbUser := os.Getenv("DB_USER")
		dbPassword := os.Getenv("DB_PASSWORD")
		dbHost := os.Getenv("DB_HOST")
		dbPort := os.Getenv("DB_LOCAL_PORT")
		dbName := os.Getenv("DB_NAME")

		// For Cloud SQL using Unix socket
		dbSocket := os.Getenv("DB_SOCKET") // e.g., /cloudsql/PROJECT_ID:REGION:INSTANCE_NAME

		if dbPort == "" {
			dbPort = "5432"
		}

		if dbSocket != "" {
			// Cloud SQL connection via Unix socket
			dbURL = fmt.Sprintf("postgres://%s:%s@/%s?host=%s&sslmode=disable",
				dbUser, dbPassword, dbName, dbSocket)
		} else {
			// Standard TCP connection
			dbURL = fmt.Sprintf("postgres://%s:%s@%s:%s/%s?sslmode=require",
				dbUser, dbPassword, dbHost, dbPort, dbName)
		}
	}

	// Parse the configuration
	config, err := pgxpool.ParseConfig(dbURL)
	if err != nil {
		return nil, fmt.Errorf("failed to parse database URL: %v", err)
	}

	// Configure the connection pool for Cloud Functions
	// Keep these values low since each function instance handles one request at a time
	config.MaxConns = 2                        // Maximum 2 connections
	config.MinConns = 1                        // Don't maintain idle connections
	config.MaxConnLifetime = 15 * time.Minute  // Refresh connections every 30 min
	config.MaxConnIdleTime = 30 * time.Minute  // Close idle connections after 10 min
	config.HealthCheckPeriod = 1 * time.Minute // Check connection health every minute

	// Connection configuration
	config.ConnConfig.ConnectTimeout = 5 * time.Second

	// Add custom configuration for each connection
	config.BeforeConnect = func(ctx context.Context, cfg *pgx.ConnConfig) error {
		// Can add custom logic here if needed
		log.Println("Establishing new connection to database...")
		return nil
	}

	config.AfterConnect = func(ctx context.Context, conn *pgx.Conn) error {
		// Can register custom types, prepare statements, etc.
		log.Println("Connection established successfully...")
		return nil
	}

	// Create the connection pool
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	dbpool, err := pgxpool.New(context.Background(), os.Getenv("DATABASE_URL"))
	if err != nil {
		fmt.Fprintf(os.Stderr, "Unable to create connection pool: %v\n", err)
		os.Exit(1)
	}

	// Test the connection
	if err := dbpool.Ping(ctx); err != nil {
		return nil, fmt.Errorf("failed to ping database: %v", err)
	}

	// Optional: Prepare frequently used statements for better performance
	// This happens once per connection in the pool
	config.AfterConnect = func(ctx context.Context, conn *pgx.Conn) error {
		_, err := conn.Prepare(ctx, "get_user", `
			SELECT id, email, name, created_at FROM users WHERE id = $1
		`)
		if err != nil {
			return fmt.Errorf("failed to prepare get_user statement: %v", err)
		}

		_, err = conn.Prepare(ctx, "create_user", `
			INSERT INTO users (email, name, created_at)
			VALUES ($1, $2, $3)
			RETURNING id, email, name, created_at
		`)
		if err != nil {
			return fmt.Errorf("failed to prepare create_user statement: %v", err)
		}

		return nil
	}

	log.Printf("Database connection pool initialized successfully (Max: %d, Min: %d)",
		config.MaxConns, config.MinConns)
	return dbpool, nil
}

// ConnectDB connects to PostgreSQL and returns the connection
func ConnectDB(dbURL string) (*pgx.Conn, error) {
	if dbURL == "" {
		dbURL = fmt.Sprintf(
			"postgres://%s:%s@%s:%s/%s?sslmode=disable",
			os.Getenv("DB_USER"),
			os.Getenv("DB_PASSWORD"),
			os.Getenv("DB_HOST"),
			os.Getenv("DB_LOCAL_PORT"),
			os.Getenv("DB_NAME"))
	}

	conn, err := pgx.Connect(context.Background(), dbURL)
	if err != nil {
		return nil, fmt.Errorf("unable to connect to database: %w", err)
	}

	return conn, nil
}

// CloseDB closes the database connection
func CloseDB(conn *pgx.Conn) error {
	err := conn.Close(context.Background())
	if err != nil {
		return fmt.Errorf("unable to connect to database: %w", err)
	}
	return nil
}
