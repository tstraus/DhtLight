#include "ESP8266WiFi.h"
#include "WiFiUDP.h"
#include "DHT.h"

#define DHTPIN 4
#define DHTTYPE DHT22

WiFiUDP client;
DHT dht(DHTPIN, DHTTYPE);

const char* ssid = "TP-LINK_9F7D";
const char* password = "64137896";

int light = 0;
float temp = 0.0f;
float humidity = 0.0f;
float heatIndex = 0.0f;

char buffer[64];

void setup() {
  Serial.begin(9600);

  Serial.print("Connecting to ");
  Serial.println(ssid);
  WiFi.mode(WIFI_STA);
  WiFi.begin(ssid, password);

  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }

  client.begin(1234);

  Serial.println("WiFi connected");  
  Serial.print("IP address: ");
  Serial.println(WiFi.localIP());
}

void loop() {
  digitalWrite(LED_BUILTIN, LOW);

  // scan for networks
  /*int n = WiFi.scanNetworks();
  if (n == 0)
    Serial.println("no networks found");
  else
  {
    Serial.print(n);
    Serial.println(" networks found");
    
    for (int i = 0; i < n; ++i)
    {
      // Print SSID and RSSI for each network found
      Serial.print(i + 1);
      Serial.print(": ");
      Serial.print(WiFi.SSID(i));
      Serial.print(" (");
      Serial.print(WiFi.RSSI(i));
      Serial.print(")");
      Serial.println((WiFi.encryptionType(i) == ENC_TYPE_NONE)?" ":"*");
      delay(10);
    }
  }*/

  // read from the photoresistor
  light = analogRead(A0);
  Serial.println("----------");
  
  // read from the DHT22
  temp = dht.readTemperature(true);
  humidity = dht.readHumidity();
  heatIndex = dht.computeHeatIndex(temp, humidity);

  if (!isnan(temp) && !isnan(humidity) && !isnan(heatIndex))
  {
    Serial.println(light);
    Serial.println(temp);
    Serial.println(humidity);
    Serial.println(heatIndex);
  
    client.beginPacket("192.168.0.107", 1234);
    client.write("{\"light\":");
    itoa(light, buffer, 10);
    client.write(buffer);
    client.write(",\"temp\":");
    snprintf(buffer, sizeof buffer, "%f", temp);
    client.write(buffer);
    client.write(",\"humidity\":");
    snprintf(buffer, sizeof buffer, "%f", humidity);
    client.write(buffer);
    client.write(",\"heatIndex\":");
    snprintf(buffer, sizeof buffer, "%f", heatIndex);
    client.write(buffer);
    client.write("}");
    client.endPacket();
  }
  
  delay(2500);
}