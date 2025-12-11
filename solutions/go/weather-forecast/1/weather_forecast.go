// Package weather provides weather forecast for the a city given the weather conditions.
package weather

var (
    // CurrentCondition gives the current weather information for a specified location. 
	CurrentCondition string
    // CurrentLocation gives the information regarding the location.
	CurrentLocation  string
)

// Forecast functions takes two variables(city, condition), It helps in forcasting the weather for a city given the current weather conditions for that city.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
