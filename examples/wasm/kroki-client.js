/** Client for kroki diagram service (https://kroki.io/).
 *
 * Shamelessly copied from: https://github.com/Mogztter/asciidoctor-kroki
 */

// use pako deflate for compressing the diagram
import "https://unpkg.com/pako@1.0.10/dist/pako_deflate.min.js";

// Class representing the data about a diagram
export class KrokiDiagram {
  constructor (type, format, text, opts) {
    this.text = text
    this.type = type
    this.format = format
    this.opts = opts
  }

  // Get the uri from which the diagram can be retrieved using get
  getDiagramUri (serverUrl) {
    const queryParams = Object.entries(this.opts).map(([key, value]) => `${key}=${encodeURIComponent(value.toString())}`).join('&')
    return `${serverUrl}/${this.type}/${this.format}/${this.encode()}${queryParams ? `?${queryParams}` : ''}`
  }

  // Encode the diagram from utf-8
  textEncode() {
    if (window.TextEncoder) {
      return new TextEncoder('utf-8').encode(this.text);
    }
    var utf8 = unescape(encodeURIComponent(this.text));
    var result = new Uint8Array(utf8.length);
    for (var i = 0; i < utf8.length; i++) {
      result[i] = utf8.charCodeAt(i);
    }
    return result;
  }

  // Encode diagram for kroki (utf-8 + compress + base64)
  encode () {
    var compressed = pako.deflate(this.textEncode(), { level: 9, to: 'string' })
    return btoa(compressed)
        .replace(/\+/g, '-').replace(/\//g, '_');
  }
}

// Client using fetch to retreive kroki diagrams
export class KrokiClient {
  constructor () {
  }

  getTextContent (krokiDiagram) {
    return this.getImage(krokiDiagram, 'utf8')
  }

  // Get the image for the diagram
  getImage (krokiDiagram, encoding) {
    const serverUrl = this.getServerUrl()
    const type = krokiDiagram.type
    const format = krokiDiagram.format
    const text = krokiDiagram.text
    const opts = krokiDiagram.opts
    const headers = {
      ...Object.fromEntries(Object.entries(opts).map(([key, value]) => [`Kroki-Diagram-Options-${key}`, value]))
    }
    const uri = krokiDiagram.getDiagramUri(serverUrl)
    return fetch(uri, {headers: headers})
  }

  getServerUrl () {
    return 'https://kroki.io'
  }
}
