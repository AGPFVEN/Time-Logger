package utils

import (
	"context"
	"fmt"
	"os"

	"github.com/jackc/pgx/v5"
)

// ConnectDB connects to PostgreSQL and returns the connection
func ConnectDB(dbURL string) (*pgx.Conn, error) {
	if dbURL == "" {
		dbURL = fmt.Sprintf("postgres://%s:%s@%s:%s/%s?sslmode=disable", 
		os.Getenv("DB_USER"),
		os.Getenv("DB_PASSWORD"),
		os.Getenv("DB_URL"),
		os.Getenv("DB_LOCAL_PORT"),
		os.Getenv("DB_SELECTED"))
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
