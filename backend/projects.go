package function

import (
	"net/http"
	"fmt"

	"time-logger/API/utils"
)

// Creates a project inside the db
func createProject(w http.ResponseWriter, r *http.Request) {
	conn, err := utils.ConnectDB("")
	if err != nil {
		http.Error(w, fmt.Sprintf("Database connection failed: %v", err), http.StatusInternalServerError)
		return
	}
	defer utils.CloseDB(conn)
	
	fmt.Fprintln(w, "Operation to create project")
	
}
