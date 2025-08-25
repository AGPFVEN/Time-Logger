package main

import (
	"log"
	"os"
	"path/filepath"

	// Blank-import the function package so the init() runs
	_ "time-logger/API"

	"github.com/GoogleCloudPlatform/functions-framework-go/funcframework"
	"github.com/lpernett/godotenv"
)

func main() {
	// Load local envs
	envPath, _ := filepath.Abs("../.env")
	err := godotenv.Load(envPath)
  	if err != nil {
    	log.Fatal("Error loading .env file")
  	}

	// Use PORT environment variable, or default to 8080.
	port := "8080"
	if envPort := os.Getenv("BACKEND_PORT"); envPort != "" {
		port = envPort
	}
	
	// By default, listen on all interfaces. If testing locally, run with 
	// LOCAL_ONLY=true to avoid triggering firewall warnings and 
	// exposing the server outside of your own machine.
	hostname := ""
	if localOnly := os.Getenv("LOCAL_ONLY"); localOnly == "true" {
		hostname = "127.0.0.1"
	} 
	if err := funcframework.StartHostPort(hostname, port); err != nil {
		log.Fatalf("funcframework.StartHostPort: %v\n", err)
	}
}