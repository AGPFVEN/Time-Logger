package function

import (
	"fmt"
	"net/http"
	"sync"

	"github.com/GoogleCloudPlatform/functions-framework-go/functions"
	"github.com/jackc/pgx/v5/pgxpool"
)

// Global database pool - initialized once per function instance
var (
	pool *pgxpool.Pool
	once sync.Once
)

func init() {
	functions.HTTP("HelloWorld", helloWorld)
	functions.HTTP("CreateProject", createProject)
}

// helloWorld writes "Hello, World!" to the HTTP response.
func helloWorld(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "Hello, World!")
}