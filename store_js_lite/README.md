# AHQ Store JS.Lite
A Core Package for interacting with the AHQ Store Framework (the framework uses electron under the hood)

## Quick Example (Not Tested)
```js
const ahqstore = require("ahqstorejs.lite");
// If debug
ahqstore.debug().start();

// If Installed as an ahq store app
ahqstore.start();
```
## Constants

<dl>
<dt><a href="#version">version</a> : <code>string</code></dt>
<dd></dd>
</dl>

## Functions

<dl>
<dt><a href="#debug">debug()</a> ⇒ <code>Object</code></dt>
<dd><p>Use Debug Data</p>
</dd>
<dt><a href="#start">start()</a> ⇒ <code>Promise.&lt;void&gt;</code></dt>
<dd><p>Tries to connect to AHQ Store Framework Agent
Resolves if it succeeds, rejects if it fails</p>
</dd>
<dt><a href="#listenWs">listenWs(fn)</a> ⇒ <code>number</code></dt>
<dd><p>Listen to WebSocket Events</p>
</dd>
<dt><a href="#sendWsMessage">sendWsMessage(data)</a></dt>
<dd><p>Sends a message to the electron app</p>
</dd>
<dt><a href="#getRawWs">getRawWs()</a> ⇒ <code>WebSocket</code></dt>
<dd><p>Get the raw ws that is being wrapped</p>
</dd>
</dl>

<a name="version"></a>

## version : <code>string</code>
**Kind**: global constant  
<a name="debug"></a>

## debug() ⇒ <code>Object</code>
Use Debug Data

**Kind**: global function  
<a name="start"></a>

## start() ⇒ <code>Promise.&lt;void&gt;</code>
Tries to connect to AHQ Store Framework Agent
Resolves if it succeeds, rejects if it fails

**Kind**: global function  
<a name="listenWs"></a>

## listenWs(fn) ⇒ <code>number</code>
Listen to WebSocket Events

**Kind**: global function  
**Returns**: <code>number</code> - total number of listeners  

| Param | Type |
| --- | --- |
| fn | <code>function</code> | 

<a name="sendWsMessage"></a>

## sendWsMessage(data)
Sends a message to the electron app

**Kind**: global function  

| Param | Type |
| --- | --- |
| data | <code>Object</code> | 

<a name="getRawWs"></a>

## getRawWs() ⇒ <code>WebSocket</code>
Get the raw ws that is being wrapped

**Kind**: global function  
