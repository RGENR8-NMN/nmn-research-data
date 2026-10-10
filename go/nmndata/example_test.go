package nmndata_test

import (
	"fmt"

	"github.com/RGENR8-NMN/nmn-research-data/go/nmndata"
)

func ExampleHumanTrials() {
	fmt.Println(len(nmndata.HumanTrials()))
	// Output: 25
}

func ExamplePricePerGram() {
	// 60 capsules of 500 mg for 39.99 GBP.
	fmt.Printf("%.2f\n", nmndata.PricePerGram(39.99, 500, 60))
	// Output: 1.33
}

func ExampleCostPerDay() {
	fmt.Printf("%.2f\n", nmndata.CostPerDay(39.99, 500, 60, 500))
	// Output: 0.67
}
