package function

import (
	"fmt"
	"log"
	"net/http"

	"github.com/AGPFVEN/Time-Logger/backend/utils"

	"github.com/GoogleCloudPlatform/functions-framework-go/funcframework"
)

// Creates a project inside the db
func createProject(w http.ResponseWriter, r *http.Request) {
	// Set up logger
	l := log.New(funcframework.LogWriter(r.Context()), "", 0)

	// Connect to the db
	l.Println("Connecting to the db...")
	conn, err := utils.ConnectDB("")
	if err != nil {
		http.Error(w, fmt.Sprintf("Database connection failed: %v", err),
		http.StatusInternalServerError)
		return
	}
	l.Println("Connected to db")
	defer utils.CloseDB(conn)
	
	// Creating project
	l.Println("Creating project in DB...")
	
}
