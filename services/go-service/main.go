package main

import (
	"context"
	"fmt"
	"log"

	cloudevents "github.com/cloudevents/sdk-go/v2"
	"github.com/cloudevents/sdk-go/v2/protocol"
	"github.com/google/uuid"
)

type Example struct {
	Sequence int    `json:"id"`
	Message  string `json:"message"`
}

func receive(ctx context.Context, incomingEvent cloudevents.Event) (*cloudevents.Event, protocol.Result) {
	// do something with event.
	fmt.Printf("Incoming event - %s\n", incomingEvent)
	var example Example
	if err := incomingEvent.DataAs(&example); err != nil {
		panic(err)
	}

	fmt.Printf("%+v\n", example)

	example.Message = fmt.Sprintf("%s - hello from go-service", example.Message)

	responseEvent := cloudevents.NewEvent()
	responseEvent.SetID(uuid.New().String())
	responseEvent.SetSource("https://go-service")
	responseEvent.SetType("reply.to")
	responseEvent.SetData("application/json", example)

	return &responseEvent, nil
}

func main() {
	// The default client is HTTP.
	c, err := cloudevents.NewDefaultClient()
	if err != nil {
		log.Fatalf("failed to create client, %v", err)
	}

	log.Println("Listening on port 8080")
	log.Fatal(c.StartReceiver(context.Background(), receive))
}
