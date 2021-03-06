// GENERATED BY THE COMMAND ABOVE; DO NOT EDIT
// This file was generated by swaggo/swag

package docs

import (
	"bytes"
	"encoding/json"
	"strings"

	"github.com/alecthomas/template"
	"github.com/swaggo/swag"
)

var doc = `{
    "schemes": {{ marshal .Schemes }},
    "swagger": "2.0",
    "info": {
        "description": "{{.Description}}",
        "title": "{{.Title}}",
        "termsOfService": "http://swagger.io/terms/",
        "contact": {
            "name": "Burak Selim Şenyurt",
            "url": "https://www.buraksenyurt.com",
            "email": "selim@buraksenyurt.com"
        },
        "license": {},
        "version": "{{.Version}}"
    },
    "host": "{{.Host}}",
    "basePath": "{{.BasePath}}",
    "paths": {
        "/quote/": {
            "get": {
                "produces": [
                    "application/json"
                ],
                "summary": "Tüm kitap alıntılarını döndürür",
                "responses": {
                    "200": {
                        "description": "OK",
                        "schema": {
                            "type": "array",
                            "items": {
                                "$ref": "#/definitions/main.quote"
                            }
                        }
                    },
                    "500": {}
                }
            },
            "post": {
                "consumes": [
                    "application/json"
                ],
                "produces": [
                    "application/json"
                ],
                "summary": "Yeni bir kitap alıntısı ekler",
                "parameters": [
                    {
                        "description": "Alıntı Bilgileri",
                        "name": "quote",
                        "in": "body",
                        "required": true,
                        "schema": {
                            "$ref": "#/definitions/main.quote"
                        }
                    }
                ],
                "responses": {
                    "200": {},
                    "400": {},
                    "500": {}
                }
            }
        }
    },
    "definitions": {
        "main.quote": {
            "type": "object",
            "properties": {
                "book": {
                    "type": "string"
                },
                "description": {
                    "type": "string"
                },
                "id": {
                    "type": "string"
                },
                "writer": {
                    "type": "string"
                }
            }
        }
    }
}`

type swaggerInfo struct {
	Version     string
	Host        string
	BasePath    string
	Schemes     []string
	Title       string
	Description string
}

// SwaggerInfo holds exported Swagger Info so clients can modify it
var SwaggerInfo = swaggerInfo{
	Version:     "1.0",
	Host:        "localhost:5003",
	BasePath:    "/api/v1",
	Schemes:     []string{},
	Title:       "BookWorm Swagger API",
	Description: "Servis kullanım rehberidir",
}

type s struct{}

func (s *s) ReadDoc() string {
	sInfo := SwaggerInfo
	sInfo.Description = strings.Replace(sInfo.Description, "\n", "\\n", -1)

	t, err := template.New("swagger_info").Funcs(template.FuncMap{
		"marshal": func(v interface{}) string {
			a, _ := json.Marshal(v)
			return string(a)
		},
	}).Parse(doc)
	if err != nil {
		return doc
	}

	var tpl bytes.Buffer
	if err := t.Execute(&tpl, sInfo); err != nil {
		return doc
	}

	return tpl.String()
}

func init() {
	swag.Register(swag.Name, &s{})
}
