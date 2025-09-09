package experiments

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"sync"
	"time"

	"github.com/GoogleCloudPlatform/functions-framework-go/functions"
)

// Global database pool - initialized once per function instance
var (
	db   *sql.DB
	once sync.Once
)

// User represents a user in the database
type User struct {
	ID        int       `json:"id"`
	Email     string    `json:"email"`
	Name      string    `json:"name"`
	CreatedAt time.Time `json:"created_at"`
}

// RequestPayload for incoming requests
type RequestPayload struct {
	UserID int    `json:"user_id,omitempty"`
	Email  string `json:"email,omitempty"`
	Name   string `json:"name,omitempty"`
}

// Response structure
type Response struct {
	Success bool        `json:"success"`
	Data    interface{} `json:"data,omitempty"`
	Error   string      `json:"error,omitempty"`
}

// init runs during cold start - register the function
func init() {
	functions.HTTP("HandleRequest", HandleRequest)
}

// initDB initializes the database connection pool (called once)
func initDB() error {
	var err error
	
	// Get configuration from environment variables
	dbHost := os.Getenv("DB_HOST")
	dbPort := os.Getenv("DB_PORT")
	dbUser := os.Getenv("DB_USER")
	dbPassword := os.Getenv("DB_PASSWORD")
	dbName := os.Getenv("DB_NAME")
	
	// For Cloud SQL using Unix socket
	dbSocket := os.Getenv("DB_SOCKET") // e.g., /cloudsql/PROJECT_ID:REGION:INSTANCE_NAME
	
	var dsn string
	if dbSocket != "" {
		// Cloud SQL connection via Unix socket
		dsn = fmt.Sprintf("host=%s user=%s password=%s dbname=%s sslmode=disable",
			dbSocket, dbUser, dbPassword, dbName)
	} else {
		// Standard TCP connection
		if dbPort == "" {
			dbPort = "5432"
		}
		dsn = fmt.Sprintf("host=%s port=%s user=%s password=%s dbname=%s sslmode=require",
			dbHost, dbPort, dbUser, dbPassword, dbName)
	}
	
	// Open database connection pool
	db, err = sql.Open("postgres", dsn)
	if err != nil {
		return fmt.Errorf("failed to open database: %v", err)
	}
	
	// Configure the connection pool
	// Keep these values low for Cloud Functions
	db.SetMaxOpenConns(2)                  // Max 2 connections per instance
	db.SetMaxIdleConns(1)                  // Keep 1 idle connection
	db.SetConnMaxLifetime(30 * time.Minute) // Refresh connections every 30 min
	db.SetConnMaxIdleTime(10 * time.Minute) // Close idle connections after 10 min
	
	// Test the connection
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	
	if err := db.PingContext(ctx); err != nil {
		return fmt.Errorf("failed to ping database: %v", err)
	}
	
	log.Println("Database connection pool initialized successfully")
	return nil
}

// getDB returns the database connection pool, initializing if needed
func getDB() (*sql.DB, error) {
	var initErr error
	once.Do(func() {
		initErr = initDB()
	})
	
	if initErr != nil {
		return nil, initErr
	}
	
	return db, nil
}

// HandleRequest is the main Cloud Function entry point
func HandleRequest(w http.ResponseWriter, r *http.Request) {
	// Set response headers
	w.Header().Set("Content-Type", "application/json")
	
	// Handle different HTTP methods
	switch r.Method {
	case http.MethodGet:
		handleGetUser(w, r)
	case http.MethodPost:
		handleCreateUser(w, r)
	case http.MethodPut:
		handleUpdateUser(w, r)
	default:
		sendResponse(w, http.StatusMethodNotAllowed, Response{
			Success: false,
			Error:   "Method not allowed",
		})
	}
}

// handleGetUser retrieves a user by ID
func handleGetUser(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("id")
	if userID == "" {
		sendResponse(w, http.StatusBadRequest, Response{
			Success: false,
			Error:   "User ID is required",
		})
		return
	}
	
	// Get database connection pool
	database, err := getDB()
	if err != nil {
		log.Printf("Failed to get database: %v", err)
		sendResponse(w, http.StatusInternalServerError, Response{
			Success: false,
			Error:   "Database connection failed",
		})
		return
	}
	
	// Query with context timeout
	ctx, cancel := context.WithTimeout(r.Context(), 5*time.Second)
	defer cancel()
	
	var user User
	query := `SELECT id, email, name, created_at FROM users WHERE id = $1`
	err = database.QueryRowContext(ctx, query, userID).Scan(
		&user.ID, &user.Email, &user.Name, &user.CreatedAt,
	)
	
	if err == sql.ErrNoRows {
		sendResponse(w, http.StatusNotFound, Response{
			Success: false,
			Error:   "User not found",
		})
		return
	}
	
	if err != nil {
		log.Printf("Query failed: %v", err)
		sendResponse(w, http.StatusInternalServerError, Response{
			Success: false,
			Error:   "Failed to retrieve user",
		})
		return
	}
	
	sendResponse(w, http.StatusOK, Response{
		Success: true,
		Data:    user,
	})
}

// handleCreateUser creates a new user
func handleCreateUser(w http.ResponseWriter, r *http.Request) {
	var payload RequestPayload
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		sendResponse(w, http.StatusBadRequest, Response{
			Success: false,
			Error:   "Invalid request body",
		})
		return
	}
	
	if payload.Email == "" || payload.Name == "" {
		sendResponse(w, http.StatusBadRequest, Response{
			Success: false,
			Error:   "Email and name are required",
		})
		return
	}
	
	database, err := getDB()
	if err != nil {
		log.Printf("Failed to get database: %v", err)
		sendResponse(w, http.StatusInternalServerError, Response{
			Success: false,
			Error:   "Database connection failed",
		})
		return
	}
	
	ctx, cancel := context.WithTimeout(r.Context(), 5*time.Second)
	defer cancel()
	
	// Use a transaction for the insert
	tx, err := database.BeginTx(ctx, nil)
	if err != nil {
		log.Printf("Failed to begin transaction: %v", err)
		sendResponse(w, http.StatusInternalServerError, Response{
			Success: false,
			Error:   "Transaction failed",
		})
		return
	}
	defer tx.Rollback() // Will be no-op if commit succeeds
	
	var newUser User
	query := `
		INSERT INTO users (email, name, created_at) 
		VALUES ($1, $2, $3) 
		RETURNING id, email, name, created_at`
	
	err = tx.QueryRowContext(ctx, query, payload.Email, payload.Name, time.Now()).Scan(
		&newUser.ID, &newUser.Email, &newUser.Name, &newUser.CreatedAt,
	)
	
	if err != nil {
		log.Printf("Insert failed: %v", err)
		sendResponse(w, http.StatusInternalServerError, Response{
			Success: false,
			Error:   "Failed to create user",
		})
		return
	}
	
	if err := tx.Commit(); err != nil {
		log.Printf("Commit failed: %v", err)
		sendResponse(w, http.StatusInternalServerError, Response{
			Success: false,
			Error:   "Failed to save user",
		})
		return
	}
	
	sendResponse(w, http.StatusCreated, Response{
		Success: true,
		Data:    newUser,
	})
}

// handleUpdateUser updates an existing user
func handleUpdateUser(w http.ResponseWriter, r *http.Request) {
	var payload RequestPayload
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		sendResponse(w, http.StatusBadRequest, Response{
			Success: false,
			Error:   "Invalid request body",
		})
		return
	}
	
	if payload.UserID == 0 {
		sendResponse(w, http.StatusBadRequest, Response{
			Success: false,
			Error:   "User ID is required",
		})
		return
	}
	
	database, err := getDB()
	if err != nil {
		log.Printf("Failed to get database: %v", err)
		sendResponse(w, http.StatusInternalServerError, Response{
			Success: false,
			Error:   "Database connection failed",
		})
		return
	}
	
	ctx, cancel := context.WithTimeout(r.Context(), 5*time.Second)
	defer cancel()
	
	// Build dynamic update query
	query := `UPDATE users SET name = $1, email = $2 WHERE id = $3`
	result, err := database.ExecContext(ctx, query, payload.Name, payload.Email, payload.UserID)
	
	if err != nil {
		log.Printf("Update failed: %v", err)
		sendResponse(w, http.StatusInternalServerError, Response{
			Success: false,
			Error:   "Failed to update user",
		})
		return
	}
	
	rowsAffected, _ := result.RowsAffected()
	if rowsAffected == 0 {
		sendResponse(w, http.StatusNotFound, Response{
			Success: false,
			Error:   "User not found",
		})
		return
	}
	
	sendResponse(w, http.StatusOK, Response{
		Success: true,
		Data:    map[string]interface{}{"updated": true, "user_id": payload.UserID},
	})
}

// sendResponse sends JSON response
func sendResponse(w http.ResponseWriter, statusCode int, response Response) {
	w.WriteHeader(statusCode)
	json.NewEncoder(w).Encode(response)
}

// Cleanup function (optional) - Cloud Functions may not always call this
func cleanup() {
	if db != nil {
		db.Close()
		log.Println("Database connection pool closed")
	}
}