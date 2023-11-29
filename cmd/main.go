package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func main() {
	router := gin.Default()
	s := &http.Server{
		Addr:    ":8081",
		Handler: router,
	}
	router.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})
	s.ListenAndServe()
	// router.Run() // listen and serve on 0.0.0.0:8080 (for windows "localhost:8080")
}
