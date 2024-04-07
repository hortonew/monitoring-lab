package main

import (
	"fmt"
	"sync"
	"time"

	"github.com/hashicorp/vault/api"
)

var (
	initialConcurrency = 10
	maxConcurrency     = 10000
	vaultAddress       = "http://vault:8200"

	// PID coefficients - Adjust these based on testing and observation
	kp = 100.0
	ki = 60.0
	kd = 75.0

	// Controller state
	integral    float64
	lastError   float64
	targetError = 0.01 // Target error rate (1%)
)

func main() {
	time.Sleep(5 * time.Second)
	client, _ := createVaultClient(vaultAddress)
	writeInitialSecret(client)

	currentConcurrency := initialConcurrency

	for {
		fmt.Printf("Testing with %d concurrent requests.\n", currentConcurrency)
		avgLatency, errRate := loadTestVault(client, currentConcurrency)
		fmt.Printf("Average latency: %v, Error rate: %v%%\n", avgLatency, errRate*100)

		// Calculate PID
		error := targetError - errRate
		integral += error
		derivative := error - lastError
		adjustment := kp*error + ki*integral + kd*derivative
		fmt.Printf("PID Adjustment: %v (P: %v, I: %v, D: %v)\n", adjustment, kp*error, ki*integral, kd*derivative)

		// Apply PID output to adjust concurrency
		newConcurrency := currentConcurrency + int(adjustment)
		if newConcurrency > maxConcurrency {
			newConcurrency = maxConcurrency
		} else if newConcurrency < 1 {
			newConcurrency = 1
		}
		currentConcurrency = newConcurrency

		lastError = error
		time.Sleep(1 * time.Second)
	}
}

func createVaultClient(address string) (*api.Client, error) {
	config := &api.Config{
		Address: address,
	}
	client, err := api.NewClient(config)
	if err != nil {
		return nil, err
	}
	client.SetToken("root")
	return client, nil
}

func writeInitialSecret(client *api.Client) error {
	data := map[string]interface{}{
		"data": map[string]interface{}{
			"value": "initial test data",
		},
	}
	_, err := client.Logical().Write("secret/data/test", data)
	return err
}

func loadTestVault(client *api.Client, concurrentRequests int) (time.Duration, float64) {
	var wg sync.WaitGroup
	wg.Add(concurrentRequests)

	responses := make(chan time.Duration, concurrentRequests)
	errors := make(chan error, concurrentRequests)

	for i := 0; i < concurrentRequests; i++ {
		go func() {
			defer wg.Done()
			start := time.Now()
			_, err := client.Logical().Read("secret/data/test")
			elapsed := time.Since(start)
			if err != nil {
				errors <- err
				return
			}
			responses <- elapsed
		}()
	}
	wg.Wait()

	var totalLatency time.Duration
	count := 0
	for {
		select {
		case resp := <-responses:
			totalLatency += resp
			count++
		default:
			if count == 0 {
				return 0, 1 // Return maximum error rate if no successful requests
			}
			avgLatency := totalLatency / time.Duration(count)
			errCount := len(errors)
			errRate := float64(errCount) / float64(concurrentRequests)
			return avgLatency, errRate
		}
	}
}
