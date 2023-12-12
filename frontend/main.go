package main

import (
	"github.com/jetroberts/fina/templates"
	"net/http"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, req *http.Request) {
		hello := templates.Index()
		hello.Render(req.Context(), w)
	})

	dir := http.Dir("./assets")
	fs := http.FileServer(dir)

	http.Handle("/assets/", http.StripPrefix("/assets/", fs))

	println("Listening on 8080")
	err := http.ListenAndServe("127.0.0.1:8080", nil)
	if err != nil {
		panic(err)
	}
}
