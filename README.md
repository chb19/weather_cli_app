# weather_cli_app
It is a weather CLI for Windows/Linux/macOS, which is responsible for showing weather to a user. 

It supports 2 different weather providers:
* [OpenWeather](https://openweathermap.org)
* [weatherapi](https://www.weatherapi.com)  

To launch program use: 
 * `cargo run --bin weather_cli_app`

Supported commands: 
```
weather configure <provider>
weather get <address> [date]
weather help
```

Example of usage: 
```
weather get nadvirna
weather configure open-weather-map
weather get nadvirna 2023-02-16T04:04:04
```

