import { io } from "socket.io-client";
import chalk from "chalk";
import CryptoJS from 'crypto-js';

const { AES } = CryptoJS;

function devLog(...args) {
	// if (process.env.ENV === "dev") 
	console.log(chalk.bold(...args));
	// else return
}

function errorLog(...args) {
	console.error(chalk.redBright("Ignition error:", ...args))
}

export class Ignition {
	#socket;#encryptionKey;
	constructor(config) {
		this.url = config.url;
		this.#encryptionKey = config.encryptionKey;
		this.apiKey = config.key;
		this.groupId = undefined;
		this.groupCount = 0;

		this.#socket = io("http://localhost:4000", { // public shared ignition websocket server URL - Elastic Ip
			auth: {
				token: "abc123",
			}
		});

		this.#socket.on("ERROR", (message) => { errorLog(message) });
		this.#socket.on("LOG", (message) => { devLog(chalk.cyanBright(message)) });
		console.log(chalk.cyanBright("Ignition client connecting ........", this.url, this.apiKey));
		
	}

	ecrypt(message) {
		return AES.encrypt(message, this.#encryptionKey).toString();
	}

	decrypt(message) {
		return AES.decrypt(message, this.#encryptionKey).toString(CryptoJS.enc.Utf8);
	}

	async subscribe(groupId) {
		this.groupId = groupId;
		devLog("Attempting to subscribe to room !!");
		// pub struct JoinLeaveRequestData {
		//     pub key: String,
		//     pub group_id: String,  
		// }
		this.#socket.emit("JOIN", {
			key: this.apiKey,
			group_id: `${this.apiKey}_${groupId}`,
		},
		(data) => { // this would be recived by the server & the server will join this client int that room
			console.log(chalk.cyanBright(data));
			// this.#socket.disconnect()
		});
		// this.#socket.emit("GROUP", groupId);
		
	}

	async unsubscribe(groupId) {
		this.groupId = groupId; // set groupId as global class state
		devLog("Attempting to unsubscribe from room !!");
		this.#socket.emit("LEAVE", {
			key: this.apiKey,
			group_id: `${this.apiKey}_${groupId}`,
		}, 
		(data) => {
			console.log(chalk.cyanBright(data));
		});
	}

	// this method should only be used by dedictaed, dedictaed+ & enterprize clients
	// emit directly send message to the websocket server instaed appending it to the message queue.
	async emit(eventName, groupId, message) {
		if (this.url == undefined) {
			errorLog("You must have `URL` of a server to emit a direct message, try using `publish()` method for Shared users.")
		}

		if(typeof(message) == "object") {
			message = JSON.stringify(message)
		}

		console.log("EMITITING: -> ",this.#encryptionKey ? this.ecrypt(message) : message)

		// pub struct ClientMessage {
		//     pub group_id: String,
		//     pub event_name: String,
		//     pub message: String,
		//     pub key: String,
		// }

		devLog("EMITTING EVENT !!")
		this.#socket.emit("MESSAGE", {
			group_id: this.apiKey + "_" + groupId,
			event_name: eventName,
			message: this.#encryptionKey ? this.ecrypt(message) : message,
			key: this.apiKey
		})
	}

	async on(eventName, callback) {
		if (eventName != "connect" && eventName != "disconnect" && this.groupId == undefined) {
			errorLog("Missing `groupId`. Did you forgot to `subsribe` to a group ?");
		};
		this.#socket.on(eventName, callback);
	}

	async off(eventName, callback=undefined) {
		if (eventName != "connect" && eventName != "disconnect" && this.groupId == undefined) {
			errorLog("Missing `groupId`. Did you forgot to `subsribe` to a group ?");
		};
		this.#socket.off(eventName, callback);
	}

	// this method adds the message to the `message queue` from which shared websocket server 
	// keeps pulling and braodcasting the message to all the clients subscribed to the group.
	async publish(groupId, eventName, message) {
		try {
			const res = await fetch("ignition_application_server_url", {
				method: "POST",
				mode: "cors",
				cache: "no-cache",
				credentials: "same-origin",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					"group_id": groupId,
					"event_name": eventName,
					"message": this.#encryptionKey ? this.ecrypt(message) : message,
					"key": this.apiKey
				})
			})
			if (res.status != 200) {
				throw Error(`Failed to publish message, status code: ${res.status}`)
			}
		} catch (error) {
			errorLog(error)
		}
	}

}

// let hj = io("http://localhost:3000", {
// 	auth: {
// 		token: "abc123"
// 	}
// })

// hj.on("connect", () => {
// 	console.log("connected")
// })

// hj.on("server to client event", (data) => {
// 	console.log(data)
// })

// hj.emit("client to server event", "1111111111111111111122222222222222222222333333333333333333333333333334444444444444444")

let a = new Ignition({
	url: "http://localhost:4000",
	key:"abc123",
	// encryptionKey:"RADHA"
})

a.subscribe("test")

a.on("connect", () => {
	console.log("CONNECTED ")
})




let b = new Ignition({
	url: "ws://localhost:4000",
	key:"abc123",
	// encryptionKey:"RADHA"
})

b.subscribe("test")

// b.on("test", (data) => {
// 	console.log("message recived by `b`:",data)
// })

// b.emit("test", "test", "hello world")