package utils

import (
	"context"
	"fmt"

	"github.com/jackc/pgx/v5"
)

// ConnectDB connects to PostgreSQL and returns the connection
func ConnectDB(dbURL string) (*pgx.Conn, error) {
	if dbURL == "" {
		dbURL = "postgres://app:secret@localhost:5433/appdb?sslmode=disable"
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
