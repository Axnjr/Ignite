import { Ignition } from "ignition-js-sdk"

const ws = new Ignition("abc123", "ws://localhost:3001")

const ws1 = new Ignition("abc123", "ws://localhost:3001")

ws.subscribe("ignition-site-room");

ws.on("message", (data) => {
	console.log(data);
})

ws1.emit("message", "ignition-site-room", "YOUR DATA FROM THE SERVER !")