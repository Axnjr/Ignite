import { Ignition } from "ignition-js-sdk"

const ws = new Ignition("abc123", "wss://localhost:3000")

const ws1 = new Ignition("abc1239", "ws://localhost:3000")

ws.subscribe("emails");
ws1.subscribe("emails");

ws.on("message", (data) => {
	console.log(data);
})
ws1.on("message", (data) => {
	console.log(data);
})

// ws1.emit("message", "emails", "YOUR DATA FROM THE SERVER !")