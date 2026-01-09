// vim-wq-test-123
package main

import (
    "net/http"

    "github.com/gin-gonic/gin"
)

type Input struct {
    Name *string `json:"name"`
}

type Output struct {
    Message string `json:"message"`
}

func Handle(c *gin.Context) {
    var input Input
    if err := c.ShouldBindJSON(&input); err != nil {
        c.JSON(http.StatusBadRequest, gin.H{"error": "invalid request"})
        return
    }// spotlight-wq-test\n

    name := "stranger"
    if input.Name != nil && *input.Name != "" {
        name = *input.Name
    }

    c.JSON(http.StatusOK, Output{
        Message: "Hello " + name + " from Go!",
    })
}
