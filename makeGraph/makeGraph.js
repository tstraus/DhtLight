var program = require('commander')
var path = require('path')
var fs = require('fs')
var papa = require('papaparse')
var plotly = require('plotly')('tstraus', 'gw9TxI4Nsdz3cW7iZDoR')
var _ = require('lodash')

program.version('0.1.0')
  .option('-f, --file [file]', 'The data file to open', 'file')
  .parse(process.argv)

try {
  var dataFile = fs.readFileSync(program.file, 'utf-8')

  papa.parse(dataFile, {
    header: true,
    delimiter: ', ',
    complete: function (results) {
      let x = _.range(1, results.data.length)
      var light = []
      var temp = []
      var humidity = []

      results.data.forEach(function (entry) {
        light.push(Number(entry.light) / 10.0)
        temp.push(Number(entry['temp(F)']))
        humidity.push(Number(entry['humidity(%)']))
      })

      var lightTrace = {
        x: x,
        y: light,
        mode: 'lines',
        name: 'Light'
      }

      var tempTrace = {
        x: x,
        y: temp,
        mode: 'lines',
        name: 'Temperature (F)'
      }

      var humidityTrace = {
        x: x,
        y: humidity,
        mode: 'lines',
        name: 'Humidity (%)'
      }

      var figure = { 'data': [lightTrace, tempTrace, humidityTrace] }

      var imgOpts = {
        format: 'svg',
        width: 3000,
        height: 1000
      }

      plotly.getImage(figure, imgOpts, function (error, imageStream) {
        if (error) return console.log(error)

        var fileStream = fs.createWriteStream('./data/svg/' + path.parse(program.file).name + '.svg')
        imageStream.pipe(fileStream)
      })
    }
  })
} catch (e) {
  console.log('Error:', e.stack)
}
