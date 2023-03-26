# WPA-Supplicant-Creator


Anyone who has used a Raspberry PI, while often switching thier wifi network, has experienced the pain of constantly needing to create a `wpa_supplement.conf` file. This small tool tries to make creating these easier. It automatically creates the wpa_supplicant.conf file with your current wifi network and country.


# Usage
![Screenshot 2023-03-26 214943](https://user-images.githubusercontent.com/46920345/227800854-ecd6e5fe-7bd5-44ba-9a90-42736860a387.png)

- Detect: Try to automatically fill the fields based on your currently connected wifi, your region and your connected drives
- Country: Your region as an alpha 2 country code (`AT`, `DE`, `US`, `AU`)
- Wifi SSID: The SSID of the wifi you'd like the Raspberry Pi to connect
- Wifi Password: The Password of the wifi you'd like the Raspberry Pi to connect
- Output File Path: The path and name of the config file
- Create Config File: Creates the config file
