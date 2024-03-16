import { io } from "socket.io-client";
import chalk from "chalk";

export class Spark {

	#apiKey;
	#socket;
	#state;
	#groupId;

	constructor(key) {
		this.#apiKey = key;
		this.#groupId = "";
		this.#state = true;

		this.#socket = io("ws://localhost:3000", {
			auth: {
				token: this.#apiKey,
			}
		}); 

        this.#socket.on("ERROR", (message) => { 
            console.error(chalk.bgBlack.redBright("Spark Error:", message));
        });

        this.#socket.on("OK", (message) => { console.log(message) });
	}

	async subscribe(groupId) {
		this.#groupId = groupId;

		console.log("Attempting to subscribe to room !!");

		this.#socket.emit("JOIN", `${this.#apiKey}_${groupId}`, (data) => {
			console.log(data);
			console.log("Subscribed to room !!");
			this.#state = true;
		});

		this.#socket.emit("message", groupId);
	}

	async on(eventName, callback){
		if(this.#groupId.length < 1){
			console.error(
				chalk.bgRed.cyanBright(
					"Spark Error: Missing `groupId`. Did you forgot to `subsribe` to a group ?"
				)
			);
		};
		console.log(callback.toString())
		this.#socket.on(eventName, callback);
	}

	async ignite(groupId, eventName, message){

	}

}


let spark = new Spark("abc123");
spark.subscribe("emails");

// spark.on("MESSAGE", (data) => {
// 	console.log("CALLBACK EXECUTED: ",data);
// })




// socket.on("aa", { || async { println!("EVENT 'aa' OCCURED !!!!!!!!!! ============") } });

// socket.join("test group");

// socket.on("client", {|message: Data::<String>| async move { 
//     println!("GOT RESPONSE FROM THE CLIENT: {:?}", message.0) 
// }});

// let _ = socket.emit("open", "THIS IS THE SERVER RESPONDING TO YOU !!");














// import { io } from "socket.io-client";

// const socket = io("ws://localhost:3000", { transports: ["websocket"] });

// socket.on("connect", (arg) => {
//     console.log(arg);
// })

// socket.on("open", (payload) => {
//     console.log("RESPONSE FROM THE SERVER ON EVENT 'open': ", payload)
// })

// socket.on("event from handler", (payload) => {
//     // socket
//     console.log("RESPONSE FROM THE SERVER ON EVENT 'handler event': ", payload)
// })


// socket.on("disconnect", () => {
//     console.log(socket.connected); // false
// });